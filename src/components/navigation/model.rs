use dioxus::prelude::*;
use crate::models::Stage;

#[derive(Clone, PartialEq, Props)]
pub struct NavigationProps {
    pub current_stage: Stage,
    pub on_stage_change: EventHandler<Stage>,
    pub on_panic: EventHandler<()>,
}

#[derive(Clone, PartialEq, Props)]
pub struct StageButtonProps {
    pub stage_val: Stage,
    pub current: Stage,
    pub onclick: EventHandler<Stage>,
}
