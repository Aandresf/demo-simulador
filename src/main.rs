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
    let mut selected_stage = use_signal(|| Stage::Fusion);
    
    // Global simulation state
    let mut temp = use_signal(|| 1200.0);
    let mut gas_level = use_signal(|| 100.0);
    
    // Stage-specific controls
    #[allow(unused_mut)]
    let mut o2_flow = use_signal(|| 50); // Fusion & Conversion
    #[allow(unused_mut)]
    let mut scrap_rate = use_signal(|| 20); // Conversion
    #[allow(unused_mut)]
    let mut voltage = use_signal(|| 50); // Refining
    #[allow(unused_mut)]
    let mut atomization_gas = use_signal(|| 80); // Atomization
    #[allow(unused_mut)]
    let mut laser_power = use_signal(|| 50); // Printing
    
    let mut is_panic = use_signal(|| false);
    let mut is_finished = use_signal(|| false);

    // Animations / Ticks
    use_future(move || async move {
        loop {
            sleep(Duration::from_millis(1000)).await;
            if !*is_panic.read() && !*is_finished.read() {
                // Determine temperature based on stage
                let stage = *selected_stage.read();
                let base_temp = match stage {
                    Stage::Fusion => 1200.0 + (*o2_flow.read() as f64 * 2.0),
                    Stage::Conversion => 1100.0 - (*scrap_rate.read() as f64 * 3.0),
                    Stage::Refining => 800.0 + (*voltage.read() as f64 * 0.5),
                    Stage::Atomization => 400.0,
                    Stage::Printing => 1500.0 + (*laser_power.read() as f64 * 10.0),
                };
                
                let new_temp = base_temp + (rand::random::<f64>() * 10.0 - 5.0);
                temp.set(new_temp);
                
                let current_gas = *gas_level.read();
                if current_gas > 0.0 {
                    gas_level.set(current_gas - 0.5);
                }
            }
        }
    });

    let o2_val = *o2_flow.read();
    let v_val = *voltage.read();
    let laser_val = *laser_power.read();

    if *is_finished.read() {
        // Result Screen
        let is_defective = *is_panic.read() || o2_val < 20 || v_val > 80 || laser_val < 30;
        return rsx! {
            div {
                class: "flex flex-col items-center justify-center h-screen bg-gray-50 text-gray-800 font-sans",
                h1 { class: "text-4xl font-bold mb-8", "Resultado de Calidad" }
                
                div {
                    class: "bg-white p-8 rounded-xl shadow-2xl w-1/2 border border-blue-100",
                    table {
                        class: "w-full text-left text-xl",
                        tbody {
                            tr {
                                class: "border-b border-gray-200",
                                th { class: "py-4 font-bold", "Variable" }
                                th { class: "py-4 font-bold", "Valor" }
                                th { class: "py-4 font-bold", "Estado" }
                            }
                            tr {
                                class: "border-b border-gray-200 text-gray-600",
                                td { class: "py-4", "Pureza Final" }
                                td { class: "py-4 text-emerald-600 font-bold", "99.99%" }
                                td { class: "py-4", if is_defective { "⚠️ Comprometida" } else { "✅ Excelente" } }
                            }
                            tr {
                                class: "border-b border-gray-200 text-gray-600",
                                td { class: "py-4", "Densidad de Pieza" }
                                td { class: "py-4 text-emerald-600 font-bold", "98.2%" }
                                td { class: "py-4", if is_defective { "⚠️ Porosidad detectada" } else { "✅ Óptimo" } }
                            }
                            tr {
                                class: "text-gray-600",
                                td { class: "py-4", "Tiempo Total" }
                                td { class: "py-4 text-blue-600 font-bold", "14.5s" }
                                td { class: "py-4", if is_defective { "🛑 Fallo Crítico" } else { "⏱️ Óptimo" } }
                            }
                        }
                    }
                }
                
                if is_defective {
                    div { class: "mt-8 text-3xl font-bold text-red-600 animate-pulse", "PIEZA DEFECTUOSA" }
                } else {
                    div { class: "mt-8 text-3xl font-bold text-emerald-600", "PROCESO EXITOSO" }
                }
                
                button {
                    class: "mt-12 px-6 py-3 bg-blue-600 hover:bg-blue-700 shadow-md text-white rounded-lg text-xl font-bold transition",
                    onclick: move |_| {
                        is_finished.set(false);
                        is_panic.set(false);
                        o2_flow.set(50);
                        voltage.set(50);
                        laser_power.set(50);
                        gas_level.set(100.0);
                        selected_stage.set(Stage::Fusion);
                    },
                    "Reiniciar Simulación"
                }
            }
        };
    }

    let stage = *selected_stage.read();
    let bg_color = "bg-gray-100";
    let chart_color = match stage {
        Stage::Fusion => "bg-orange-400",
        Stage::Conversion => "bg-amber-500",
        Stage::Refining => "bg-blue-400",
        Stage::Atomization => "bg-purple-400",
        Stage::Printing => "bg-rose-400",
    };
    
    rsx! {
        div {
            class: "flex h-screen {bg_color} text-gray-800 font-sans overflow-hidden",
            
            // 1. Sidebar (Progress)
            div {
                class: "w-1/4 bg-white border-r border-gray-200 flex flex-col p-6 shadow-lg z-10",
                h2 { class: "text-2xl font-black mb-8 text-blue-800 tracking-wider", "CICLO DEL COBRE" }
                
                div { class: "flex flex-col gap-4",
                    StageButton { stage: Stage::Fusion, current: stage, set_stage: selected_stage }
                    StageButton { stage: Stage::Conversion, current: stage, set_stage: selected_stage }
                    StageButton { stage: Stage::Refining, current: stage, set_stage: selected_stage }
                    StageButton { stage: Stage::Atomization, current: stage, set_stage: selected_stage }
                    StageButton { stage: Stage::Printing, current: stage, set_stage: selected_stage }
                }

                div { class: "mt-auto",
                    button {
                        class: "w-full py-4 bg-red-600 hover:bg-red-700 text-white font-bold rounded-lg uppercase tracking-widest shadow-lg shadow-red-200 transition transform hover:scale-105 active:scale-95",
                        onclick: move |_| is_panic.set(true),
                        "⚠️ ERROR EN HORNO"
                    }
                }
            }
            
            // 2. Main Central Display
            div {
                class: "w-2/4 flex flex-col p-8 bg-gray-50 relative overflow-y-auto",
                
                if *is_panic.read() {
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
                        // Dynamic Cards based on stage
                        match stage {
                            Stage::Fusion => rsx!{
                                CentralCard { title: "TEMPERATURA (HORNO)", val: format!("{}°C", *temp.read() as i32), max: 2000.0, current_f: *temp.read(), color: "orange" }
                                CentralCard { title: "OXÍGENO INYECTADO", val: format!("{}%", *o2_flow.read()), max: 100.0, current_f: *o2_flow.read() as f64, color: "blue" }
                            },
                            Stage::Conversion => rsx!{
                                CentralCard { title: "TEMP. DEL BAÑO", val: format!("{}°C", *temp.read() as i32), max: 1500.0, current_f: *temp.read(), color: "orange" }
                                CentralCard { title: "VISCOSIDAD ESCORIA", val: format!("{:.1} Pa·s", 5.0 + (1100.0 - *temp.read()) * 0.1), max: 50.0, current_f: 5.0 + (1100.0 - *temp.read()) * 0.1, color: "amber" }
                            },
                            Stage::Refining => rsx!{
                                CentralCard { title: "SOBREPOTENCIAL (η)", val: format!("{} mV", *voltage.read() * 5), max: 500.0, current_f: (*voltage.read() * 5) as f64, color: "blue" }
                                CentralCard { title: "MASA DEPOSITADA", val: format!("{:.2} kg", 100.0 - *gas_level.read()), max: 100.0, current_f: 100.0 - *gas_level.read(), color: "emerald" }
                            },
                            Stage::Atomization => rsx!{
                                CentralCard { title: "TAMAÑO PARTÍCULA (d_m)", val: format!("{} μm", 120 - *atomization_gas.read()), max: 200.0, current_f: (120 - *atomization_gas.read()) as f64, color: "purple" }
                                CentralCard { title: "PRESIÓN DE GAS", val: format!("{} PSI", *atomization_gas.read() * 2), max: 200.0, current_f: (*atomization_gas.read() * 2) as f64, color: "indigo" }
                            },
                            Stage::Printing => rsx!{
                                CentralCard { title: "DENSIDAD ENERGÉTICA (VED)", val: format!("{} J/mm³", *laser_power.read() * 2), max: 200.0, current_f: (*laser_power.read() * 2) as f64, color: "rose" }
                                CentralCard { title: "TEMPERATURA FUSIÓN", val: format!("{}°C", *temp.read() as i32), max: 3000.0, current_f: *temp.read(), color: "orange" }
                            }
                        }
                    }

                    // Dynamic Chart Placeholder
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
            
            // 3. Control Console
            div {
                class: "w-1/4 bg-white border-l border-gray-200 p-6 flex flex-col shadow-lg z-10",
                h2 { class: "text-xl font-black mb-8 text-gray-800 tracking-wider", "CONSOLA DE CONTROL" }
                
                div { class: "flex flex-col gap-10 flex-1",
                    match stage {
                        Stage::Fusion => rsx!{
                            ConsoleSlider { name: "Flujo de O2 (Arrhenius)", val: o2_flow, min: 0, max: 100, unit: "%" }
                            if o2_val < 30 {
                                div { class: "-mt-6 text-red-500 font-bold text-xs animate-pulse", "REACCIÓN EXTINTA" }
                            }
                        },
                        Stage::Conversion => rsx!{
                            ConsoleSlider { name: "Flujo de O2", val: o2_flow, min: 0, max: 100, unit: "%" }
                            ConsoleSlider { name: "Adición de Chatarra", val: scrap_rate, min: 0, max: 100, unit: " kg/s" }
                            if *temp.read() < 1000.0 {
                                div { class: "-mt-6 text-red-500 font-bold text-xs animate-pulse", "RIESGO DE CONGELACIÓN" }
                            }
                        },
                        Stage::Refining => rsx!{
                            ConsoleSlider { name: "Voltaje Electrolítico", val: voltage, min: 0, max: 100, unit: "V" }
                            if v_val > 80 {
                                div { class: "-mt-6 text-red-500 font-bold text-xs animate-pulse", "IMPUREZAS DETECTADAS" }
                            }
                        },
                        Stage::Atomization => rsx!{
                            ConsoleSlider { name: "Presión Argón", val: atomization_gas, min: 10, max: 100, unit: " PSI" }
                        },
                        Stage::Printing => rsx!{
                            ConsoleSlider { name: "Potencia Láser", val: laser_power, min: 0, max: 100, unit: "%" }
                        }
                    }
                }
                
                button {
                    class: "w-full py-5 bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-700 text-white font-bold rounded-xl text-lg shadow-xl shadow-blue-200 transition transform hover:-translate-y-1 active:translate-y-0 disabled:opacity-50",
                    onclick: move |_| is_finished.set(true),
                    "IMPRIMIR PIEZA FINAL"
                }
            }
        }
    }
}

#[component]
fn CentralCard(title: &'static str, val: String, max: f64, current_f: f64, color: &'static str) -> Element {
    let width_pct = (current_f / max * 100.0).clamp(0.0, 100.0);
    
    // Simplistic mapping for Tailwind colors because Dioxus classes prefer literals
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
fn StageButton(stage: Stage, current: Stage, set_stage: Signal<Stage>) -> Element {
    let is_active = stage == current;
    let bg = if is_active { "bg-blue-50 border-l-4 border-blue-600" } else { "hover:bg-gray-50 border-l-4 border-transparent" };
    let text = if is_active { "text-blue-800 font-bold" } else { "text-gray-500 font-medium" };
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
                label { class: "text-sm text-gray-500 tracking-wider font-bold", "{name}" }
                span { class: "font-mono text-blue-600 font-black", "{(*val.read())}{unit}" }
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
                class: "w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-blue-600"
            }
        }
    }
}
