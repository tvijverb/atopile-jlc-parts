use sqlx::PgPool;

use crate::jlc::v1::jlc_models::*;
use crate::jlc::v1::jlc_part_finder::Component;

pub async fn find_inductor(
    pool: PgPool,
    request: JLCPartRequest,
) -> Result<(Vec<Component>, JLCValue), sqlx::Error> {
    // get
    let inductor_category_id: (i32,) = sqlx::query_as("SELECT id FROM categories WHERE name = 'Inductors/Coils/Transformers' and subcategory_name = 'Inductors (SMD)'")
    .fetch_one(&pool).await?;

    // value conversion
    let henry_string = request.value.unit.to_string();
    let (henry_value, henry_max, henry_min) = match henry_string.as_str() {
        "pH" | "picohenry" => (
            request.value.nominal,
            request.value.nominal * (1.0 + request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            request.value.nominal * (1.0 - request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
        ),
        "nH" | "nanohenry" => (
            request.value.nominal * 1e3,
            request.value.nominal * 1e3 * (1.0 + request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            request.value.nominal * 1e3 * (1.0 - request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
        ),
        "μH" | "uH" | "microhenry" => (
            request.value.nominal * 1e6,
            request.value.nominal * 1e6 * (1.0 + request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            request.value.nominal * 1e6 * (1.0 - request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
        ),
        "mH" | "millihenry" => (
            request.value.nominal * 1e9,
            request.value.nominal * 1e9 * (1.0 + request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            request.value.nominal * 1e9 * (1.0 - request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
        ),
        "kH" | "kilohenry" => (
            request.value.nominal * 1e15,
            request.value.nominal * 1e15 * (1.0 + request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            request.value.nominal * 1e15 * (1.0 - request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
        ),
        "MH" | "megahenry" => (
            request.value.nominal * 1e18,
            request.value.nominal * 1e18 * (1.0 + request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            request.value.nominal * 1e18 * (1.0 - request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
        ),
        "H" | "henry" => (
            request.value.nominal * 1e12,
            request.value.nominal * 1e12 * (1.0 + request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
            request.value.nominal * 1e12 * (1.0 - request.value.tolerance_pct.unwrap_or(0.0) / 100.0),
        ),
        _ => return Err(sqlx::Error::RowNotFound),
    };

    let jlc_henry_value = JLCValue {
        unit: "H".to_string(),
        min_val: henry_min,
        max_val: henry_max,
        nominal: henry_value,
        tolerance: request.value.tolerance,
        tolerance_pct: Some(request.value.tolerance_pct.unwrap_or(0.0)),
    };

    // if request.package is not None, filter components_df on package = request.package
    if request.package.is_some() {
        let mut matching_parts: Vec<Component> = sqlx::query_as!(
            Component,
            r#"SELECT id as "id!", lcsc as "lcsc!", category_id as "category_id!", mfr as "mfr?", package as "package?", joints as "joints!", manufacturer as "manufacturer!", basic as "basic!", description as "description?", datasheet as "datasheet?", stock as "stock!", price as "price?", last_update as "last_update!", inductance as "inductance?", capacitance, resistance, dielectric as "dielectric?", current, voltage FROM parts WHERE category_id = $1 and inductance between $2 and $3 and package = $4 ORDER BY basic DESC LIMIT 100"#,
            inductor_category_id.0,
            henry_min,
            henry_max,
            request.package.unwrap()
        ).fetch_all(&pool).await?;
        return Ok((matching_parts, jlc_henry_value));
    } else {
        let mut matching_parts: Vec<Component> = sqlx::query_as!(
            Component,
            r#"SELECT id as "id!", lcsc as "lcsc!", category_id as "category_id!", mfr as "mfr?", package as "package?", joints as "joints!", manufacturer as "manufacturer!", basic as "basic!", description as "description?", datasheet as "datasheet?", stock as "stock!", price as "price?", last_update as "last_update!", inductance as "inductance?", capacitance, resistance, dielectric as "dielectric?", current, voltage FROM parts WHERE category_id = $1 and inductance between $2 and $3 ORDER BY basic DESC LIMIT 100"#,
            inductor_category_id.0,
            henry_min,
            henry_max
        ).fetch_all(&pool).await?;
        return Ok((matching_parts, jlc_henry_value));
    }
}