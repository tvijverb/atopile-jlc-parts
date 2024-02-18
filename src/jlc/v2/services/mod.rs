pub mod capacitor;
pub mod inductor;
pub mod resistor;
use super::models::Component;

use polars::prelude::*;

pub fn sort_dataframe(df: DataFrame) -> DataFrame {
    df.sort(["basic", "preferred", "stock"], true, false)
        .unwrap()
}

fn to_bool<'a>(v: &AnyValue<'a>) -> bool {
    if let AnyValue::Int64(b) = v {
        *b != 0
    } else {
        tracing::error!("Expected boolean, got {:?}", v);
        false
    }
}

pub fn dataframe_to_component(df: DataFrame) -> Component {
    let df_first = df.get_row(0).unwrap();
    Component {
        lcsc: format!(
            "C{}",
            df_first.0.get(0).unwrap().to_string().replace("\"", "")
        ),
        mpn: df_first.0.get(2).unwrap().to_string().replace("\"", ""),
        package: df_first.0.get(3).unwrap().to_string().replace("\"", ""),
        footprint: format!(
            "R{}",
            df_first.0.get(3).unwrap().to_string().replace("\"", "")
        ),
        joints: df_first.0.get(4).unwrap().try_extract().unwrap(),
        manufacturer_id: df_first.0.get(5).unwrap().try_extract().unwrap(),
        basic: to_bool(df_first.0.get(6).unwrap()),
        description: df_first.0.get(7).unwrap().to_string().replace("\"", ""),
        datasheet: df_first.0.get(8).unwrap().to_string().replace("\"", ""),
        stock: df_first.0.get(9).unwrap().try_extract().unwrap(),
        price: df_first.0.get(10).unwrap().to_string().replace("\"", ""),
        last_update: df_first.0.get(11).unwrap().try_extract().unwrap(),
        flag: df_first.0.get(12).unwrap().try_extract().unwrap(),
        last_on_stock: df_first.0.get(13).unwrap().try_extract().unwrap(),
        preferred: df_first.0.get(14).unwrap().try_extract().unwrap(),
        dielectric: df_first
            .0
            .get(18)
            .unwrap()
            .to_string()
            .replace("\"", "")
            .parse()
            .ok(),
        category_id: df_first.0.get(1).unwrap().try_extract().unwrap(),
    }
}
