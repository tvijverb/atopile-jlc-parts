use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum CapacitorUnit {
    PicoFarad,
    NanoFarad,
    MicroFarad,
    MilliFarad,
    Farad,
    KiloFarad,
    MegaFarad,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct CapacitorRequest {
    pub unit: CapacitorUnit,
    pub value: f64,
    pub package: Option<String>,
    pub tolerance_percentage: Option<f64>,
    pub absolute_tolerance: Option<f64>,
    pub absolute_tolerance_unit: Option<CapacitorUnit>,
}
