#![allow(non_snake_case)]
mod models;
mod components;
mod physics;

use dioxus::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

use models::{Stage, SimState};
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
    
    #[allow(unused_mut)]
    let mut o2_flow = use_signal(|| 50); 
    #[allow(unused_mut)]
    let mut scrap_rate = use_signal(|| 20); 
    #[allow(unused_mut)]
    let mut voltage = use_signal(|| 50); 
    #[allow(unused_mut)]
    let mut atomization_gas = use_signal(|| 80); 
    #[allow(unused_mut)]
    let mut laser_power = use_signal(|| 50); 
    
    let mut is_panic = use_signal(|| false);
    let mut is_finished = use_signal(|| false);

    let state = SimState {
        selected_stage,
        temp,
        gas_level,
        o2_flow,
        scrap_rate,
        voltage,
        atomization_gas,
        laser_power,
        is_panic,
        is_finished,
    };

    // Animations / Ticks calling Physics Engine
    use_future(move || async move {
        loop {
            sleep(Duration::from_millis(1000)).await;
            if !*is_panic.read() && !*is_finished.read() {
                let stage = *selected_stage.read();
                
                let new_temp = match stage {
                    Stage::Fusion => physics::calculate_fusion_temp(1200.0, *o2_flow.read()),
                    Stage::Conversion => physics::calculate_conversion_temp(1100.0, *scrap_rate.read()),
                    Stage::Refining => physics::calculate_electrolytic_temp(*voltage.read()),
                    Stage::Atomization => 400.0,
                    Stage::Printing => physics::calculate_laser_temp(*laser_power.read()),
                };
                
                temp.set(new_temp);
                
                let current_gas = *gas_level.read();
                if current_gas > 0.0 {
                    gas_level.set(current_gas - 0.5);
                }
            }
        }
    });

    if *is_finished.read() {
        let is_defective = *is_panic.read() || *o2_flow.read() < 20 || *voltage.read() > 80 || *laser_power.read() < 30;
        return rsx! {
            div {
                class: "flex flex-col items-center justify-center h-screen bg-[#0f172a] text-slate-200 font-sans",
                h1 { class: "text-4xl font-bold mb-8 text-blue-400 tracking-wider", "Resultado de Calidad" }
                
                div {
                    class: "bg-slate-800 p-8 rounded-xl shadow-2xl w-1/2 border border-slate-700",
                    table {
                        class: "w-full text-left text-xl",
                        tbody {
                            tr {
                                class: "border-b border-slate-700",
                                th { class: "py-4 text-slate-300 font-bold", "Variable" }
                                th { class: "py-4 text-slate-300 font-bold", "Valor" }
                                th { class: "py-4 text-slate-300 font-bold", "Estado" }
                            }
                            tr {
                                class: "border-b border-slate-700 text-slate-400",
                                td { class: "py-4", "Pureza Final" }
                                td { class: "py-4 text-emerald-400 font-bold", "99.99%" }
                                td { class: "py-4", if is_defective { "⚠️ Comprometida" } else { "✅ Excelente" } }
                            }
                            tr {
                                class: "border-b border-slate-700 text-slate-400",
                                td { class: "py-4", "Densidad de Pieza" }
                                td { class: "py-4 text-emerald-400 font-bold", "98.2%" }
                                td { class: "py-4", if is_defective { "⚠️ Porosidad detectada" } else { "✅ Óptimo" } }
                            }
                            tr {
                                class: "text-slate-400",
                                td { class: "py-4", "Tiempo Total" }
                                td { class: "py-4 text-blue-400 font-bold", "14.5s" }
                                td { class: "py-4", if is_defective { "🛑 Fallo Crítico" } else { "⏱️ Óptimo" } }
                            }
                        }
                    }
                }
                
                if is_defective {
                    div { class: "mt-8 text-3xl font-bold text-red-500 animate-pulse tracking-widest", "PIEZA DEFECTUOSA" }
                } else {
                    div { class: "mt-8 text-3xl font-bold text-emerald-400 tracking-widest", "PROCESO EXITOSO" }
                }
                
                button {
                    class: "mt-12 px-6 py-3 bg-blue-600 hover:bg-blue-500 shadow-md text-white rounded-lg text-xl font-bold transition",
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
