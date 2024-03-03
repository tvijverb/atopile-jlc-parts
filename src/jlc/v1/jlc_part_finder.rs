use polars::prelude::*;
use sqlx::PgPool;
use uuid::Uuid;

use crate::jlc::v1::jlc_models::*;
use crate::jlc::v1::jlc_searchers::jlc_find_capacitor::find_capacitor;
use crate::jlc::v1::jlc_searchers::jlc_find_inductor::find_inductor;
use crate::jlc::v1::jlc_searchers::jlc_find_resistor::find_resistor;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Component {
    pub id: i64,
    pub lcsc: String,
    pub category_id: i64,
    pub mfr: Option<String>,
    pub package: Option<String>,
    pub joints: i64,
    pub manufacturer: String,
    pub basic: bool,
    pub description: Option<String>,
    pub datasheet: Option<String>,
    pub stock: i64,
    pub price: Option<f64>,
    pub last_update: sqlx::types::time::PrimitiveDateTime,
    pub resistance: Option<f64>,
    pub inductance: Option<f64>,
    pub capacitance: Option<f64>,
    pub dielectric: Option<String>,
    pub current: Option<f64>,
    pub voltage: Option<f64>,
}

pub async fn find_part(pool: PgPool, request: JLCPartRequest) -> Result<JLCPartResponse, String> {
    tracing::info!("Searching JLC part: {:?}", request);
    if request.type_field == "resistor".to_string() {
        let option_resistor_vec = find_resistor(pool, request.clone()).await;
        if option_resistor_vec.is_err() {
            return Err("No resistor found".to_string());
        } else {
            let (component_vec, jlc_value) = option_resistor_vec.unwrap();
            if component_vec.len() == 0 {
                return Err("No resistor found".to_string());
            }
            return Ok(component_vec_to_jlcpb_part_response(request, component_vec, jlc_value));
        }

    } else if request.type_field == "capacitor".to_string() {
        let option_capacitor_vec = find_capacitor(pool, request.clone()).await;
        if option_capacitor_vec.is_err() {
            return Err("No capacitor found".to_string());
        } else {
            let (component_vec, jlc_value) = option_capacitor_vec.unwrap();
            if component_vec.len() == 0 {
                return Err("No capacitor found".to_string());
            }
            return Ok(component_vec_to_jlcpb_part_response(request, component_vec, jlc_value));
        }
    } else if request.type_field == "inductor".to_string() {
        let option_inductor_vec = find_inductor(pool, request.clone()).await;
        if option_inductor_vec.is_err() {
            return Err("No inductor found".to_string());
        } else {
            let (component_vec, jlc_value) = option_inductor_vec.unwrap();
            if component_vec.len() == 0 {
                return Err("No inductor found".to_string());
            }
            return Ok(component_vec_to_jlcpb_part_response(request, component_vec, jlc_value));
        }
    }
    else {
        Err("Unsupported part type".to_string())
    }
}

pub fn component_vec_to_jlcpb_part_response(
    request: JLCPartRequest,
    components: Vec<Component>,
    jlc_value: JLCValue,
) -> JLCPartResponse {
    // return first element of components vector
    let component = components.get(0).unwrap();

    // kicad_footprint is R + package for resistors and C + package for capacitors
    let kicad_footprint = match request.type_field.as_str() {
        "resistor" => format!("R{}", &component.package.clone().unwrap_or("".to_string())),
        "capacitor" => format!("C{}", &component.package.clone().unwrap_or("".to_string())),
        "inductor" => "L".to_string(),
        _ => "".to_string(),
    };
    let lcsc_id = component.lcsc.clone();
    let best_component = BestComponent {
        dielectric: component.dielectric.clone(),
        basic_part: component.basic,
        description: component.description.clone().unwrap_or("".to_string()),
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
        mpn: component.mfr.clone().unwrap_or("".to_string()),
        datasheet: component.datasheet.clone().unwrap_or("".to_string()),
        category: "Resistors".to_string(), // not really needed right now
        lcsc_id: lcsc_id,
        package: component.package.clone().unwrap_or("".to_string()),
        footprint_data: FootprintData {
            kicad: "standard-library".to_string(),
        },
    };
    JLCPartResponse {
        best_component: best_component,
    }

}