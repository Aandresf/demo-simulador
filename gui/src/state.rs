use dioxus::prelude::*;
use simulador_core::models::*;

#[derive(Clone, Copy, PartialEq)]
pub struct SimState {
    pub selected_stage: Signal<Stage>,
    
    pub temp: Signal<f64>,
    pub gas_level: Signal<f64>,
    pub current_status: Signal<SystemStatus>,
    pub process_report: Signal<Option<ProcessReport>>,
    
    pub flash_input: Signal<HornoFlashInput>,
    pub conv_input: Signal<ConvertidorInput>,
    pub afino_input: Signal<AfinoInput>,
    pub electro_input: Signal<ElectrolysisInput>,
    pub atom_input: Signal<AtomizationInput>,
    pub print_input: Signal<PrintingInput>,
    
    pub is_panic: Signal<bool>,
    pub is_finished: Signal<bool>,
}
