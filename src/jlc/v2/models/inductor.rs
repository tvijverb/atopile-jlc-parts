use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum InductorUnit {
    PicoHenry,
    NanoHenry,
    MicroHenry,
    MilliHenry,
    Henry,
    KiloHenry,
    MegaHenry,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct InductorRequest {
    pub unit: InductorUnit,
    pub value: f64,
    pub package: Option<String>,
    pub tolerance_percentage: Option<f64>,
    pub absolute_tolerance: Option<f64>,
    pub absolute_tolerance_unit: Option<InductorUnit>,
}
