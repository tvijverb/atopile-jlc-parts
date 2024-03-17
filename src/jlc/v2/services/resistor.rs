use sqlx::PgPool;

use self::resistor::{ResistorRequest, ResistorUnit};
use crate::jlc::v2::models::*;

pub enum Tolerance {
    Up,
    Down,
}

pub async fn find_resistor(
    pool: PgPool,
    request: ResistorRequest,
) -> Result<Vec<Component>, sqlx::Error> {
    // value conversion
    let jlc_ohm_value = get_resistor_value(request.value, request.unit.clone());
    let jlc_ohm_tolerance_up = get_resistor_tolerance(request.clone(), Tolerance::Up);
    let jlc_ohm_tolerance_down = get_resistor_tolerance(request.clone(), Tolerance::Down);
    tracing::info!(
        "Searching for resistor with value: {} ohm, min: {} ohm, max: {} ohm",
        jlc_ohm_value,
        jlc_ohm_tolerance_down,
        jlc_ohm_tolerance_up
    );

    let resistor_category_id: (i32,) = sqlx::query_as("SELECT id FROM categories WHERE name = 'Resistors' and subcategory_name = 'Chip Resistor - Surface Mount'")
    .fetch_one(&pool).await?;

    if request.package.is_some() {
        let matching_parts: Vec<Component> = sqlx::query_as!(
            Component,
            r#"SELECT lcsc as "lcsc!", category_id as "category_id!", mfr as "mfr?", package as "package?", joints as "joints!", manufacturer as "manufacturer!", basic as "basic!", description as "description?", datasheet as "datasheet?", stock as "stock!", price as "price?", dielectric as "dielectric?" FROM parts WHERE category_id = $1 and resistance between $2 and $3 and package = $4 ORDER BY basic DESC LIMIT 100"#,
            resistor_category_id.0,
            jlc_ohm_tolerance_down,
            jlc_ohm_tolerance_up,
            request.package.unwrap()
        ).fetch_all(&pool).await?;
        return Ok(matching_parts);
    } else {
        let matching_parts: Vec<Component> = sqlx::query_as!(
            Component,
            r#"SELECT lcsc as "lcsc!", category_id as "category_id!", mfr as "mfr?", package as "package?", joints as "joints!", manufacturer as "manufacturer!", basic as "basic!", description as "description?", datasheet as "datasheet?", stock as "stock!", price as "price?", dielectric as "dielectric?" FROM parts WHERE category_id = $1 and resistance between $2 and $3 ORDER BY basic DESC LIMIT 100"#,
            resistor_category_id.0,
            jlc_ohm_tolerance_down,
            jlc_ohm_tolerance_up
        ).fetch_all(&pool).await?;
        return Ok(matching_parts);
    }
}

pub fn get_resistor_tolerance(request: ResistorRequest, tolerance: Tolerance) -> f64 {
    let nominal_value = get_resistor_value(request.value, request.unit);
    // check if absolute_tolerance is set or tolerance_percentage is set
    if request.absolute_tolerance.is_some() {
        let absolute_tolerance = request.absolute_tolerance.unwrap();
        let absolute_tolerance_unit = request.absolute_tolerance_unit.unwrap();
        let tolerance_value = get_resistor_value(absolute_tolerance, absolute_tolerance_unit);
        match tolerance {
            Tolerance::Up => {
                return nominal_value + tolerance_value;
            }
            Tolerance::Down => {
                return nominal_value - tolerance_value;
            }
        }
    } else {
        let tolerance_pct = request.tolerance_percentage.unwrap();
        match tolerance {
            Tolerance::Up => {
                return nominal_value + (nominal_value * (tolerance_pct / 100.0));
            }
            Tolerance::Down => {
                return nominal_value - (nominal_value * (tolerance_pct / 100.0));
            }
        }
    }
}

pub fn get_resistor_value(request_value: f64, request_unit: ResistorUnit) -> f64 {
    match request_unit {
        ResistorUnit::PicoOhm => return request_value * 1e-12,
        ResistorUnit::NanoOhm => return request_value * 1e-9,
        ResistorUnit::MicroOhm => return request_value * 1e-6,
        ResistorUnit::MilliOhm => return request_value * 1e-3,
        ResistorUnit::KiloOhm => return request_value * 1e3,
        ResistorUnit::MegaOhm => return request_value * 1e6,
        ResistorUnit::Ohm => return request_value,
    };
}
