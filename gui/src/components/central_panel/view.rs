use dioxus::prelude::*;
use simulador_core::models::{Stage, SystemStatus};
use super::model::{CentralDisplayProps, CentralCardProps};

#[component]
pub fn CentralDisplay(props: CentralDisplayProps) -> Element {
    let state = props.state;
    let stage = *state.selected_stage.read();

    let status = state.current_status.read().clone();
    let (status_color, status_border) = match status {
        SystemStatus::Normal => ("text-emerald-600", "border-emerald-500/50 bg-emerald-100/50"),
        SystemStatus::Warning(_) => ("text-amber-600", "border-amber-500/50 bg-amber-100/50"),
        SystemStatus::Critical(_) | SystemStatus::Fatal(_) => ("text-red-600", "border-red-500/50 bg-red-100/50"),
    };

    rsx! {
        div {
            class: "w-2/4 flex flex-col p-8 bg-slate-50 relative overflow-y-auto gap-6 border-x border-slate-200 shadow-xl z-20",
            
            div { class: "flex justify-between items-center bg-white p-6 rounded-2xl border border-slate-300 shadow-sm",
                div {
                    h2 { class: "text-2xl font-black tracking-widest text-slate-800 uppercase mb-1", "{stage.to_string()}" }
                    div { class: "text-slate-500 font-bold text-sm", "Simulando Dinámica en Tiempo Real" }
                }
                div { class: "flex flex-col items-end",
                    div { class: "text-xs font-bold text-slate-500 tracking-wider mb-1", "DIAGNÓSTICO FSM" }
                    div { class: "px-4 py-2 rounded-lg border font-bold tracking-widest {status_color} {status_border} animate-pulse shadow-sm",
                        "{status.message()}"
                    }
                }
            }

            div { class: "grid grid-cols-2 gap-4 h-32 shrink-0",
                match stage {
                    Stage::Fusion => rsx!{
                        CentralCard { title: "TEMPERATURA (HORNO)", val: format!("{}°C", *state.temp.read() as i32), max: 2000.0, current_f: *state.temp.read(), color: "orange" }
                        CentralCard { title: "VISCOSIDAD ESCORIA", val: format!("{:.1} Pa·s", simulador_core::physics::calculate_slag_viscosity(*state.temp.read())), max: 50.0, current_f: simulador_core::physics::calculate_slag_viscosity(*state.temp.read()), color: "amber" }
                    },
                    Stage::Conversion => rsx!{
                        CentralCard { title: "TEMP. DEL BAÑO", val: format!("{}°C", *state.temp.read() as i32), max: 1500.0, current_f: *state.temp.read(), color: "orange" }
                        CentralCard { title: "DENSIDAD MAT.", val: format!("{:.1} g/cm³", 4.5 + (*state.temp.read() / 2000.0)), max: 10.0, current_f: 5.0, color: "rose" }
                    },
                    Stage::Refining => rsx!{
                        CentralCard { title: "PUREZA ESTIMADA", val: format!("99.{}%", 5 + (*state.temp.read() as i32 / 100)), max: 100.0, current_f: 99.5, color: "emerald" }
                        CentralCard { title: "CANTIDAD O2", val: format!("{} ppm", *state.temp.read() as i32 / 10), max: 500.0, current_f: 100.0, color: "cyan" }
                    },
                    Stage::Electrolysis => rsx!{
                        CentralCard { title: "VOLTAJE DE CELDA", val: format!("{:.2} V", *state.temp.read() / 800.0), max: 2.0, current_f: *state.temp.read() / 800.0, color: "emerald" }
                        CentralCard { title: "AMPERAJE (A)", val: format!("{} A", state.electro_input.read().current_amps), max: 100.0, current_f: state.electro_input.read().current_amps as f64, color: "purple" }
                    },
                    Stage::Atomization => rsx!{
                        CentralCard { title: "TAMAÑO PARTÍCULA (d)", val: format!("{:.1} μm", simulador_core::physics::calculate_particle_size(state.atom_input.read().gas_pressure)), max: 200.0, current_f: simulador_core::physics::calculate_particle_size(state.atom_input.read().gas_pressure), color: "purple" }
                        CentralCard { title: "PRESIÓN DE GAS", val: format!("{} PSI", state.atom_input.read().gas_pressure * 2), max: 200.0, current_f: (state.atom_input.read().gas_pressure * 2) as f64, color: "indigo" }
                    },
                    Stage::Printing => rsx!{
                        CentralCard { title: "DENSIDAD ENERGÉTICA (VED)", val: format!("{:.1} J/mm³", simulador_core::physics::calculate_ved(state.print_input.read().laser_power)), max: 200.0, current_f: simulador_core::physics::calculate_ved(state.print_input.read().laser_power), color: "rose" }
                        CentralCard { title: "TEMPERATURA FUSIÓN", val: format!("{}°C", *state.temp.read() as i32), max: 3000.0, current_f: *state.temp.read(), color: "orange" }
                    }
                }
            }

            div { class: "w-full h-[35%] relative overflow-hidden flex flex-col bg-white rounded-2xl border border-slate-300 p-2 shadow-sm mt-auto",
                match stage {
                    Stage::Fusion => rsx!{ crate::components::central_panel::charts::FlashFurnaceCharts { 
                        o2_flow: state.flash_input.read().o2_flow as f64, silica: state.flash_input.read().silica_flux as f64
                    }},
                    Stage::Conversion => rsx!{ crate::components::central_panel::charts::ConverterCharts { 
                        o2_flow: state.conv_input.read().o2_flow as f64, scrap: state.conv_input.read().scrap_added as f64
                    }},
                    Stage::Refining => rsx!{ crate::components::central_panel::charts::ThermalRefiningCharts { 
                        red_gas: state.afino_input.read().reducing_gas as f64
                    }},
                    Stage::Electrolysis => rsx!{ crate::components::central_panel::charts::ElectrolysisCharts { 
                        amps: state.electro_input.read().current_amps as f64
                    }},
                    Stage::Atomization => rsx!{ crate::components::central_panel::charts::AtomizationCharts { 
                        pressure: state.atom_input.read().gas_pressure as f64
                    }},
                    Stage::Printing => rsx!{ crate::components::central_panel::charts::AdditiveManufacturingCharts { 
                        laser: state.print_input.read().laser_power as f64
                    }},
                }
            }
        }
    }
}

#[component]
fn CentralCard(props: CentralCardProps) -> Element {
    let percentage = (props.current_f / props.max) * 100.0;
    
    rsx! {
        div { class: "bg-white p-4 rounded-xl border border-slate-300 flex flex-col justify-between shadow-sm relative overflow-hidden group hover:border-{props.color}-400 transition-colors",
            div { class: "flex justify-between items-start z-10",
                div { class: "text-slate-500 font-bold text-xs tracking-widest", "{props.title}" }
                div { class: "text-{props.color}-600 font-black text-xl drop-shadow-sm", "{props.val}" }
            }
            div { class: "h-2 w-full bg-slate-100 rounded-full mt-4 overflow-hidden z-10 border border-slate-200",
                div {
                    class: "h-full bg-{props.color}-500 rounded-full transition-all duration-1000 ease-out",
                    style: "width: {percentage}%"
                }
            }
        }
    }
}
