use dioxus::prelude::*;
use crate::models::Stage;
use super::model::{ControlPanelProps, ConsoleSliderProps};
use super::controller::{on_slider_input, finish_simulation};

#[component]
pub fn ControlPanel(props: ControlPanelProps) -> Element {
    let state = props.state;
    let stage = *state.selected_stage.read();
    let o2_val = *state.o2_flow.read();
    let v_val = *state.voltage.read();
    
    rsx! {
        div {
            class: "w-1/4 bg-slate-800 border-l border-slate-700 p-6 flex flex-col shadow-lg z-10",
            h2 { class: "text-xl font-black mb-8 text-slate-300 tracking-wider", "CONSOLA DE CONTROL" }
            
            div { class: "flex flex-col gap-10 flex-1",
                match stage {
                    Stage::Fusion => rsx!{
                        ConsoleSlider { name: "Flujo de O2 (Arrhenius)", val: state.o2_flow, min: 0, max: 100, unit: "%" }
                        if o2_val < 30 {
                            div { class: "-mt-6 text-red-500 font-bold text-xs animate-pulse", "REACCIÓN EXTINTA" }
                        }
                    },
                    Stage::Conversion => rsx!{
                        ConsoleSlider { name: "Flujo de O2", val: state.o2_flow, min: 0, max: 100, unit: "%" }
                        ConsoleSlider { name: "Adición de Chatarra", val: state.scrap_rate, min: 0, max: 100, unit: " kg/s" }
                        if *state.temp.read() < 1000.0 {
                            div { class: "-mt-6 text-red-500 font-bold text-xs animate-pulse", "RIESGO DE CONGELACIÓN" }
                        }
                    },
                    Stage::Refining => rsx!{
                        ConsoleSlider { name: "Voltaje Electrolítico", val: state.voltage, min: 0, max: 100, unit: "V" }
                        if v_val > 80 {
                            div { class: "-mt-6 text-red-500 font-bold text-xs animate-pulse", "IMPUREZAS DETECTADAS" }
                        }
                    },
                    Stage::Atomization => rsx!{
                        ConsoleSlider { name: "Presión Argón", val: state.atomization_gas, min: 10, max: 100, unit: " PSI" }
                    },
                    Stage::Printing => rsx!{
                        ConsoleSlider { name: "Potencia Láser", val: state.laser_power, min: 0, max: 100, unit: "%" }
                    }
                }
            }
            
            button {
                class: "w-full py-5 bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-700 text-white font-bold rounded-xl text-lg shadow-xl shadow-blue-900/40 transition transform hover:-translate-y-1 active:translate-y-0 disabled:opacity-50",
                onclick: move |_| finish_simulation(state.is_finished),
                "IMPRIMIR PIEZA FINAL"
            }
        }
    }
}

#[component]
fn ConsoleSlider(props: ConsoleSliderProps) -> Element {
    let val_sig = props.val;
    rsx! {
        div { class: "flex flex-col gap-2",
            div { class: "flex justify-between items-end",
                label { class: "text-sm text-slate-400 tracking-wider font-bold", "{props.name}" }
                span { class: "font-mono text-blue-400 font-black", "{(*props.val.read())}{props.unit}" }
            }
            input {
                r#type: "range",
                min: "{props.min}",
                max: "{props.max}",
                value: "{*props.val.read()}",
                oninput: move |e| on_slider_input(val_sig, e.value()),
                class: "w-full h-2 bg-slate-700 rounded-lg appearance-none cursor-pointer accent-blue-500"
            }
        }
    }
}
