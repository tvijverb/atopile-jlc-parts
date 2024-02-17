use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum ResistorUnit {
    MegaOhm,
    KiloOhm,
    Ohm,
    MiliOhm,
    MicroOhm,
    NanoOhm,
    PicoOhm,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct ResistorRequest {
    pub unit: ResistorUnit,
    pub value: f64,
    pub package: Option<String>,
    pub tolerance_percentage: Option<f64>,
    pub absolute_tolerance: Option<f64>,
    pub absolute_tolerance_unit: Option<ResistorUnit>,
}
