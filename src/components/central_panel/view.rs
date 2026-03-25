use dioxus::prelude::*;
use crate::models::Stage;
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

    rsx! {
        div {
            class: "w-2/4 flex flex-col p-8 bg-slate-900/50 relative overflow-y-auto",
            
            if *state.is_panic.read() {
                div {
                    class: "absolute inset-0 bg-red-900/40 z-0 animate-pulse pointer-events-none"
                }
                div {
                    class: "absolute top-4 left-1/2 -translate-x-1/2 bg-red-600 text-white shadow-xl px-6 py-2 rounded-full font-bold z-10 animate-fade-in animate-bounce",
                    "¡ALERTA DE SISTEMA! PARADA DE EMERGENCIA"
                }
            }
            
            div { class: "z-10 h-full flex flex-col",
                h1 { class: "text-4xl font-light mb-2 text-slate-100", "{stage.to_string()}" }
                p { class: "text-slate-400 mb-8 font-medium", "Monitorización en Tiempo Real (1ms tick)" }
                
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
