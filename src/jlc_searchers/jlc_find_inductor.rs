use polars::prelude::*;

use crate::jlc_models::*;
use crate::jlc_searchers::sort_dataframe;


// inductors use Henry as unit (H)
pub fn find_inductor(request: JLCPartRequest) -> Option<(DataFrame, JLCValue)> {
    let mut components_df = LazyFrame::scan_parquet("components.parquet", ScanArgsParquet::default()).unwrap();
    
    // filter components_df on category_id = 12 (inductors)
    components_df = components_df.filter(col("category_id").eq(lit(12)));

    // value conversion
    let henry_string = request.value.unit.to_string();
    let henry_multiplier = match henry_string.as_str() {
        "pH" => 1.0,
        "picohenry" => 1.0,
        "nH" => 1e3,
        "nanohenry" => 1e3,
        "μH" => 1e6,
        "microhenry" => 1e6,
        "mH" => 1e9,
        "millihenry" => 1e9,
        "kH" => 1e15,
        "kilohenry" => 1e15,
        "MH" => 1e18,
        "megahenry" => 1e18,
        "H" => 1e12,
        "henry" => 1e12,
        _ => return None
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

    // filter components_df on inductance = henry_value
    let mut components_df_eq = components_df.clone().filter(col("inductance").eq(lit(henry_value)));

    // if request.package is not None, filter components_df on package = request.package
    if request.package.is_some() {
        components_df_eq = components_df_eq.filter(col("package").eq(lit(request.package.unwrap())));
    }

    let df_eq = components_df_eq.collect().unwrap();
    if df_eq.height() >= 1 {
        let df_eq_sorted = sort_dataframe(df_eq);
        return Some((df_eq_sorted, jlc_henry_value));
    }

    // filter components_df on inductance > henry_min and inductance < henry_max
    let components_df_range = components_df.filter(col("inductance").gt(lit(henry_min)).and(col("inductance").lt(lit(henry_max))));
    let df_range = components_df_range.collect().unwrap();
    if df_range.height() >= 1 {
        let df_range_sorted = sort_dataframe(df_range);
        return Some((df_range_sorted, jlc_henry_value));
    }

    None
}