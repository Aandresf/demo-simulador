use simulador_core::models::Stage;

pub fn get_chart_color(stage: Stage) -> &'static str {
    match stage {
        Stage::Fusion => "bg-orange-400",
        Stage::Conversion => "bg-amber-500",
        Stage::Refining => "bg-emerald-400",
        Stage::Electrolysis => "bg-blue-500",
        Stage::Atomization => "bg-purple-400",
        Stage::Printing => "bg-rose-400",
    }
}

pub fn get_card_styles(color: &str) -> (&'static str, &'static str) {
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
        "orange" => "text-orange-400",
        "blue" => "text-blue-400",
        "amber" => "text-amber-400",
        "emerald" => "text-emerald-400",
        "purple" => "text-purple-400",
        "indigo" => "text-indigo-400",
        "rose" => "text-rose-400",
        _ => "text-gray-400"
    };
    (bg_color, text_color)
}
