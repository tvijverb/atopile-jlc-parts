use sqlx::PgPool;

use self::inductor::{InductorRequest, InductorUnit};
use crate::jlc::v2::models::*;

pub enum Tolerance {
    Up,
    Down,
}

pub async fn find_inductor(
    pool: PgPool,
    request: InductorRequest,
) -> Result<Vec<Component>, sqlx::Error> {
    let jlc_henry_value = get_inductor_value(request.value, request.unit.clone());
    let jlc_henry_tolerance_up = get_inductor_tolerance(request.clone(), Tolerance::Up);
    let jlc_henry_tolerance_down = get_inductor_tolerance(request.clone(), Tolerance::Down);
    tracing::info!(
        "Searching for inductor with value: {} henry, min: {} henry, max: {} henry",
        jlc_henry_value * 1e-12,
        jlc_henry_tolerance_down * 1e-12,
        jlc_henry_tolerance_up * 1e-12
    );

    let inductor_category_id: (i32,) = sqlx::query_as("SELECT id FROM categories WHERE name = 'Inductors/Coils/Transformers' and subcategory_name = 'Inductors (SMD)'")
    .fetch_one(&pool).await?;

    if request.package.is_some() {
        let matching_parts: Vec<Component> = sqlx::query_as!(
            Component,
            r#"SELECT lcsc as "lcsc!", category_id as "category_id!", mfr as "mfr?", package as "package?", joints as "joints!", manufacturer as "manufacturer!", basic as "basic!", description as "description?", datasheet as "datasheet?", stock as "stock!", price as "price?", dielectric as "dielectric?" FROM parts WHERE category_id = $1 and inductance between $2 and $3 and package = $4 ORDER BY basic DESC LIMIT 100"#,
            inductor_category_id.0,
            jlc_henry_tolerance_down,
            jlc_henry_tolerance_up,
            request.package.unwrap()
        ).fetch_all(&pool).await?;
        return Ok(matching_parts);
    } else {
        let matching_parts: Vec<Component> = sqlx::query_as!(
            Component,
            r#"SELECT lcsc as "lcsc!", category_id as "category_id!", mfr as "mfr?", package as "package?", joints as "joints!", manufacturer as "manufacturer!", basic as "basic!", description as "description?", datasheet as "datasheet?", stock as "stock!", price as "price?", dielectric as "dielectric?" FROM parts WHERE category_id = $1 and inductance between $2 and $3 ORDER BY basic DESC LIMIT 100"#,
            inductor_category_id.0,
            jlc_henry_tolerance_down,
            jlc_henry_tolerance_up
        ).fetch_all(&pool).await?;
        return Ok(matching_parts);
    }
}

pub fn get_inductor_tolerance(request: InductorRequest, tolerance: Tolerance) -> f64 {
    let nominal_value = get_inductor_value(request.value, request.unit);
    if request.absolute_tolerance.is_some() {
        let absolute_tolerance = request.absolute_tolerance.unwrap();
        let absolute_tolerance_unit = request.absolute_tolerance_unit.unwrap();
        let tolerance_value = get_inductor_value(absolute_tolerance, absolute_tolerance_unit);
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

pub fn get_inductor_value(request_value: f64, request_unit: InductorUnit) -> f64 {
    match request_unit {
        InductorUnit::PicoHenry => return request_value * 1.0,
        InductorUnit::NanoHenry => return request_value * 1e3,
        InductorUnit::MicroHenry => return request_value * 1e6,
        InductorUnit::MilliHenry => return request_value * 1e9,
        InductorUnit::Henry => return request_value * 1e12,
        InductorUnit::KiloHenry => return request_value * 1e15,
        InductorUnit::MegaHenry => return request_value * 1e18,
    };
}
