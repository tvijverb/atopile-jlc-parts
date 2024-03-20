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

impl Ord for Component {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.basic && !other.basic {
            std::cmp::Ordering::Less
        } else if !self.basic && other.basic {
            std::cmp::Ordering::Greater
        } else {
            other.stock.cmp(&self.stock)
        }
    }
}

impl PartialOrd for Component {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Component {}

impl PartialEq for Component {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
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
            return Ok(component_vec_to_jlcpb_part_response(
                request,
                component_vec,
                jlc_value,
            ));
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
            return Ok(component_vec_to_jlcpb_part_response(
                request,
                component_vec,
                jlc_value,
            ));
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
            return Ok(component_vec_to_jlcpb_part_response(
                request,
                component_vec,
                jlc_value,
            ));
        }
    } else {
        Err("Unsupported part type".to_string())
    }
}

pub fn component_vec_to_jlcpb_part_response(
    request: JLCPartRequest,
    mut components: Vec<Component>,
    jlc_value: JLCValue,
) -> JLCPartResponse {
    // return first element of components vector
    components.sort();
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
        price_usd: component.price,
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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;

    use crate::Args;

    #[tokio::test]
    async fn test_resistor() {
        dotenv().ok();
        let args = Args::parse();
        let pool = PgPool::connect(args.database_url.as_str()).await.unwrap();
        let request = JLCPartRequest {
            type_field: "resistor".to_string(),
            designator_prefix: "R".to_string(),
            mpn: "generic_resistor".to_string(),
            value: JLCValue {
                unit: "megaohm".to_string(),
                min_val: 4.0,
                max_val: 5.0,
                nominal: 4.5,
            },
            package: Some("0603".to_string()),
        };
        let resistor_result = find_resistor(pool, request.clone()).await;
        assert!(resistor_result.is_ok());
        let (component_vec, _jlc_value) = resistor_result.unwrap();
        assert!(component_vec.len() > 0);
        assert!(component_vec[0].package == Some("0603".to_string()));
    }

    #[tokio::test]
    async fn test_capacitor() {
        dotenv().ok();
        let args = Args::parse();
        let pool = PgPool::connect(args.database_url.as_str()).await.unwrap();
        let request = JLCPartRequest {
            type_field: "capacitor".to_string(),
            designator_prefix: "C".to_string(),
            mpn: "generic_capacitor".to_string(),
            value: JLCValue {
                unit: "microfarad".to_string(),
                min_val: 4.0,
                max_val: 5.0,
                nominal: 4.5,
            },
            package: Some("0603".to_string()),
        };
        let capacitor_result = find_capacitor(pool, request.clone()).await;
        assert!(capacitor_result.is_ok());
        let (component_vec, _jlc_value) = capacitor_result.unwrap();
        assert!(component_vec.len() > 0);
        assert!(component_vec[0].package == Some("0603".to_string()));
    }

    #[tokio::test]
    async fn test_inductor() {
        dotenv().ok();
        let args = Args::parse();
        let pool = PgPool::connect(args.database_url.as_str()).await.unwrap();
        let request = JLCPartRequest {
            type_field: "inductor".to_string(),
            designator_prefix: "L".to_string(),
            mpn: "generic_inductor".to_string(),
            value: JLCValue {
                unit: "microhenry".to_string(),
                min_val: 4.0,
                max_val: 5.0,
                nominal: 4.5,
            },
            package: Some("0603".to_string()),
        };
        let inductor_result = find_inductor(pool, request.clone()).await;
        assert!(inductor_result.is_ok());
        let (component_vec, _jlc_value) = inductor_result.unwrap();
        assert!(component_vec.len() > 0);
        assert!(component_vec[0].package == Some("0603".to_string()));
    }
}
