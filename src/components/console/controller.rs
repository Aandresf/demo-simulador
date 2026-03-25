use dioxus::prelude::*;

pub fn on_slider_input(mut val: Signal<i32>, value: String) {
    if let Ok(v) = value.parse::<i32>() {
        val.set(v);
    }
}

pub fn finish_simulation(mut is_finished: Signal<bool>) {
    is_finished.set(true);
}
