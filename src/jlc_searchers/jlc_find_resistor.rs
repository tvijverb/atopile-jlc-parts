use polars::prelude::*;

use crate::jlc_models::*;
use crate::jlc_searchers::sort_dataframe;

pub fn find_resistor(request: JLCPartRequest) -> Option<(DataFrame, JLCValue)> {
    let mut components_df = LazyFrame::scan_parquet("components.parquet", ScanArgsParquet::default()).unwrap();
    
    // filter components_df on category_id = 46
    components_df = components_df.filter(col("category_id").eq(lit(46)));

    // value conversion
    let ohm_string = request.value.unit.to_string();
    // old multipliers
    // let ohm_multiplier = match ohm_string.as_str() {
    //     "Ω" => 1e12,
    //     "ohm" => 1e12,
    //     "kΩ" => 1e15,
    //     "kiloohm" => 1e15,
    //     "MΩ" => 1e18,
    //     "megaohm" => 1e18,
    //     "pΩ" => 1.0,
    //     "picoohm" => 1.0,
    //     "nΩ" => 1e3,
    //     "nanoohm" => 1e3,
    //     "μΩ" => 1e6,
    //     "microohm" => 1e6,
    //     "mΩ" => 1e9,
    //     "milliohm" => 1e9,
    //     _ => return None
    // };
    // new multipliers
    let ohm_multiplier = match ohm_string.as_str() {
        "pΩ" => 1e-12,
        "picoohm" => 1e-12,
        "nΩ" => 1e-9,
        "nanoohm" => 1e-9,
        "μΩ" => 1e-6,
        "uΩ" => 1e-6,
        "microohm" => 1e-6,
        "mΩ" => 1e-3,
        "milliohm" => 1e-3,
        "kΩ" => 1e3,
        "kiloohm" => 1e3,
        "MΩ" => 1e6,
        "megaohm" => 1e6,
        "Ω" => 1e0,
        "ohm" => 1e0,
        _ => return None
    };
    let ohm_value = request.value.nominal * ohm_multiplier;
    let ohm_max = ohm_value + ohm_value * request.value.tolerance_pct / 100.0;
    let ohm_min = ohm_value - ohm_value * request.value.tolerance_pct / 100.0;

    let jlc_ohm_value = JLCValue {
        unit: "Ω".to_string(),
        min_val: ohm_min * 1e-12,
        max_val: ohm_max * 1e-12,
        nominal: ohm_value * 1e-12,
        tolerance: request.value.tolerance,
        tolerance_pct: request.value.tolerance_pct,
    };

    // filter components_df on resistance = ohm_value
    let components_df_eq = components_df.clone().filter(col("resistance").eq(lit(ohm_value)));

    // if request.package is not None, filter components_df on package = request.package
    if request.package.is_some() {
        components_df = components_df.filter(col("package").eq(lit(request.package.unwrap())));
    }

    let df_eq = components_df_eq.collect().unwrap();
    if df_eq.height() >= 1 {
        let df_eq_sorted = sort_dataframe(df_eq);
        return Some((df_eq_sorted, jlc_ohm_value));
    }
    
    // filter components_df on resistance > ohm_min and resistance < ohm_max
    let components_df_range = components_df.filter(col("resistance").gt(lit(ohm_min)).and(col("resistance").lt(lit(ohm_max))));
    let df_range = components_df_range.collect().unwrap();
    if df_range.height() >= 1 {
        let df_range_sorted = sort_dataframe(df_range);
        return Some((df_range_sorted, jlc_ohm_value));
    }

    None
}