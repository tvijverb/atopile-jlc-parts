use sqlx::PgPool;

use crate::jlc::v1::jlc_models::*;
use crate::jlc::v1::jlc_part_finder::Component;

pub async fn find_capacitor(
    pool: PgPool,
    request: JLCPartRequest,
) -> Result<(Vec<Component>, JLCValue), sqlx::Error> {
    // get
    let capacitor_category_id: (i32,) = sqlx::query_as("SELECT id FROM categories WHERE name = 'Capacitors' and subcategory_name = 'Multilayer Ceramic Capacitors MLCC - SMD/SMT'")
    .fetch_one(&pool).await?;

    // value conversion
    let farad_string = request.value.unit.to_string();
    let (farad_value, farad_max, farad_min) = match farad_string.as_str() {
        "pF" | "picofarad" => (
            request.value.nominal,
            request.value.nominal * (1.0 + request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            request.value.nominal * (1.0 - request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
        ),
        "nF" | "nanofarad" => (
            request.value.nominal * 1e3,
            request.value.nominal * 1e3 * (1.0 + request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            request.value.nominal * 1e3 * (1.0 - request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
        ),
        "μF" | "uF" | "microfarad" => (
            request.value.nominal * 1e6,
            request.value.nominal * 1e6 * (1.0 + request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            request.value.nominal * 1e6 * (1.0 - request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
        ),
        "mF" | "millifarad" => (
            request.value.nominal * 1e9,
            request.value.nominal * 1e9 * (1.0 + request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            request.value.nominal * 1e9 * (1.0 - request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
        ),
        "F" | "farad" => (
            request.value.nominal * 1e12,
            request.value.nominal * 1e12 * (1.0 + request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            request.value.nominal * 1e12 * (1.0 - request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
        ),
        _ => return Err(sqlx::Error::RowNotFound),
    };

    let jlc_farad_value = JLCValue {
        unit: "F".to_string(),
        min_val: farad_min,
        max_val: farad_max,
        nominal: farad_value,
        tolerance: request.value.tolerance,
        tolerance_pct: Some(request.value.tolerance_pct.unwrap_or(0.0)),
    };

    // if request.package is not None, filter components_df on package = request.package
    if request.package.is_some() {
        let mut matching_parts: Vec<Component> = sqlx::query_as!(
            Component,
            r#"SELECT id as "id!", lcsc as "lcsc!", category_id as "category_id!", mfr as "mfr?", package as "package?", joints as "joints!", manufacturer as "manufacturer!", basic as "basic!", description as "description?", datasheet as "datasheet?", stock as "stock!", price as "price?", last_update as "last_update!", capacitance as "capacitance?", inductance, resistance, dielectric as "dielectric?", current, voltage FROM parts WHERE category_id = $1 and capacitance between $2 and $3 and package = $4 ORDER BY basic DESC LIMIT 100"#,
            capacitor_category_id.0,
            farad_min,
            farad_max,
            request.package.unwrap()
        ).fetch_all(&pool).await?;
        return Ok((matching_parts, jlc_farad_value));
    } else {
        let mut matching_parts: Vec<Component> = sqlx::query_as!(
            Component,
            r#"SELECT id as "id!", lcsc as "lcsc!", category_id as "category_id!", mfr as "mfr?", package as "package?", joints as "joints!", manufacturer as "manufacturer!", basic as "basic!", description as "description?", datasheet as "datasheet?", stock as "stock!", price as "price?", last_update as "last_update!", capacitance as "capacitance?", inductance, resistance, dielectric as "dielectric?", current, voltage FROM parts WHERE category_id = $1 and capacitance between $2 and $3 ORDER BY basic DESC LIMIT 100"#,
            capacitor_category_id.0,
            farad_min,
            farad_max
        ).fetch_all(&pool).await?;
        return Ok((matching_parts, jlc_farad_value));
    }
}