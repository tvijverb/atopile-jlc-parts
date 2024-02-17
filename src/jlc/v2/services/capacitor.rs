use polars::prelude::*;

use self::capacitor::{CapacitorRequest, CapacitorUnit};

use super::sort_dataframe;
use crate::jlc::v2::models::*;

pub enum Tolerance {
    Up,
    Down,
}

pub fn find_capacitor(mut capacitors_df: LazyFrame, request: CapacitorRequest) -> Option<DataFrame> {
    // get the nominal value and tolerance values
    let jlc_farad_value = get_capacitor_value(request.value, request.unit.clone());
    let jlc_farad_tolerance_up = get_capacitor_tolerance(request.clone(), Tolerance::Up);
    let jlc_farad_tolerance_down = get_capacitor_tolerance(request.clone(), Tolerance::Down);
    tracing::info!(
        "Searching for capacitor with value: {} farad, min: {} farad, max: {} farad",
        jlc_farad_value,
        jlc_farad_tolerance_down,
        jlc_farad_tolerance_up
    );

    // if request.package is not None, filter capacitors_df on package = request.package
    if request.package.is_some() {
        capacitors_df = capacitors_df.filter(col("package").eq(lit(request.package.unwrap())));
    }

    // filter capacitors_df on capacitance = farad_value
    let capacitors_df_eq = capacitors_df
        .clone()
        .filter(col("capacitance").eq(lit(jlc_farad_value)));

    let df_eq = capacitors_df_eq.clone().collect().unwrap();
    if df_eq.height() >= 1 {
        let df_eq_sorted = sort_dataframe(df_eq);
        return Some(df_eq_sorted);
    }

    // filter capacitors_df on capacitance > farad_min and capacitance < farad_max
    let capacitors_df_range: LazyFrame = capacitors_df.filter(
        col("capacitance")
            .gt(lit(jlc_farad_tolerance_down))
            .and(col("capacitance").lt(lit(jlc_farad_tolerance_up))),
    );

    let df_range = capacitors_df_range.collect().unwrap();
    if df_range.height() >= 1 {
        let df_range_sorted = sort_dataframe(df_range);
        return Some(df_range_sorted);
    }

    None
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