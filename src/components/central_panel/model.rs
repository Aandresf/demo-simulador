use dioxus::prelude::*;
use crate::models::SimState;

#[derive(Clone, PartialEq, Props)]
pub struct CentralDisplayProps {
    pub state: SimState,
}

#[derive(Clone, PartialEq, Props)]
pub struct CentralCardProps {
    pub title: &'static str,
    pub val: String,
    pub max: f64,
    pub current_f: f64,
    pub color: &'static str,
}
