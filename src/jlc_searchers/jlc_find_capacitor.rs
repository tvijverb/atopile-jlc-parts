use polars::prelude::*;

use crate::jlc_models::*;
use crate::jlc_searchers::sort_dataframe;


pub fn find_capacitor(request: JLCPartRequest) -> Option<(DataFrame, JLCValue)> {
    let mut components_df = LazyFrame::scan_parquet("components.parquet", ScanArgsParquet::default()).unwrap();
    
    // filter components_df on category_id = 27 (capacitors)
    components_df = components_df.filter(col("category_id").eq(lit(27)));

    // value conversion
    let farad_string = request.value.unit.to_string();
    let farad_multiplier = match farad_string.as_str() {
        "pF" => 1.0,
        "picofarad" => 1.0,
        "nF" => 1e3,
        "nanofarad" => 1e3,
        "μF" => 1e6,
        "uF" => 1e6,
        "microfarad" => 1e6,
        "mF" => 1e9,
        "millifarad" => 1e9,
        "kF" => 1e15,
        "kilofarad" => 1e15,
        "MF" => 1e18,
        "megafarad" => 1e18,
        "F" => 1e12,
        "farad" => 1e12,
        _ => return None
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

    // filter components_df on capacitance = farad_value
    let mut components_df_eq = components_df.clone().filter(col("capacitance").eq(lit(farad_value)));

    // if request.package is not None, filter components_df on package = request.package
    if request.package.is_some() {
        components_df_eq = components_df_eq.filter(col("package").eq(lit(request.package.unwrap())));
    }

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