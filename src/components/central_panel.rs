use dioxus::prelude::*;
use crate::models::{Stage, SimState};

#[component]
pub fn CentralCard(title: &'static str, val: String, max: f64, current_f: f64, color: &'static str) -> Element {
    let width_pct = (current_f / max * 100.0).clamp(0.0, 100.0);
    
    let bg_color = match color {
        "orange" => "bg-orange-500",
        "blue" => "bg-blue-500",
        "amber" => "bg-amber-500",
        "emerald" => "bg-emerald-500",
        "purple" => "bg-purple-500",
        "indigo" => "bg-indigo-500",
        "rose" => "bg-rose-500",
        _ => "bg-gray-500"
    };

    let text_color = match color {
        "orange" => "text-orange-600",
        "blue" => "text-blue-600",
        "amber" => "text-amber-600",
        "emerald" => "text-emerald-600",
        "purple" => "text-purple-600",
        "indigo" => "text-indigo-600",
        "rose" => "text-rose-600",
        _ => "text-gray-600"
    };

    rsx! {
        div { class: "bg-white p-6 rounded-2xl border border-gray-200 shadow-sm flex flex-col items-center justify-between",
            h3 { class: "text-gray-400 mb-2 text-xs font-bold tracking-widest", "{title}" }
            div { class: "text-4xl font-black {text_color}", "{val}" }
            div { class: "w-full h-3 bg-gray-100 rounded-full mt-6 overflow-hidden relative",
                div { 
                    class: "h-full {bg_color} transition-all duration-300",
                    style: "width: {width_pct}%;"
                }
            }
        }
    }
}

#[component]
pub fn CentralDisplay(state: SimState) -> Element {
    let stage = *state.selected_stage.read();
    
    let chart_color = match stage {
        Stage::Fusion => "bg-orange-400",
        Stage::Conversion => "bg-amber-500",
        Stage::Refining => "bg-blue-400",
        Stage::Atomization => "bg-purple-400",
        Stage::Printing => "bg-rose-400",
    };

    rsx! {
        div {
            class: "w-2/4 flex flex-col p-8 bg-gray-50 relative overflow-y-auto",
            
            if *state.is_panic.read() {
                div {
                    class: "absolute inset-0 bg-red-500/10 z-0 animate-pulse pointer-events-none"
                }
                div {
                    class: "absolute top-4 left-1/2 -translate-x-1/2 bg-red-600 text-white shadow-xl px-6 py-2 rounded-full font-bold z-10 animate-fade-in animate-bounce",
                    "¡ALERTA DE SISTEMA! PARADA DE EMERGENCIA"
                }
            }
            
            div { class: "z-10 h-full flex flex-col",
                h1 { class: "text-4xl font-light mb-2 text-gray-800", "{stage.to_string()}" }
                p { class: "text-gray-500 mb-8 font-medium", "Monitorización en Tiempo Real (1ms tick)" }
                
                div { class: "grid grid-cols-2 gap-6 mb-8",
                    match stage {
                        Stage::Fusion => rsx!{
                            CentralCard { title: "TEMPERATURA (HORNO)", val: format!("{}°C", *state.temp.read() as i32), max: 2000.0, current_f: *state.temp.read(), color: "orange" }
                            CentralCard { title: "OXÍGENO INYECTADO", val: format!("{}%", *state.o2_flow.read()), max: 100.0, current_f: *state.o2_flow.read() as f64, color: "blue" }
                        },
                        Stage::Conversion => rsx!{
                            CentralCard { title: "TEMP. DEL BAÑO", val: format!("{}°C", *state.temp.read() as i32), max: 1500.0, current_f: *state.temp.read(), color: "orange" }
                            CentralCard { title: "VISCOSIDAD ESCORIA", val: format!("{:.1} Pa·s", 5.0 + (1100.0 - *state.temp.read()) * 0.1), max: 50.0, current_f: 5.0 + (1100.0 - *state.temp.read()) * 0.1, color: "amber" }
                        },
                        Stage::Refining => rsx!{
                            CentralCard { title: "SOBREPOTENCIAL (η)", val: format!("{} mV", *state.voltage.read() * 5), max: 500.0, current_f: (*state.voltage.read() * 5) as f64, color: "blue" }
                            CentralCard { title: "MASA DEPOSITADA", val: format!("{:.2} kg", 100.0 - *state.gas_level.read()), max: 100.0, current_f: 100.0 - *state.gas_level.read(), color: "emerald" }
                        },
                        Stage::Atomization => rsx!{
                            CentralCard { title: "TAMAÑO PARTÍCULA (d_m)", val: format!("{} μm", 120 - *state.atomization_gas.read()), max: 200.0, current_f: (120 - *state.atomization_gas.read()) as f64, color: "purple" }
                            CentralCard { title: "PRESIÓN DE GAS", val: format!("{} PSI", *state.atomization_gas.read() * 2), max: 200.0, current_f: (*state.atomization_gas.read() * 2) as f64, color: "indigo" }
                        },
                        Stage::Printing => rsx!{
                            CentralCard { title: "DENSIDAD ENERGÉTICA (VED)", val: format!("{} J/mm³", *state.laser_power.read() * 2), max: 200.0, current_f: (*state.laser_power.read() * 2) as f64, color: "rose" }
                            CentralCard { title: "TEMPERATURA FUSIÓN", val: format!("{}°C", *state.temp.read() as i32), max: 3000.0, current_f: *state.temp.read(), color: "orange" }
                        }
                    }
                }

                div { class: "bg-white flex-1 p-6 rounded-2xl border border-gray-200 shadow-sm relative overflow-hidden flex flex-col",
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
