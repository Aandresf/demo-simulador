use dioxus::prelude::*;
use crate::state::SimState;

#[derive(Clone, PartialEq, Props)]
pub struct ControlPanelProps {
    pub state: SimState,
}

#[derive(Clone, PartialEq, Props)]
pub struct ConsoleSliderProps {
    pub name: &'static str,
    pub val: Signal<i32>,
    pub min: i32,
    pub max: i32,
    pub unit: &'static str,
}
