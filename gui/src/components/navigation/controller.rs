// Import Stage removed to clear unused warning
use super::model::StageButtonProps;

pub fn get_button_styles(props: &StageButtonProps) -> (&'static str, &'static str, &'static str, &'static str) {
    let is_active = props.stage_val == props.current;
    
    // Dark mode styles as requested
    let bg = if is_active { "bg-blue-50 border-l-4 border-blue-500" } else { "hover:bg-slate-50 border-l-4 border-transparent" };
    let text = if is_active { "text-blue-700 font-bold" } else { "text-slate-600" };
    let icon = if is_active { "⚙️" } else { "✓" };
    let anim = if is_active { "animate-spin" } else { "" };
    
    (bg, text, icon, anim)
}
