use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Stage {
    Fusion,
    Conversion,
    Refining,
    Atomization,
    Printing,
}

impl Stage {
    pub fn to_string(&self) -> &'static str {
        match self {
            Stage::Fusion => "Fusión Primaria",
            Stage::Conversion => "Conversión",
            Stage::Refining => "Refino Térmico",
            Stage::Atomization => "Atomización",
            Stage::Printing => "Impresión 3D",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct SimState {
    pub selected_stage: Signal<Stage>,
    pub temp: Signal<f64>,
    pub gas_level: Signal<f64>,
    pub o2_flow: Signal<i32>,
    pub scrap_rate: Signal<i32>,
    pub voltage: Signal<i32>,
    pub atomization_gas: Signal<i32>,
    pub laser_power: Signal<i32>,
    pub is_panic: Signal<bool>,
    pub is_finished: Signal<bool>,
}
