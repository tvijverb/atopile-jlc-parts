use polars::prelude::*;
use uuid::Uuid;

use crate::jlc_models::*;
use crate::jlc_searchers::jlc_find_resistor::find_resistor;
use crate::jlc_searchers::jlc_find_capacitor::find_capacitor;
use crate::jlc_searchers::jlc_find_inductor::find_inductor;

#[derive(Debug, Clone)]
struct Component {
    lcsc: String,
    category_id: i64,
    mfr: String,
    package: String,
    joints: i64,
    manufacturer_id: i64,
    basic: i64,
    description: String,
    datasheet: String,
    stock: i64,
    price: String,
    last_update: i64,
    flag: i64,
    last_on_stock: i64,
    preferred: i64,
    resistance: Option<i64>,
    inductance: Option<i64>,
    capacitance: Option<i64>,
    dielectric: Option<String>
}


pub fn find_part(polars_df: LazyFrame, request: JLCPartRequest) -> Result<JLCPartResponse, String> {
    tracing::info!("Searching JLC part: {:?}", request);
    if request.type_field == "resistor".to_string() {
        let option_resistor_df = find_resistor(polars_df, request.clone());
        if option_resistor_df.is_none() {
            return Err("No resistor found".to_string());
        } else {
            let (df, jlc_value) = option_resistor_df.unwrap();
            return Ok(df_to_jlcpb_part_response(request, df, jlc_value));
        }
    } else if request.type_field == "capacitor".to_string() {
        let option_capacitor_df = find_capacitor(polars_df, request.clone());
        if option_capacitor_df.is_none() {
            return Err("No capacitor found".to_string());
        } else {
            let (df, jlc_value) = option_capacitor_df.unwrap();
            return Ok(df_to_jlcpb_part_response(request, df, jlc_value));
        }
    } else if request.type_field == "inductor".to_string() {
        let option_inductor_df = find_inductor(polars_df, request.clone());
        if option_inductor_df.is_none() {
            return Err("No inductor found".to_string());
        } else {
            let (df, jlc_value) = option_inductor_df.unwrap();
            return Ok(df_to_jlcpb_part_response(request, df, jlc_value));
        }
    } else {
        Err("Unsupported part type".to_string())
    }
}

pub fn df_to_jlcpb_part_response(request: JLCPartRequest, df: DataFrame, jlc_value: JLCValue) -> JLCPartResponse {
    // select first value of every column
    let df_first = df.get_row(0).unwrap();
    let component = Component {
        lcsc: df_first.0.get(0).unwrap().to_string(),
        category_id: df_first.0.get(1).unwrap().try_extract().unwrap(),
        mfr: df_first.0.get(2).unwrap().to_string(),
        package: df_first.0.get(3).unwrap().to_string(),
        joints: df_first.0.get(4).unwrap().try_extract().unwrap(),
        manufacturer_id: df_first.0.get(5).unwrap().try_extract().unwrap(),
        basic: df_first.0.get(6).unwrap().try_extract().unwrap(),
        description: df_first.0.get(7).unwrap().to_string(),
        datasheet: df_first.0.get(8).unwrap().to_string(),
        stock: df_first.0.get(9).unwrap().try_extract().unwrap(),
        price: df_first.0.get(10).unwrap().to_string(),
        last_update: df_first.0.get(11).unwrap().try_extract().unwrap(),
        flag: df_first.0.get(12).unwrap().try_extract().unwrap(),
        last_on_stock: df_first.0.get(13).unwrap().try_extract().unwrap(),
        preferred: df_first.0.get(14).unwrap().try_extract().unwrap(),
        resistance: df_first.0.get(15).unwrap().try_extract().ok(),
        inductance: df_first.0.get(16).unwrap().try_extract().ok(),
        capacitance: df_first.0.get(17).unwrap().try_extract().ok(),
        dielectric: df_first.0.get(18).unwrap().to_string().replace("\"", "").parse().ok(),
    };
    // kicad_footprint is R + package for resistors and C + package for capacitors
    let kicad_footprint = match request.type_field.as_str() {
        "resistor" => format!("R{}", &component.package),
        "capacitor" => format!("C{}", &component.package),
        "inductor" => "L".to_string(),
        _ => "".to_string()
    };
    let lcsc_id = format!("C{}", &component.lcsc.replace("\"", ""));
    let best_component = BestComponent {
        dielectric: component.dielectric,
        basic_part: component.basic == 1,
        description: component.description.replace("\"", ""),
        type_field: request.type_field,
        uuid: Uuid::new_v4().to_string(),
        value: jlc_value,
        stock: component.stock,
        code: 200.to_string(),
        voltage: Voltage {
            min_val: 0,
            max_val: 0,
            unit: "V".to_string(),
        },
        price_usd: None,
        area: None,
        footprint: Footprint {
            kicad: kicad_footprint.replace("\"", ""),
        },
        mpn: component.mfr.replace("\"", ""),
        datasheet: component.datasheet.replace("\"", ""),
        category: "".to_string(), // not really needed right now
        lcsc_id: lcsc_id,
        package: component.package.replace("\"", ""),
        footprint_data: FootprintData {
            kicad: "standard-library".to_string(),
        },
    };
    JLCPartResponse {
        best_component: best_component,
    }
}