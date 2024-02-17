pub mod resistor;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct Component {
    pub lcsc: String,
    pub category_id: i64,
    pub mpn: String,
    pub package: String,
    pub footprint: String,
    pub joints: i64,
    pub manufacturer_id: i64,
    pub basic: bool,
    pub description: String,
    pub datasheet: String,
    pub stock: i64,
    pub price: String,
    pub last_update: i64,
    pub flag: i64,
    pub last_on_stock: i64,
    pub preferred: i64,
    pub dielectric: Option<String>,
}

// Response No Part Found
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct NoPartFound {
    pub code: i64,
    pub message: String,
}