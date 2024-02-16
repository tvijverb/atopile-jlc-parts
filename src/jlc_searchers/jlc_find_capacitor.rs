use polars::prelude::*;

use crate::jlc_models::*;
use crate::jlc_searchers::sort_dataframe;


pub fn find_capacitor(components_df: LazyFrame, request: JLCPartRequest) -> Option<(DataFrame, JLCValue)> {
    // filter components_df on category_id = 27 (capacitors)
    let mut components_df = components_df.filter(col("category_id").eq(lit(27))).collect().unwrap().lazy();

    // value conversion
    let (farad_multiplier, farad_string) = match request.value.unit.as_str() {
        "pF" | "picofarad" => (1.0, "pF"),
        "nF" | "nanofarad" => (1e3, "nF"),
        "μF" | "uF" | "microfarad" => (1e6, "μF"),
        "mF" | "millifarad" => (1e9, "mF"),
        "kF" | "kilofarad" => (1e15, "kF"),
        "MF" | "megafarad" => (1e18, "MF"),
        "F" | "farad" => (1e12, "F"),
        _ => return None,
    };

    let farad_value = (request.value.nominal * farad_multiplier).round();
    let farad_max = farad_value + farad_value * request.value.tolerance_pct / 100.0;
    let farad_min = farad_value - farad_value * request.value.tolerance_pct / 100.0;

    let jlc_farad_value = JLCValue {
        unit: "F".to_string(),
        min_val: farad_min * 1e-12,
        max_val: farad_max * 1e-12,
        nominal: farad_value * 1e-12,
        tolerance: request.value.tolerance,
        tolerance_pct: request.value.tolerance_pct,
    };

    // if request.package is not None, filter components_df on package = request.package
    if request.package.is_some() {
        components_df = components_df.filter(col("package").eq(lit(request.package.unwrap())));
    }

    // filter components_df on capacitance = farad_value
    let components_df_eq = components_df.clone().filter(col("capacitance").eq(lit(farad_value)));

    let df_eq = components_df_eq.collect().unwrap();
    if df_eq.height() >= 1 {
        let df_eq_sorted = sort_dataframe(df_eq);
        return Some((df_eq_sorted, jlc_farad_value));
    }
    
    // filter components_df on capacitance > farad_min and capacitance < farad_max
    let components_df_range = components_df.filter(col("capacitance").gt(lit(farad_min)).and(col("capacitance").lt(lit(farad_max))));
    let df_range = components_df_range.collect().unwrap();
    if df_range.height() >= 1 {
        let df_range_sorted = sort_dataframe(df_range);
        return Some((df_range_sorted, jlc_farad_value));
    }

    None
}