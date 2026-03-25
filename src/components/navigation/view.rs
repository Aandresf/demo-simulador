use dioxus::prelude::*;
use crate::models::Stage;
use super::model::{NavigationProps, StageButtonProps};
use super::controller::get_button_styles;

#[component]
pub fn Sidebar(props: NavigationProps) -> Element {
    rsx! {
        div {
            class: "w-1/4 bg-slate-900 border-r border-slate-800 flex flex-col p-6 shadow-xl z-10",
            h2 { class: "text-2xl font-bold mb-8 text-blue-400 tracking-wider", "CICLO DEL COBRE" }
            
            div { class: "flex flex-col gap-4",
                StageButton { stage_val: Stage::Fusion, current: props.current_stage, onclick: move |s| props.on_stage_change.call(s) }
                StageButton { stage_val: Stage::Conversion, current: props.current_stage, onclick: move |s| props.on_stage_change.call(s) }
                StageButton { stage_val: Stage::Refining, current: props.current_stage, onclick: move |s| props.on_stage_change.call(s) }
                StageButton { stage_val: Stage::Atomization, current: props.current_stage, onclick: move |s| props.on_stage_change.call(s) }
                StageButton { stage_val: Stage::Printing, current: props.current_stage, onclick: move |s| props.on_stage_change.call(s) }
            }

            div { class: "mt-auto",
                button {
                    class: "w-full py-4 bg-red-600 hover:bg-red-500 text-white font-bold rounded-lg uppercase tracking-widest shadow-lg shadow-red-900/50 transition transform hover:scale-105 active:scale-95",
                    onclick: move |_| props.on_panic.call(()),
                    "⚠️ ERROR EN HORNO"
                }
            }
        }
    }
}

#[component]
fn StageButton(props: StageButtonProps) -> Element {
    let (bg, text, icon, anim) = get_button_styles(&props);
    let stage_val = props.stage_val;
    
    rsx! {
        button {
            class: "flex items-center gap-4 w-full p-4 rounded-r-lg transition-all duration-200 text-left {bg} {text}",
            onclick: move |_| props.onclick.call(stage_val),
            span { class: "text-xl {anim}", "{icon}" }
            span { class: "tracking-wide", "{stage_val.to_string()}" }
        }
    }
}
