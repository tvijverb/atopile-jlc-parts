pub mod capacitor;
pub mod inductor;
pub mod resistor;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct Component {
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
    pub dielectric: Option<String>,
}

// Response No Part Found
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct NoPartFound {
    pub code: i64,
    pub message: String,
}
