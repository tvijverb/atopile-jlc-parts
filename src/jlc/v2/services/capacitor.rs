use sqlx::PgPool;

use self::capacitor::{CapacitorRequest, CapacitorUnit};
use crate::jlc::v2::models::*;

pub enum Tolerance {
    Up,
    Down,
}

pub async fn find_capacitor(
    pool: PgPool,
    request: CapacitorRequest,
) -> Result<Vec<Component>, sqlx::Error> {
    // get the nominal value and tolerance values
    let jlc_farad_value = get_capacitor_value(request.value, request.unit.clone());
    let jlc_farad_tolerance_up = get_capacitor_tolerance(request.clone(), Tolerance::Up);
    let jlc_farad_tolerance_down = get_capacitor_tolerance(request.clone(), Tolerance::Down);
    tracing::info!(
        "Searching for capacitor with value: {} farad, min: {} farad, max: {} farad",
        jlc_farad_value * 1e-12,
        jlc_farad_tolerance_down * 1e-12,
        jlc_farad_tolerance_up * 1e-12
    );

    let capacitor_category_id: (i32,) = sqlx::query_as("SELECT id FROM categories WHERE name = 'Capacitors' and subcategory_name = 'Multilayer Ceramic Capacitors MLCC - SMD/SMT'")
    .fetch_one(&pool).await?;

    // if request.package is not None, filter components_df on package = request.package
    if request.package.is_some() {
        let matching_parts: Vec<Component> = sqlx::query_as!(
            Component,
            r#"SELECT lcsc as "lcsc!", category_id as "category_id!", mfr as "mfr?", package as "package?", joints as "joints!", manufacturer as "manufacturer!", basic as "basic!", description as "description?", datasheet as "datasheet?", stock as "stock!", price as "price?", dielectric as "dielectric?" FROM parts WHERE category_id = $1 and capacitance between $2 and $3 and package = $4 ORDER BY basic DESC LIMIT 100"#,
            capacitor_category_id.0,
            jlc_farad_tolerance_down,
            jlc_farad_tolerance_up,
            request.package.unwrap()
        ).fetch_all(&pool).await?;
        return Ok(matching_parts);
    } else {
        let matching_parts: Vec<Component> = sqlx::query_as!(
            Component,
            r#"SELECT lcsc as "lcsc!", category_id as "category_id!", mfr as "mfr?", package as "package?", joints as "joints!", manufacturer as "manufacturer!", basic as "basic!", description as "description?", datasheet as "datasheet?", stock as "stock!", price as "price?", dielectric as "dielectric?" FROM parts WHERE category_id = $1 and capacitance between $2 and $3 ORDER BY basic DESC LIMIT 100"#,
            capacitor_category_id.0,
            jlc_farad_tolerance_down,
            jlc_farad_tolerance_up
        ).fetch_all(&pool).await?;
        return Ok(matching_parts);
    }
}

pub fn get_capacitor_tolerance(request: CapacitorRequest, tolerance: Tolerance) -> f64 {
    let nominal_value = get_capacitor_value(request.value, request.unit);
    // check if absolute_tolerance is set or tolerance_percentage is set
    if request.absolute_tolerance.is_some() {
        let absolute_tolerance = request.absolute_tolerance.unwrap();
        let absolute_tolerance_unit = request.absolute_tolerance_unit.unwrap();
        let tolerance_value = get_capacitor_value(absolute_tolerance, absolute_tolerance_unit);
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

pub fn get_capacitor_value(request_value: f64, request_unit: CapacitorUnit) -> f64 {
    match request_unit {
        CapacitorUnit::PicoFarad => return request_value * 1.0,
        CapacitorUnit::NanoFarad => return request_value * 1e3,
        CapacitorUnit::MicroFarad => return request_value * 1e6,
        CapacitorUnit::MilliFarad => return request_value * 1e9,
        CapacitorUnit::Farad => return request_value * 1e12,
        CapacitorUnit::KiloFarad => return request_value * 1e15,
        CapacitorUnit::MegaFarad => return request_value * 1e18,
    };
}
