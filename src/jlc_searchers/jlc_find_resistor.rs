use polars::prelude::*;

use crate::jlc_models::*;
use crate::jlc_searchers::sort_dataframe;


pub fn find_resistor(components_df: LazyFrame, request: JLCPartRequest) -> Option<(DataFrame, JLCValue)> {
    // filter components_df on category_id = 46
    let mut components_df = components_df.filter(col("category_id").eq(lit(46))).collect().unwrap().lazy();

    // value conversion
    let ohm_string = request.value.unit.to_string();
    let (ohm_value, ohm_max, ohm_min, ohm_multiplier) = match ohm_string.as_str() {
        "pΩ" | "picoohm" => (request.value.nominal * 1e-12, request.value.nominal * 1e-12 * (1.0 + request.value.tolerance_pct / 100.0), request.value.nominal * 1e-12 * (1.0 - request.value.tolerance_pct / 100.0), 1e-12),
        "nΩ" | "nanoohm" => (request.value.nominal * 1e-9, request.value.nominal * 1e-9 * (1.0 + request.value.tolerance_pct / 100.0), request.value.nominal * 1e-9 * (1.0 - request.value.tolerance_pct / 100.0), 1e-9),
        "μΩ" | "uΩ" | "microohm" => (request.value.nominal * 1e-6, request.value.nominal * 1e-6 * (1.0 + request.value.tolerance_pct / 100.0), request.value.nominal * 1e-6 * (1.0 - request.value.tolerance_pct / 100.0), 1e-6),
        "mΩ" | "milliohm" => (request.value.nominal * 1e-3, request.value.nominal * 1e-3 * (1.0 + request.value.tolerance_pct / 100.0), request.value.nominal * 1e-3 * (1.0 - request.value.tolerance_pct / 100.0), 1e-3),
        "kΩ" | "kiloohm" => (request.value.nominal * 1e3, request.value.nominal * 1e3 * (1.0 + request.value.tolerance_pct / 100.0), request.value.nominal * 1e3 * (1.0 - request.value.tolerance_pct / 100.0), 1e3),
        "MΩ" | "megaohm" => (request.value.nominal * 1e6, request.value.nominal * 1e6 * (1.0 + request.value.tolerance_pct / 100.0), request.value.nominal * 1e6 * (1.0 - request.value.tolerance_pct / 100.0), 1e6),
        "Ω" | "ohm" => (request.value.nominal, request.value.nominal * (1.0 + request.value.tolerance_pct / 100.0), request.value.nominal * (1.0 - request.value.tolerance_pct / 100.0), 1.0),
        _ => return None
    };

    let jlc_ohm_value = JLCValue {
        unit: "Ω".to_string(),
        min_val: ohm_min * 1e-12,
        max_val: ohm_max * 1e-12,
        nominal: ohm_value * 1e-12,
        tolerance: request.value.tolerance,
        tolerance_pct: request.value.tolerance_pct,
    };

    // if request.package is not None, filter components_df on package = request.package
    if request.package.is_some() {
        components_df = components_df.filter(col("package").eq(lit(request.package.unwrap())));
    }

    // filter components_df on resistance = ohm_value
    let components_df_eq = components_df.clone().filter(col("resistance").eq(lit(ohm_value)));


    let df_eq = components_df_eq.clone().collect().unwrap();
    if df_eq.height() >= 1 {
        let df_eq_sorted = sort_dataframe(df_eq);
        return Some((df_eq_sorted, jlc_ohm_value));
    }
    
    // filter components_df on resistance > ohm_min and resistance < ohm_max
    let components_df_range: LazyFrame = components_df.filter(col("resistance").gt(lit(ohm_min)).and(col("resistance").lt(lit(ohm_max))));
    
    let df_range = components_df_range.collect().unwrap();
    if df_range.height() >= 1 {
        let df_range_sorted = sort_dataframe(df_range);
        return Some((df_range_sorted, jlc_ohm_value));
    }

    None
}