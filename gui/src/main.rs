#![allow(non_snake_case)]
mod state;
mod components;

use dioxus::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

use simulador_core::models::*;
use simulador_core::physics;
use state::SimState;
use components::navigation::Sidebar;
use components::central_panel::CentralDisplay;
use components::console::ControlPanel;

fn main() {
    let conf = dioxus::desktop::Config::new()
        .with_custom_head(r#"<script src="https://cdn.tailwindcss.com"></script>"#.to_string());
    LaunchBuilder::desktop().with_cfg(conf).launch(App);
}

fn App() -> Element {
    let mut selected_stage = use_signal(|| Stage::Fusion);
    let mut temp = use_signal(|| 1200.0);
    let mut gas_level = use_signal(|| 100.0);
    
    let mut current_status = use_signal(|| SystemStatus::Normal);
    let mut process_report = use_signal(|| None::<ProcessReport>);

    // Encapsulated Inputs
    let flash_input = use_signal(|| HornoFlashInput { o2_flow: 50, silica_flux: 50 });
    let conv_input = use_signal(|| ConvertidorInput { o2_flow: 50, scrap_added: 20 });
    let afino_input = use_signal(|| AfinoInput { reducing_gas: 50 });
    let electro_input = use_signal(|| ElectrolysisInput { current_amps: 50 });
    let atom_input = use_signal(|| AtomizationInput { gas_pressure: 80 });
    let print_input = use_signal(|| PrintingInput { laser_power: 50 });
    
    let mut is_panic = use_signal(|| false);
    let mut is_finished = use_signal(|| false);

    let state = SimState {
        selected_stage,
        temp,
        gas_level,
        current_status,
        process_report,
        
        flash_input,
        conv_input,
        afino_input,
        electro_input,
        atom_input,
        print_input,
        
        is_panic,
        is_finished,
    };

    // Control Loop calling Physics Engine
    use_future(move || async move {
        loop {
            sleep(Duration::from_millis(1000)).await;
            if !*is_panic.read() && !*is_finished.read() {
                let stage = *selected_stage.read();
                
                let new_temp = match stage {
                    Stage::Fusion => physics::calculate_fusion_temp(1200.0, flash_input.read().o2_flow),
                    Stage::Conversion => physics::calculate_conversion_temp(1100.0, conv_input.read().scrap_added),
                    Stage::Refining => 1100.0,
                    Stage::Electrolysis => physics::calculate_electrolytic_temp(electro_input.read().current_amps),
                    Stage::Atomization => 400.0,
                    Stage::Printing => physics::calculate_laser_temp(print_input.read().laser_power),
                };
                
                temp.set(new_temp);
                
                let current_gas = *gas_level.read();
                if current_gas > 0.0 {
                    gas_level.set(current_gas - 0.5);
                }

                let report = match stage {
                    Stage::Fusion => physics::evaluate_flash(&*flash_input.read()),
                    Stage::Conversion => physics::evaluate_conversion(&*conv_input.read()),
                    Stage::Refining => physics::evaluate_afino(&*afino_input.read()),
                    Stage::Electrolysis => physics::evaluate_electrolysis(&*electro_input.read()),
                    Stage::Atomization => physics::evaluate_atomization(&*atom_input.read()),
                    Stage::Printing => physics::evaluate_printing(&*print_input.read()),
                };

                current_status.set(report.status.clone());
                process_report.set(Some(report));
            }
        }
    });

    if *is_finished.read() {
        if let Some(report) = &*process_report.read() {
            let is_fatal = report.status.is_blocking();
            let status_color = match report.status {
                SystemStatus::Normal => "text-emerald-400",
                SystemStatus::Warning(_) => "text-amber-400",
                SystemStatus::Critical(_) | SystemStatus::Fatal(_) => "text-red-500",
            };
            
            return rsx! {
                div {
                    class: "flex flex-col items-center justify-center h-screen bg-[#0f172a] text-slate-200 font-sans",
                    h1 { class: "text-4xl font-bold mb-8 text-blue-400 tracking-wider", "REPORTE DE SALIDA" }
                    
                    div {
                        class: "bg-slate-800 p-8 rounded-xl shadow-2xl w-1/2 border border-slate-700",
                        table {
                            class: "w-full text-left text-xl",
                            tbody {
                                tr {
                                    class: "border-b border-slate-700",
                                    th { class: "py-4 text-slate-300 font-bold", "Variable" }
                                    th { class: "py-4 text-slate-300 font-bold", "Resultado" }
                                }
                                tr {
                                    class: "border-b border-slate-700 text-slate-400",
                                    td { class: "py-4", "Pureza Resultante" }
                                    td { class: "py-4 text-emerald-400 font-bold", "{report.output_purity}% Cu" }
                                }
                                tr {
                                    class: "border-b border-slate-700 text-slate-400",
                                    td { class: "py-4", "Subproductos" }
                                    td { class: "py-4 font-bold text-slate-200", "{report.byproducts}" }
                                }
                                tr {
                                    class: "text-slate-400",
                                    td { class: "py-4", "Diagnóstico FSM" }
                                    td { class: "py-4 {status_color} font-bold text-sm", "{report.status.message()}" }
                                }
                            }
                        }
                    }
                    
                    if is_fatal {
                        div { class: "mt-8 text-3xl font-bold text-red-500 animate-pulse tracking-widest uppercase", "TRANSICIÓN BLOQUEADA" }
                    } else {
                        div { class: "mt-8 text-3xl font-bold text-emerald-400 tracking-widest uppercase", "LOTE APROBADO" }
                    }
                    
                    div { class: "mt-12 flex gap-4",
                        button {
                            class: "px-6 py-3 bg-slate-600 hover:bg-slate-500 shadow-md text-white rounded-lg text-xl font-bold transition",
                            onclick: move |_| {
                                is_finished.set(false);
                            },
                            "Volver a Parámetros 👀"
                        }
                        if !is_fatal {
                            button {
                                class: "px-6 py-3 bg-blue-600 hover:bg-blue-500 shadow-md text-white rounded-lg text-xl font-bold transition",
                                onclick: move |_| {
                                    is_finished.set(false);
                                    is_panic.set(false);
                                    
                                    // Mover a la siguiente etapa si no es la última
                                    let next = match *selected_stage.read() {
                                        Stage::Fusion => Stage::Conversion,
                                        Stage::Conversion => Stage::Refining,
                                        Stage::Refining => Stage::Electrolysis,
                                        Stage::Electrolysis => Stage::Atomization,
                                        Stage::Atomization => Stage::Printing,
                                        Stage::Printing => Stage::Fusion, // Reinicia el ciclo
                                    };
                                    selected_stage.set(next);
                                },
                                if *selected_stage.read() == Stage::Printing { "Finalizar Ciclo Maestro" } else { "Avanzar a la Siguiente Etapa ➡️" }
                            }
                        }
                    }
                }
            };
        }
    }

    rsx! {
        div {
            class: "flex h-screen bg-[#0f172a] text-slate-200 font-sans overflow-hidden",
            Sidebar { 
                current_stage: *selected_stage.read(),
                on_stage_change: move |s| selected_stage.set(s),
                on_panic: move |_| is_panic.set(true)
            }
            CentralDisplay { state }
            ControlPanel { state }
        }
    }
}
