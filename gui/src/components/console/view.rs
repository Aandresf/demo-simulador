use dioxus::prelude::*;
use simulador_core::models::*;
use crate::state::SimState;

#[component]
pub fn ControlPanel(state: SimState) -> Element {
    let stage = *state.selected_stage.read();
    
    rsx! {
        div {
            class: "w-1/4 bg-white border-l border-slate-300 p-6 flex flex-col shadow-lg z-10",
            h2 { class: "text-xl font-black mb-8 text-slate-700 tracking-wider", "CONSOLA DE CONTROL" }
            
            div { class: "flex flex-col gap-10 flex-1",
                match stage {
                    Stage::Fusion => rsx!{
                        ConsoleSlider { 
                            name: "Flujo de O2", 
                            val: state.flash_input.read().o2_flow,
                            tooltip: "Controla la combustión autógena. Su exceso funde refractarios. Su defecto congela la mata.",
                            min: 0, max: 100, unit: "%",
                            on_change: move |v| { let mut i = *state.flash_input.read(); i.o2_flow = v; state.flash_input.set(i); }
                        }
                        ConsoleSlider { 
                            name: "Flujo de Sílice", 
                            val: state.flash_input.read().silica_flux,
                            tooltip: "Fundente para capturar hierro. Su defecto crea escoria viscosa que tapona el horno (Crítico).",
                            min: 0, max: 100, unit: " kg/s",
                            on_change: move |v| { let mut i = *state.flash_input.read(); i.silica_flux = v; state.flash_input.set(i); }
                        }
                    },
                    Stage::Conversion => rsx!{
                        ConsoleSlider { 
                            name: "Flujo de O2", 
                            val: state.conv_input.read().o2_flow,
                            tooltip: "Oxida azufre e hierro. Muy lento = Blíster sucio. Muy rápido = Quema el cobre.",
                            min: 0, max: 100, unit: "%",
                            on_change: move |v| { let mut i = *state.conv_input.read(); i.o2_flow = v; state.conv_input.set(i); }
                        }
                        ConsoleSlider { 
                            name: "Adición Chatarra", 
                            val: state.conv_input.read().scrap_added,
                            tooltip: "Refrigerante físico. Menos de 10% causa fundición del refractario termal (Fatal).",
                            min: 0, max: 100, unit: " kg/s",
                            on_change: move |v| { let mut i = *state.conv_input.read(); i.scrap_added = v; state.conv_input.set(i); }
                        }
                    },
                    Stage::Refining => rsx!{
                        ConsoleSlider { 
                            name: "Gas Reductor", 
                            val: state.afino_input.read().reducing_gas,
                            tooltip: "Roba oxígeno residual. >70% causa absorción de hidrógeno y ánodos deformes (Crítico).",
                            min: 0, max: 100, unit: "%",
                            on_change: move |v| { let mut i = *state.afino_input.read(); i.reducing_gas = v; state.afino_input.set(i); }
                        }
                    },
                    Stage::Electrolysis => rsx!{
                        ConsoleSlider { 
                            name: "Amperaje", 
                            val: state.electro_input.read().current_amps,
                            tooltip: "Fuerza eléctrica. Muy alta arranca impurezas y destruye el grado 99.99% (Fatal).",
                            min: 0, max: 100, unit: " A",
                            on_change: move |v| { let mut i = *state.electro_input.read(); i.current_amps = v; state.electro_input.set(i); }
                        }
                    },
                    Stage::Atomization => rsx!{
                        ConsoleSlider { 
                            name: "Presión de Gas", 
                            val: state.atom_input.read().gas_pressure,
                            tooltip: "Impacta el líquido. Presión baja crea polvo gigante irregular que atasca la 3D (Crítico).",
                            min: 0, max: 100, unit: " PSI",
                            on_change: move |v| { let mut i = *state.atom_input.read(); i.gas_pressure = v; state.atom_input.set(i); }
                        }
                    },
                    Stage::Printing => rsx!{
                        ConsoleSlider { 
                            name: "Potencia Láser", 
                            val: state.print_input.read().laser_power,
                            tooltip: "Menos de 30% causa porosidad y falta de fusión. Exceso evapora el cobre (Warning).",
                            min: 0, max: 100, unit: "%",
                            on_change: move |v| { let mut i = *state.print_input.read(); i.laser_power = v; state.print_input.set(i); }
                        }
                    }
                }
            }
            
            button {
                class: "w-full py-5 bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-700 text-white font-bold rounded-xl text-lg shadow-xl shadow-blue-200/40 transition transform hover:-translate-y-1 active:translate-y-0 disabled:opacity-50",
                onclick: move |_| state.is_finished.set(true),
                "EVALUAR DESEMPEÑO"
            }
        }
    }
}

#[component]
fn ConsoleSlider(name: &'static str, val: i32, tooltip: &'static str, min: i32, max: i32, unit: &'static str, on_change: EventHandler<i32>) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2 group",
            div { class: "flex justify-between items-end",
                label { 
                    class: "text-sm text-slate-600 tracking-wider font-bold cursor-help border-b border-dashed border-slate-600 transition-colors hover:text-blue-600 hover:border-blue-400",
                    title: "{tooltip}",
                    "{name} ℹ️" 
                }
                span { class: "font-mono text-blue-400 font-black", "{val}{unit}" }
            }
            input {
                r#type: "range",
                min: "{min}",
                max: "{max}",
                value: "{val}",
                oninput: move |e| {
                    if let Ok(v) = e.value().parse::<i32>() {
                        on_change.call(v);
                    }
                },
                class: "w-full h-2 bg-slate-200 rounded-lg appearance-none cursor-pointer accent-blue-500 hover:accent-blue-400 transition"
            }
        }
    }
}
