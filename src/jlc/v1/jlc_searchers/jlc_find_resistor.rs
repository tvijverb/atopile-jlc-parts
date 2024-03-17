use sqlx::PgPool;

use crate::jlc::v1::jlc_models::*;
use crate::jlc::v1::jlc_part_finder::Component;

pub async fn find_resistor(
    pool: PgPool,
    request: JLCPartRequest,
) -> Result<(Vec<Component>, JLCValue), sqlx::Error> {
    // get
    let resistor_category_id: (i32,) = sqlx::query_as("SELECT id FROM categories WHERE name = 'Resistors' and subcategory_name = 'Chip Resistor - Surface Mount'")
    .fetch_one(&pool).await?;

    // value conversion
    let ohm_string = request.value.unit.to_string();
    let (ohm_value, ohm_max, ohm_min, _ohm_multiplier) = match ohm_string.as_str() {
        "pΩ" | "picoohm" => (
            request.value.nominal * 1e-12,
            request.value.nominal * 1e-12 * (1.0 + request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            request.value.nominal * 1e-12 * (1.0 - request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            1e-12,
        ),
        "nΩ" | "nanoohm" => (
            request.value.nominal * 1e-9,
            request.value.nominal * 1e-9 * (1.0 + request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            request.value.nominal * 1e-9 * (1.0 - request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            1e-9,
        ),
        "μΩ" | "uΩ" | "microohm" => (
            request.value.nominal * 1e-6,
            request.value.nominal * 1e-6 * (1.0 + request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            request.value.nominal * 1e-6 * (1.0 - request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            1e-6,
        ),
        "mΩ" | "milliohm" => (
            request.value.nominal * 1e-3,
            request.value.nominal * 1e-3 * (1.0 + request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            request.value.nominal * 1e-3 * (1.0 - request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            1e-3,
        ),
        "kΩ" | "kiloohm" => (
            request.value.nominal * 1e3,
            request.value.nominal * 1e3 * (1.0 + request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            request.value.nominal * 1e3 * (1.0 - request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            1e3,
        ),
        "MΩ" | "megaohm" => (
            request.value.nominal * 1e6,
            request.value.nominal * 1e6 * (1.0 + request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            request.value.nominal * 1e6 * (1.0 - request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            1e6,
        ),
        "Ω" | "ohm" => (
            request.value.nominal,
            request.value.nominal * (1.0 + request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            request.value.nominal * (1.0 - request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            1.0,
        ),
        _ => return Err(sqlx::Error::RowNotFound),
    };

    let jlc_ohm_value = JLCValue {
        unit: "Ω".to_string(),
        min_val: ohm_min * 1e-12,
        max_val: ohm_max * 1e-12,
        nominal: ohm_value * 1e-12,
        tolerance: request.value.tolerance,
        tolerance_pct: Some(request.value.tolerance_pct.unwrap_or(0.0)),
    };

    // if request.package is not None, filter components_df on package = request.package
    if request.package.is_some() {
        let matching_parts: Vec<Component> = sqlx::query_as!(
            Component,
            r#"SELECT id as "id!", lcsc as "lcsc!", category_id as "category_id!", mfr as "mfr?", package as "package?", joints as "joints!", manufacturer as "manufacturer!", basic as "basic!", description as "description?", datasheet as "datasheet?", stock as "stock!", price as "price?", last_update as "last_update!", resistance as "resistance?", inductance, capacitance, dielectric as "dielectric?", current, voltage FROM parts WHERE category_id = $1 and resistance between $2 and $3 and package = $4 ORDER BY basic DESC LIMIT 100"#,
            resistor_category_id.0,
            ohm_min,
            ohm_max,
            request.package.unwrap()
        ).fetch_all(&pool).await?;
        return Ok((matching_parts, jlc_ohm_value));
    } else {
        let matching_parts: Vec<Component> = sqlx::query_as!(
            Component,
            r#"SELECT id as "id!", lcsc as "lcsc!", category_id as "category_id!", mfr as "mfr?", package as "package?", joints as "joints!", manufacturer as "manufacturer!", basic as "basic!", description as "description?", datasheet as "datasheet?", stock as "stock!", price as "price?", last_update as "last_update!", resistance as "resistance?", inductance, capacitance, dielectric as "dielectric?", current, voltage FROM parts WHERE category_id = $1 and resistance between $2 and $3 ORDER BY basic DESC LIMIT 100"#,
            resistor_category_id.0,
            ohm_min,
            ohm_max
        ).fetch_all(&pool).await?;
        return Ok((matching_parts, jlc_ohm_value));
    }
}
