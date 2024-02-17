use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

// LEGACY Request Models
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct JLCPartRequest {
    #[serde(rename = "designator_prefix")]
    pub designator_prefix: String,
    pub mpn: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: JLCValue,
    pub package: Option<String>,
}

// LEGACY Request Models
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct JLCValue {
    pub unit: String,
    #[serde(rename = "min_val")]
    pub min_val: f64,
    #[serde(rename = "max_val")]
    pub max_val: f64,
    pub nominal: f64,
    pub tolerance: f64,
    #[serde(rename = "tolerance_pct")]
    pub tolerance_pct: f64,
}

// LEGACY Response Models
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct JLCPartResponse {
    pub best_component: BestComponent,
}

// LEGACY Response Models
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BestComponent {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uuid: String,
    pub value: JLCValue,
    pub stock: i64,
    pub code: String,
    pub voltage: Voltage,
    #[serde(rename = "price_usd")]
    pub price_usd: Option<i64>,
    pub area: Option<f64>,
    pub footprint: Footprint,
    pub mpn: String,
    pub datasheet: String,
    pub category: String,
    #[serde(rename = "lcsc_id")]
    pub lcsc_id: String,
    pub package: String,
    #[serde(rename = "footprint_data")]
    pub footprint_data: FootprintData,
    pub dielectric: Option<String>,
    #[serde(rename = "basic_part")]
    pub basic_part: bool,
}

// LEGACY Response Models
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Voltage {
    #[serde(rename = "min_val")]
    pub min_val: i64,
    pub unit: String,
    #[serde(rename = "max_val")]
    pub max_val: i64,
}

// LEGACY Response Models
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Footprint {
    pub kicad: String,
}

// LEGACY Response Models
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FootprintData {
    pub kicad: String,
}

// Response No Part Found
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct NoPartFound {
    pub code: i64,
    pub message: String,
}
