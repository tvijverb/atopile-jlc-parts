use polars::prelude::*;

use self::resistor::{ResistorRequest, ResistorUnit};

use super::sort_dataframe;
use crate::jlc::v2::models::*;

pub enum Tolerance {
    Up,
    Down,
}

pub fn find_resistor(mut resistors_df: LazyFrame, request: ResistorRequest) -> Option<DataFrame> {
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

    // if request.package is not None, filter resistors_df on package = request.package
    if request.package.is_some() {
        resistors_df = resistors_df.filter(col("package").eq(lit(request.package.unwrap())));
    }

    // filter resistors_df on resistance = ohm_value
    let resistors_df_eq = resistors_df
        .clone()
        .filter(col("resistance").eq(lit(jlc_ohm_value)));

    let df_eq = resistors_df_eq.collect().unwrap();
    if df_eq.height() >= 1 {
        let df_eq_sorted = sort_dataframe(df_eq);
        return Some(df_eq_sorted);
    }

    // filter resistors_df on resistance > ohm_min and resistance < ohm_max
    let resistors_df_range: LazyFrame = resistors_df.filter(
        col("resistance")
            .gt(lit(jlc_ohm_tolerance_down))
            .and(col("resistance").lt(lit(jlc_ohm_tolerance_up))),
    );

    let df_range = resistors_df_range.collect().unwrap();
    if df_range.height() >= 1 {
        let df_range_sorted = sort_dataframe(df_range);
        return Some(df_range_sorted);
    }

    None
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
