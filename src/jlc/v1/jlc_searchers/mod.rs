pub mod jlc_find_capacitor;
pub mod jlc_find_inductor;
pub mod jlc_find_resistor;

use polars::prelude::*;

pub fn sort_dataframe(df: DataFrame) -> DataFrame {
    df.sort(["basic", "preferred", "stock"], true, false)
        .unwrap()
}
