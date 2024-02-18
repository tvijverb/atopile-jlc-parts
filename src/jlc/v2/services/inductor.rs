use polars::prelude::*;

use self::inductor::{InductorRequest, InductorUnit};

use super::sort_dataframe;
use crate::jlc::v2::models::*;

pub enum Tolerance {
    Up,
    Down,
}

pub fn find_inductor(mut inductors_df: LazyFrame, request: InductorRequest) -> Option<DataFrame> {
    let jlc_henry_value = get_inductor_value(request.value, request.unit.clone());
    let jlc_henry_tolerance_up = get_inductor_tolerance(request.clone(), Tolerance::Up);
    let jlc_henry_tolerance_down = get_inductor_tolerance(request.clone(), Tolerance::Down);
    tracing::info!(
        "Searching for inductor with value: {} henry, min: {} henry, max: {} henry",
        jlc_henry_value * 1e-12,
        jlc_henry_tolerance_down * 1e-12,
        jlc_henry_tolerance_up * 1e-12
    );

    if request.package.is_some() {
        inductors_df = inductors_df.filter(col("package").eq(lit(request.package.unwrap())));
    }

    let inductors_df_eq = inductors_df
        .clone()
        .filter(col("inductance").eq(lit(jlc_henry_value)));

    let df_eq = inductors_df_eq.clone().collect().unwrap();
    if df_eq.height() >= 1 {
        let df_eq_sorted = sort_dataframe(df_eq);
        return Some(df_eq_sorted);
    }

    let inductors_df_range: LazyFrame = inductors_df.filter(
        col("inductance")
            .gt(lit(jlc_henry_tolerance_down))
            .and(col("inductance").lt(lit(jlc_henry_tolerance_up))),
    );

    let df_range = inductors_df_range.collect().unwrap();
    if df_range.height() >= 1 {
        let df_range_sorted = sort_dataframe(df_range);
        return Some(df_range_sorted);
    }

    None
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
