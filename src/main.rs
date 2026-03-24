#![allow(non_snake_case)]
use dioxus::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Clone, Copy, PartialEq)]
enum Stage {
    Fusion,
    Conversion,
    Refining,
    Atomization,
    Printing,
}

impl Stage {
    fn to_string(&self) -> &'static str {
        match self {
            Stage::Fusion => "Fusión Primaria",
            Stage::Conversion => "Conversión",
            Stage::Refining => "Refino Térmico",
            Stage::Atomization => "Atomización",
            Stage::Printing => "Impresión 3D",
        }
    }
}

fn main() {
    let conf = dioxus::desktop::Config::new()
        .with_custom_head(r#"<script src="https://cdn.tailwindcss.com"></script>"#.to_string());
    LaunchBuilder::desktop().with_cfg(conf).launch(App);
}

fn App() -> Element {
    // Application State
    let mut selected_stage = use_signal(|| Stage::Fusion);
    let mut temp = use_signal(|| 1200.0);
    let mut gas_level = use_signal(|| 100.0);
    
    // Sliders state
    let mut o2_flow = use_signal(|| 50);
    let mut voltage = use_signal(|| 50);
    let mut laser_power = use_signal(|| 50);
    
    // Global simulation state
    let mut is_panic = use_signal(|| false);
    let mut is_finished = use_signal(|| false);

    // Animations / Ticks
    use_future(move || async move {
        loop {
            sleep(Duration::from_millis(1000)).await;
            if !*is_panic.read() && !*is_finished.read() {
                // Slightly oscillate temperature
                let flow = *o2_flow.read() as f64;
                let new_temp = 1000.0 + (flow * 5.0) + (rand::random::<f64>() * 20.0 - 10.0);
                temp.set(new_temp);
                
                // Drain gas
                let current_gas = *gas_level.read();
                if current_gas > 0.0 {
                    gas_level.set(current_gas - 1.0);
                }
            }
        }
    });

    let o2_val = *o2_flow.read();
    let is_o2_low = o2_val < 15;
    let v_val = *voltage.read();
    let is_v_high = v_val > 85;

    if *is_finished.read() {
        // Result Screen
        let is_defective = *is_panic.read() || o2_val < 20 || v_val > 80 || *laser_power.read() < 30;
        return rsx! {
            div {
                class: "flex flex-col items-center justify-center h-screen bg-slate-900 text-white font-sans",
                h1 { class: "text-4xl font-bold mb-8", "Resultado de Calidad" }
                
                div {
                    class: "bg-slate-800 p-8 rounded-xl shadow-2xl w-1/2 border border-slate-700",
                    table {
                        class: "w-full text-left text-xl",
                        tbody {
                            tr {
                                class: "border-b border-slate-700",
                                th { class: "py-4", "Variable" }
                                th { class: "py-4", "Valor" }
                                th { class: "py-4", "Estado" }
                            }
                            tr {
                                class: "border-b border-slate-700",
                                td { class: "py-4", "Pureza Final" }
                                td { class: "py-4 text-emerald-400", "99.99%" }
                                td { class: "py-4", if is_defective { "⚠️ Comprometida" } else { "✅ Excelente" } }
                            }
                            tr {
                                class: "border-b border-slate-700",
                                td { class: "py-4", "Densidad de Pieza" }
                                td { class: "py-4 text-emerald-400", "98.2%" }
                                td { class: "py-4", if is_defective { "⚠️ Porosidad detectada" } else { "✅ Óptimo" } }
                            }
                            tr {
                                td { class: "py-4", "Tiempo Total" }
                                td { class: "py-4 text-emerald-400", "14.5s" }
                                td { class: "py-4", if is_defective { "🛑 Fallo Crítico" } else { "⏱️ Óptimo" } }
                            }
                        }
                    }
                }
                
                if is_defective {
                    div { class: "mt-8 text-3xl font-bold text-red-500 animate-pulse", "PIEZA DEFECTUOSA" }
                } else {
                    div { class: "mt-8 text-3xl font-bold text-emerald-500", "PROCESO EXITOSO" }
                }
                
                button {
                    class: "mt-12 px-6 py-3 bg-blue-600 hover:bg-blue-500 rounded text-xl font-bold transition",
                    onclick: move |_| {
                        is_finished.set(false);
                        is_panic.set(false);
                        o2_flow.set(50);
                        voltage.set(50);
                        laser_power.set(50);
                        gas_level.set(100.0);
                    },
                    "Reiniciar Simulación"
                }
            }
        };
    }

    rsx! {
        div {
            class: "flex h-screen bg-[#0f172a] text-slate-200 font-sans overflow-hidden",
            
            // 1. Sidebar (Progress)
            div {
                class: "w-1/4 bg-slate-900 border-r border-slate-800 flex flex-col p-6 shadow-xl z-10",
                h2 { class: "text-2xl font-bold mb-8 text-blue-400 tracking-wider", "CICLO DEL COBRE" }
                
                div { class: "flex flex-col gap-4",
                    StageButton { stage: Stage::Fusion, current: *selected_stage.read(), set_stage: selected_stage }
                    StageButton { stage: Stage::Conversion, current: *selected_stage.read(), set_stage: selected_stage }
                    StageButton { stage: Stage::Refining, current: *selected_stage.read(), set_stage: selected_stage }
                    StageButton { stage: Stage::Atomization, current: *selected_stage.read(), set_stage: selected_stage }
                    StageButton { stage: Stage::Printing, current: *selected_stage.read(), set_stage: selected_stage }
                }

                div { class: "mt-auto",
                    button {
                        class: "w-full py-4 bg-red-600 hover:bg-red-500 text-white font-bold rounded-lg uppercase tracking-widest shadow-lg shadow-red-900/50 transition transform hover:scale-105 active:scale-95",
                        onclick: move |_| is_panic.set(true),
                        "⚠️ ERROR EN HORNO"
                    }
                }
            }
            
            // 2. Main Central Display
            div {
                class: "w-2/4 flex flex-col p-8 bg-slate-900/50 relative",
                
                if *is_panic.read() {
                    div {
                        class: "absolute inset-0 bg-red-900/40 z-0 animate-pulse pointer-events-none"
                    }
                    div {
                        class: "absolute top-4 left-1/2 -translate-x-1/2 bg-red-600 text-white px-6 py-2 rounded-full font-bold z-10 animate-fade-in animate-bounce",
                        "¡ALERTA DE SISTEMA! PARADA DE EMERGENCIA"
                    }
                }
                
                div { class: "z-10",
                    h1 { class: "text-3xl font-light mb-2", "{selected_stage.read().to_string()}" }
                    p { class: "text-slate-400 mb-8", "Monitorización en Tiempo Real (1ms tick)" }
                    
                    div { class: "grid grid-cols-2 gap-8 mb-8",
                        // Thermometer
                        div { class: "bg-slate-800 p-6 rounded-2xl border border-slate-700/50 shadow-inner flex flex-col items-center",
                            h3 { class: "text-gray-400 mb-4 text-sm font-semibold tracking-wider", "TEMPERATURA" }
                            div { class: "text-5xl font-mono text-orange-400", "{(*temp.read() as i32)}°C" }
                            div { class: "w-full h-4 bg-slate-900 rounded-full mt-6 overflow-hidden relative",
                                div { 
                                    class: "h-full bg-gradient-to-r from-orange-600 to-yellow-400 transition-all duration-300",
                                    style: "width: {((*temp.read() / 2000.0) * 100.0).clamp(0.0, 100.0)}%;"
                                }
                            }
                            if is_o2_low {
                                div { class: "mt-4 text-red-500 font-bold text-sm animate-pulse", "ALERTA: REACCIÓN DETENIDA" }
                            }
                        }
                        
                        // Gas Tanks
                        div { class: "bg-slate-800 p-6 rounded-2xl border border-slate-700/50 shadow-inner flex flex-col items-center",
                            h3 { class: "text-gray-400 mb-4 text-sm font-semibold tracking-wider", "NIVEL DE GAS" }
                            div { class: "text-5xl font-mono text-blue-400", "{(*gas_level.read() as i32)}%" }
                            div { class: "w-full h-4 bg-slate-900 rounded-full mt-6 overflow-hidden relative",
                                div { 
                                    class: "h-full bg-blue-500 transition-all duration-300",
                                    style: "width: {*gas_level.read()}%;"
                                }
                            }
                        }
                    }

                    // Chart Placeholder
                    div { class: "bg-slate-800 flex-1 p-6 rounded-2xl border border-slate-700/50 shadow-inner relative overflow-hidden flex flex-col",
                        h3 { class: "text-gray-400 mb-4 text-sm font-semibold tracking-wider", "CINÉTICA DE REACCIÓN" }
                        div { class: "flex items-end flex-1 gap-1",
                            for i in 0..40 {
                                div {
                                    class: "bg-emerald-500/50 w-full flex-1 rounded-t opacity-80",
                                    style: "height: {20.0 + rand::random::<f32>() * 80.0}%; transition: height 0.5s;"
                                }
                            }
                        }
                    }
                }
            }
            
            // 3. Control Console
            div {
                class: "w-1/4 bg-slate-800 border-l border-slate-700 p-6 flex flex-col z-10",
                h2 { class: "text-xl font-bold mb-8 text-slate-300 tracking-wider", "CONSOLA DE CONTROL" }
                
                div { class: "flex flex-col gap-10 flex-1",
                    ConsoleSlider { name: "Flujo de O2", val: o2_flow, min: 0, max: 100, unit: "%" }
                    ConsoleSlider { name: "Voltaje Electrolítico", val: voltage, min: 0, max: 100, unit: "V" }
                    if is_v_high {
                        div { class: "-mt-6 text-red-500 font-bold text-xs animate-pulse", "IMPUREZAS DETECTADAS" }
                    }
                    ConsoleSlider { name: "Potencia Láser", val: laser_power, min: 0, max: 100, unit: "W" }
                }
                
                button {
                    class: "w-full py-5 bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-500 hover:to-indigo-500 text-white font-bold rounded-xl text-lg shadow-xl shadow-blue-900/30 transition transform hover:-translate-y-1 active:translate-y-0",
                    onclick: move |_| is_finished.set(true),
                    "IMPRIMIR PIEZA FINAL"
                }
            }
        }
    }
}

#[component]
fn StageButton(stage: Stage, current: Stage, set_stage: Signal<Stage>) -> Element {
    let is_active = stage == current;
    let bg = if is_active { "bg-blue-900/40 border-l-4 border-blue-500" } else { "hover:bg-slate-800 border-l-4 border-transparent" };
    let text = if is_active { "text-white font-semibold" } else { "text-slate-400" };
    let icon = if is_active { "⚙️" } else { "✓" };
    let anim = if is_active { "animate-spin" } else { "" };
    
    rsx! {
        button {
            class: "flex items-center gap-4 w-full p-4 rounded-r-lg transition-all duration-200 text-left {bg} {text}",
            onclick: move |_| set_stage.set(stage),
            span { class: "text-xl {anim}", "{icon}" }
            span { class: "tracking-wide", "{stage.to_string()}" }
        }
    }
}

#[component]
fn ConsoleSlider(name: &'static str, val: Signal<i32>, min: i32, max: i32, unit: &'static str) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2",
            div { class: "flex justify-between items-end",
                label { class: "text-sm text-slate-400 tracking-wider font-semibold", "{name}" }
                span { class: "font-mono text-blue-400 font-bold", "{(*val.read())}{unit}" }
            }
            input {
                r#type: "range",
                min: "{min}",
                max: "{max}",
                value: "{*val.read()}",
                oninput: move |e| {
                    if let Ok(v) = e.value().parse::<i32>() {
                        val.set(v);
                    }
                },
                class: "w-full h-2 bg-slate-700 rounded-lg appearance-none cursor-pointer accent-blue-500"
            }
        }
    }
}
