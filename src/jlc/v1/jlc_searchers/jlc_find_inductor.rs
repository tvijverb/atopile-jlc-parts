use polars::prelude::*;

use super::sort_dataframe;
use crate::jlc::v1::jlc_models::*;

// inductors use Henry as unit (H)
pub fn find_inductor(
    components_df: LazyFrame,
    request: JLCPartRequest,
) -> Option<(DataFrame, JLCValue)> {
    // filter components_df on category_id = 12 (inductors)
    let mut components_df = components_df
        .filter(col("category_id").eq(lit(12)))
        .collect()
        .unwrap()
        .lazy();

    // value conversion
    let (henry_multiplier, _henry_string) = match request.value.unit.as_str() {
        "pH" | "picohenry" => (1.0, "pH"),
        "nH" | "nanohenry" => (1e3, "nH"),
        "μH" | "uH" | "microhenry" => (1e6, "μH"),
        "mH" | "millihenry" => (1e9, "mH"),
        "kH" | "kilohenry" => (1e15, "kH"),
        "MH" | "megahenry" => (1e18, "MH"),
        "H" | "henry" => (1e12, "H"),
        _ => return None,
    };

    let henry_value = (request.value.nominal * henry_multiplier).round();
    let henry_max = henry_value + henry_value * request.value.tolerance_pct / 100.0;
    let henry_min = henry_value - henry_value * request.value.tolerance_pct / 100.0;

    let jlc_henry_value = JLCValue {
        unit: "H".to_string(),
        min_val: henry_min * 1e-12,
        max_val: henry_max * 1e-12,
        nominal: henry_value * 1e-12,
        tolerance: request.value.tolerance,
        tolerance_pct: request.value.tolerance_pct,
    };

    // if request.package is not None, filter components_df on package = request.package
    if request.package.is_some() {
        components_df = components_df.filter(col("package").eq(lit(request.package.unwrap())));
    }

    // filter components_df on inductance = henry_value
    let components_df_eq = components_df
        .clone()
        .filter(col("inductance").eq(lit(henry_value)));

    let df_eq = components_df_eq.collect().unwrap();
    if df_eq.height() >= 1 {
        let df_eq_sorted = sort_dataframe(df_eq);
        return Some((df_eq_sorted, jlc_henry_value));
    }

    // filter components_df on inductance > henry_min and inductance < henry_max
    let components_df_range = components_df.filter(
        col("inductance")
            .gt(lit(henry_min))
            .and(col("inductance").lt(lit(henry_max))),
    );

    let df_range = components_df_range.collect().unwrap();
    if df_range.height() >= 1 {
        let df_range_sorted = sort_dataframe(df_range);
        return Some((df_range_sorted, jlc_henry_value));
    }

    None
}
