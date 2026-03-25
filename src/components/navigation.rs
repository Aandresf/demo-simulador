use dioxus::prelude::*;
use crate::models::{Stage, SimState};

#[component]
pub fn Sidebar(state: SimState) -> Element {
    let stage = *state.selected_stage.read();
    
    rsx! {
        div {
            class: "w-1/4 bg-white border-r border-gray-200 flex flex-col p-6 shadow-lg z-10",
            h2 { class: "text-2xl font-black mb-8 text-blue-800 tracking-wider", "CICLO DEL COBRE" }
            
            div { class: "flex flex-col gap-4",
                StageButton { stage_val: Stage::Fusion, current: stage, set_stage: state.selected_stage }
                StageButton { stage_val: Stage::Conversion, current: stage, set_stage: state.selected_stage }
                StageButton { stage_val: Stage::Refining, current: stage, set_stage: state.selected_stage }
                StageButton { stage_val: Stage::Atomization, current: stage, set_stage: state.selected_stage }
                StageButton { stage_val: Stage::Printing, current: stage, set_stage: state.selected_stage }
            }

            div { class: "mt-auto",
                button {
                    class: "w-full py-4 bg-red-600 hover:bg-red-700 text-white font-bold rounded-lg uppercase tracking-widest shadow-lg shadow-red-200 transition transform hover:scale-105 active:scale-95",
                    onclick: move |_| {
                        let mut p = state.is_panic;
                        p.set(true);
                    },
                    "⚠️ ERROR EN HORNO"
                }
            }
        }
    }
}

#[component]
fn StageButton(stage_val: Stage, current: Stage, mut set_stage: Signal<Stage>) -> Element {
    let is_active = stage_val == current;
    let bg = if is_active { "bg-blue-50 border-l-4 border-blue-600" } else { "hover:bg-gray-50 border-l-4 border-transparent" };
    let text = if is_active { "text-blue-800 font-bold" } else { "text-gray-500 font-medium" };
    let icon = if is_active { "⚙️" } else { "✓" };
    let anim = if is_active { "animate-spin" } else { "" };
    
    rsx! {
        button {
            class: "flex items-center gap-4 w-full p-4 rounded-r-lg transition-all duration-200 text-left {bg} {text}",
            onclick: move |_| set_stage.set(stage_val),
            span { class: "text-xl {anim}", "{icon}" }
            span { class: "tracking-wide", "{stage_val.to_string()}" }
        }
    }
}
