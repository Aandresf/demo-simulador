use dioxus::prelude::*;
use simulador_core::models::{Stage, SystemStatus};
use super::model::{CentralDisplayProps, CentralCardProps};
use super::controller::{get_chart_color, get_card_styles};

#[component]
pub fn CentralCard(props: CentralCardProps) -> Element {
    let width_pct = (props.current_f / props.max * 100.0).clamp(0.0, 100.0);
    let (bg_color, text_color) = get_card_styles(props.color);

    rsx! {
        div { class: "bg-slate-800 p-6 rounded-2xl border border-slate-700/50 shadow-inner flex flex-col items-center justify-between",
            h3 { class: "text-gray-400 mb-2 text-xs font-bold tracking-widest", "{props.title}" }
            div { class: "text-4xl font-mono {text_color}", "{props.val}" }
            div { class: "w-full h-3 bg-slate-900 rounded-full mt-6 overflow-hidden relative",
                div { 
                    class: "h-full {bg_color} transition-all duration-300",
                    style: "width: {width_pct}%;"
                }
            }
        }
    }
}

#[component]
pub fn CentralDisplay(props: CentralDisplayProps) -> Element {
    let state = props.state;
    let stage = *state.selected_stage.read();
    let chart_color = get_chart_color(stage);

    let status = state.current_status.read().clone();
    let (status_color, status_border) = match status {
        SystemStatus::Normal => ("text-emerald-400", "border-emerald-500/50 bg-emerald-900/20"),
        SystemStatus::Warning(_) => ("text-amber-400", "border-amber-500/50 bg-amber-900/20"),
        SystemStatus::Critical(_) | SystemStatus::Fatal(_) => ("text-red-500", "border-red-500/50 bg-red-900/20"),
    };

    rsx! {
        div {
            class: "w-2/4 flex flex-col p-8 bg-slate-900/50 relative overflow-y-auto gap-6",
            
            if *state.is_panic.read() {
                div {
                    class: "absolute inset-0 bg-red-900/40 z-0 animate-pulse pointer-events-none"
                }
            }
            
            // Objective Card (Top)
            div { class: "z-10 w-full p-6 rounded-xl border border-slate-700 bg-slate-800/80 shadow-md",
                p { class: "text-slate-300 font-medium italic text-lg leading-relaxed text-center", "{stage.objective()}" }
            }
            
            // Status Card
            div { class: "z-10 w-full p-4 rounded-xl border {status_border} shadow-md flex justify-between items-center transition-colors duration-500",
                span { class: "text-slate-400 font-bold tracking-widest text-sm", "ESTADO DEL SISTEMA FSM:" }
                span { class: "font-black tracking-wide {status_color}", "{status.message()}" }
            }
            
            div { class: "z-10 flex-1 flex flex-col",
                div { class: "flex justify-between items-end mb-6 mt-4",
                    h1 { class: "text-3xl font-light text-slate-100", "{stage.to_string()}" }
                    p { class: "text-slate-500 font-mono text-sm", "Tick: 1000ms" }
                }
                
                div { class: "grid grid-cols-2 gap-6 mb-8",
                    match stage {
                        Stage::Fusion => rsx!{
                            CentralCard { title: "TEMPERATURA (HORNO)", val: format!("{}°C", *state.temp.read() as i32), max: 2000.0, current_f: *state.temp.read(), color: "orange" }
                            CentralCard { title: "VISCOSIDAD ESCORIA", val: format!("{:.1} Pa·s", simulador_core::physics::calculate_slag_viscosity(*state.temp.read())), max: 50.0, current_f: simulador_core::physics::calculate_slag_viscosity(*state.temp.read()), color: "amber" }
                        },
                        Stage::Conversion => rsx!{
                            CentralCard { title: "TEMP. DEL BAÑO", val: format!("{}°C", *state.temp.read() as i32), max: 1500.0, current_f: *state.temp.read(), color: "orange" }
                            CentralCard { title: "OXÍGENO INYECTADO", val: format!("{}%", state.conv_input.read().o2_flow), max: 100.0, current_f: state.conv_input.read().o2_flow as f64, color: "blue" }
                        },
                        Stage::Refining => rsx!{
                            CentralCard { title: "GAS REDUCTOR", val: format!("{}%", state.afino_input.read().reducing_gas), max: 100.0, current_f: state.afino_input.read().reducing_gas as f64, color: "emerald" }
                            CentralCard { title: "TEMPERATURA AFINO", val: format!("{}°C", *state.temp.read() as i32), max: 1500.0, current_f: *state.temp.read(), color: "orange" }
                        },
                        Stage::Electrolysis => rsx!{
                            CentralCard { title: "SOBREPOTENCIAL TÉRMICO", val: format!("{}°C", *state.temp.read() as i32), max: 1000.0, current_f: *state.temp.read(), color: "blue" }
                            CentralCard { title: "AMPERAJE (A)", val: format!("{} A", state.electro_input.read().current_amps), max: 100.0, current_f: state.electro_input.read().current_amps as f64, color: "purple" }
                        },
                        Stage::Atomization => rsx!{
                            CentralCard { title: "TAMAÑO PARTÍCULA (d_m)", val: format!("{:.1} μm", simulador_core::physics::calculate_particle_size(state.atom_input.read().gas_pressure)), max: 200.0, current_f: simulador_core::physics::calculate_particle_size(state.atom_input.read().gas_pressure), color: "purple" }
                            CentralCard { title: "PRESIÓN DE GAS", val: format!("{} PSI", state.atom_input.read().gas_pressure * 2), max: 200.0, current_f: (state.atom_input.read().gas_pressure * 2) as f64, color: "indigo" }
                        },
                        Stage::Printing => rsx!{
                            CentralCard { title: "DENSIDAD ENERGÉTICA (VED)", val: format!("{:.1} J/mm³", simulador_core::physics::calculate_ved(state.print_input.read().laser_power)), max: 200.0, current_f: simulador_core::physics::calculate_ved(state.print_input.read().laser_power), color: "rose" }
                            CentralCard { title: "TEMPERATURA FUSIÓN", val: format!("{}°C", *state.temp.read() as i32), max: 3000.0, current_f: *state.temp.read(), color: "orange" }
                        }
                    }
                }

                div { class: "bg-slate-800 flex-1 p-6 rounded-2xl border border-slate-700/50 shadow-inner relative overflow-hidden flex flex-col",
                    h3 { class: "text-gray-400 mb-4 text-xs font-bold tracking-widest",
                        if stage == Stage::Printing { "PERFIL TÉRMICO DE ROSENTHAL" } else { "CINÉTICA DE REACCIÓN EDO" }
                    }
                    div { class: "flex items-end flex-1 gap-1",
                        for _i in 0..40 {
                            div {
                                class: "w-full flex-1 rounded-t opacity-90 transition-all duration-500 {chart_color}",
                                style: "height: {20.0 + rand::random::<f32>() * 80.0}%;"
                            }
                        }
                    }
                }
            }
        }
    }
}
