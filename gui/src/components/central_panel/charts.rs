#![allow(dead_code)]
use dioxus::prelude::*;

// ========================================================
// 1. HORNO FLASH (Fusión Autógena)
// ========================================================
#[derive(PartialEq, Props, Clone)]
pub struct FlashFurnaceProps {
    pub o2_flow: f64,
    pub silica: f64,
}

#[component]
pub fn FlashFurnaceCharts(props: FlashFurnaceProps) -> Element {
    let path_y = 90.0 - (props.o2_flow * 0.7);
    let silica_height = props.silica.max(10.0);
    
    rsx! {
        div { class: "flex flex-row gap-4 w-full h-full",
            div { class: "w-1/2 bg-white/80 rounded-xl p-4 border border-slate-300 flex flex-col relative shadow-sm",
                h3 { class: "text-slate-600 font-bold mb-2 text-xs tracking-widest", "TEMPERATURA: INTERNA VS CARCASA" }
                svg { class: "flex-1 w-full h-full transition-all duration-300", view_box: "0 0 100 100", preserve_aspect_ratio: "none",
                    path { d: "M 0 80 Q 25 50 50 {path_y} T 100 20", fill: "none", stroke: "#fb923c", stroke_width: "2" }
                    path { d: "M 0 90 Q 20 80 50 85 T 100 70", fill: "none", stroke: "#94a3b8", stroke_width: "2", stroke_dasharray: "4" }
                }
            }
            div { class: "w-1/2 bg-white/80 rounded-xl p-4 border border-slate-300 flex flex-col shadow-sm",
                h3 { class: "text-slate-600 font-bold mb-2 text-xs tracking-widest", "NIVEL DE FASE: MATA VS ESCORIA" }
                svg { class: "flex-1 w-full h-full transition-all duration-300", view_box: "0 0 100 100", preserve_aspect_ratio: "none",
                    rect { x: "20", y: "0", width: "60", height: "100", fill: "#b45309" }
                    rect { x: "20", y: "{100.0 - silica_height}", width: "60", height: "{silica_height}", fill: "#52525b" }
                    text { x: "50", y: "{100.0 - silica_height / 2.0}", fill: "white", font_size: "10", text_anchor: "middle", "ESCORIA" }
                    text { x: "50", y: "25", fill: "white", font_size: "10", text_anchor: "middle", "MATA" }
                }
            }
        }
    }
}

// ========================================================
// 2. CONVERTIDOR (Peirce-Smith)
// ========================================================
#[derive(PartialEq, Props, Clone)]
pub struct ConverterProps {
    pub o2_flow: f64,
    pub scrap: f64,
}

#[component]
pub fn ConverterCharts(props: ConverterProps) -> Element {
    let point_y = 100.0 - props.scrap.max(10.0);
    let cu_purity = 80.0 + (props.o2_flow * 0.15);
    let rest = 100.0 - cu_purity;

    rsx! {
        div { class: "flex flex-row gap-4 w-full h-full",
            div { class: "w-1/2 bg-white/80 rounded-xl p-4 border border-slate-300 flex flex-col shadow-sm",
                h3 { class: "text-slate-600 font-bold mb-2 text-xs tracking-widest", "CINÉTICA TÉRMICA (CAÍDAS)" }
                svg { class: "flex-1 w-full h-full transition-all duration-500", view_box: "0 0 100 100", preserve_aspect_ratio: "none",
                    path { d: "M 0 50 L 20 {point_y} L 40 45 L 60 {point_y} L 80 80 L 100 20", fill: "none", stroke: "#ef4444", stroke_width: "2" }
                    circle { cx: "20", cy: "{point_y}", r: "3", fill: "red" }
                    circle { cx: "60", cy: "{point_y}", r: "3", fill: "red" }
                }
            }
            div { class: "w-1/2 bg-white/80 rounded-xl p-4 border border-slate-300 flex flex-col justify-center shadow-sm",
                h3 { class: "text-slate-600 font-bold mb-4 text-xs tracking-widest text-center", "COMPOSICIÓN ESTEQUIOMÉTRICA (%)" }
                svg { class: "flex-1 w-full h-full transition-all duration-500", view_box: "0 0 100 100", preserve_aspect_ratio: "none",
                    rect { x: "0", y: "10", width: "{cu_purity}", height: "15", fill: "#fbbf24" }
                    text { x: "5", y: "21", fill: "#334155", font_size: "8", font_weight: "bold", "Cobre (Cu) {cu_purity as i32}%" }
                    rect { x: "0", y: "40", width: "{rest}", height: "15", fill: "#64748b" }
                }
            }
        }
    }
}

// ========================================================
// 3. AFINO TÉRMICO Y MOLDEO
// ========================================================
#[derive(PartialEq, Props, Clone)]
pub struct ThermalRefiningProps {
    pub red_gas: f64,
}

#[component]
pub fn ThermalRefiningCharts(props: ThermalRefiningProps) -> Element {
    let o2_angle = 180.0 - (props.red_gas * 1.8);
    let h2_angle = props.red_gas * 1.8;
    
    rsx! {
        div { class: "flex flex-row gap-4 w-full h-full",
            div { class: "w-1/2 bg-white/80 rounded-xl p-4 border border-slate-300 flex flex-col items-center justify-center relative shadow-sm",
                h3 { class: "absolute top-4 left-4 text-slate-600 font-bold text-xs tracking-widest", "NIVEL O2 RESIDUAL" }
                svg { class: "w-full max-h-[80%]", view_box: "0 0 100 50", preserve_aspect_ratio: "xMidYMax meet",
                    path { d: "M 10 40 A 30 30 0 0 1 90 40", fill: "none", stroke: "#e2e8f0", stroke_width: "10" }
                    line { x1: "50", y1: "40", x2: "20", y2: "40", stroke: "#3b82f6", stroke_width: "3", transform: "rotate({o2_angle} 50 40)", class: "transition-transform duration-300" }
                    circle { cx: "50", cy: "40", r: "5", fill: "#334155" }
                }
            }
            div { class: "w-1/2 bg-white/80 rounded-xl p-4 border border-slate-300 flex flex-col items-center justify-center relative shadow-sm",
                h3 { class: "absolute top-4 left-4 text-slate-600 font-bold text-xs tracking-widest", "H2 ABSORBIDO" }
                svg { class: "w-full max-h-[80%]", view_box: "0 0 100 50", preserve_aspect_ratio: "xMidYMax meet",
                    path { d: "M 10 40 A 30 30 0 0 1 90 40", fill: "none", stroke: "#e2e8f0", stroke_width: "10" }
                    line { x1: "50", y1: "40", x2: "20", y2: "40", stroke: "#ef4444", stroke_width: "3", transform: "rotate({h2_angle} 50 40)", class: "transition-transform duration-300" }
                    circle { cx: "50", cy: "40", r: "5", fill: "#334155" }
                }
            }
        }
    }
}

// ========================================================
// 4. REFINO ELECTROLÍTICO
// ========================================================
#[derive(PartialEq, Props, Clone)]
pub struct ElectrolysisProps {
    pub amps: f64,
}

#[component]
pub fn ElectrolysisCharts(props: ElectrolysisProps) -> Element {
    let lodo = props.amps; 
    let anode_h = 90.0 - (props.amps * 0.7);
    let catode_h = 10.0 + (props.amps * 0.8);

    rsx! {
        div { class: "flex flex-row gap-4 w-full h-full",
            div { class: "w-1/2 bg-white/80 rounded-xl p-4 border border-slate-300 flex flex-col justify-end shadow-sm",
                h3 { class: "text-slate-600 font-bold mb-2 text-xs tracking-widest", "ÁNODO VS CÁTODO (Masa)" }
                svg { class: "flex-1 w-full h-full transition-all duration-300", view_box: "0 0 100 100", preserve_aspect_ratio: "none",
                    rect { x: "20", y: "{100.0 - anode_h}", width: "20", height: "{anode_h}", fill: "#94a3b8" }
                    rect { x: "60", y: "{100.0 - catode_h}", width: "20", height: "{catode_h}", fill: "#fbbf24" }
                }
            }
            div { class: "w-1/2 bg-white/80 rounded-xl p-4 border border-slate-300 flex flex-col shadow-sm",
                h3 { class: "text-slate-600 font-bold mb-2 text-xs tracking-widest", "ACUMULACIÓN LODOS (Ag/Au)" }
                svg { class: "flex-1 w-full h-full transition-all duration-300", view_box: "0 0 100 100", preserve_aspect_ratio: "none",
                    polygon { points: "0,100 0,{100.0 - lodo*0.5} 50,{100.0 - lodo} 100,{100.0 - lodo*0.3} 100,100", fill: "#cbd5e1" }
                }
            }
        }
    }
}

// ========================================================
// 5. ATOMIZACIÓN POR GAS
// ========================================================
#[derive(PartialEq, Props, Clone)]
pub struct AtomizationProps {
    pub pressure: f64,
}

#[component]
pub fn AtomizationCharts(props: AtomizationProps) -> Element {
    let peak_x = 100.0 - props.pressure;
    let angle = props.pressure * 1.8;

    rsx! {
        div { class: "flex flex-row gap-4 w-full h-full",
            div { class: "w-1/2 bg-white/80 rounded-xl p-4 border border-slate-300 flex flex-col justify-end shadow-sm",
                h3 { class: "text-slate-600 font-bold mb-2 text-xs tracking-widest", "HISTOGRAMA PARTÍCULAS (Gauss)" }
                svg { class: "flex-1 w-full h-full transition-all duration-300", view_box: "0 0 100 100", preserve_aspect_ratio: "none",
                    path { d: "M 0 100 Q {peak_x - 15.0} 100 {peak_x} 20 T {peak_x + 30.0} 100", fill: "#818cf8", opacity: "0.5" }
                    line { x1: "30", y1: "0", x2: "30", y2: "100", stroke: "#ef4444", stroke_dasharray: "2" }
                }
            }
            div { class: "w-1/2 bg-white/80 rounded-xl p-4 border border-slate-300 flex flex-col items-center justify-center relative shadow-sm",
                h3 { class: "absolute top-4 left-4 text-slate-600 font-bold mb-2 text-xs tracking-widest", "P. ARGÓN (PSI)" }
                svg { class: "w-full max-h-[80%]", view_box: "0 0 100 50", preserve_aspect_ratio: "xMidYMax meet",
                    path { d: "M 10 40 A 30 30 0 0 1 90 40", fill: "none", stroke: "#e2e8f0", stroke_width: "6" }
                    line { x1: "50", y1: "40", x2: "20", y2: "40", stroke: "#eab308", stroke_width: "3", transform: "rotate({angle} 50 40)", class: "transition-transform duration-300" }
                }
            }
        }
    }
}

// ========================================================
// 6. IMPRESIÓN 3D (LMD / PBF)
// ========================================================
#[derive(PartialEq, Props, Clone)]
pub struct AdditiveManufacturingProps {
    pub laser: f64,
}

#[component]
pub fn AdditiveManufacturingCharts(props: AdditiveManufacturingProps) -> Element {
    let laser_intensity = props.laser;
    let laser_color = if laser_intensity > 70.0 { "#ef4444" } else if laser_intensity < 30.0 { "#60a5fa" } else { "#fef08a" };
    let path_y = 100.0 - laser_intensity;

    rsx! {
        div { class: "flex flex-row gap-4 w-full h-full",
            div { class: "w-1/2 bg-white/80 rounded-xl p-4 border border-slate-300 flex flex-col shadow-sm",
                h3 { class: "text-slate-600 font-bold mb-2 text-xs tracking-widest", "HEATMAP LÁSER 2D" }
                svg { class: "flex-1 w-full h-full transition-colors duration-300", view_box: "0 0 100 100", preserve_aspect_ratio: "none",
                    rect { x: "0", y: "0", width: "33", height: "33", fill: "#f97316" }
                    rect { x: "33", y: "0", width: "33", height: "33", fill: "#f97316" }
                    rect { x: "66", y: "0", width: "34", height: "33", fill: "#3b82f6" }
                    
                    rect { x: "0", y: "33", width: "33", height: "33", fill: "#fef08a" }
                    rect { x: "33", y: "33", width: "33", height: "33", fill: "{laser_color}" } 
                    rect { x: "66", y: "33", width: "34", height: "33", fill: "#1e3a8a" }
                    
                    rect { x: "0", y: "66", width: "33", height: "34", fill: "#3b82f6" }
                    rect { x: "33", y: "66", width: "33", height: "34", fill: "#1e3a8a" }
                    rect { x: "66", y: "66", width: "34", height: "34", fill: "#0f172a" }
                } 
            }
            div { class: "w-1/2 bg-white/80 rounded-xl p-4 border border-slate-300 flex flex-col shadow-sm",
                h3 { class: "text-slate-600 font-bold mb-2 text-xs tracking-widest", "VED(t) vs LÍNEA OBJETIVO" }
                svg { class: "flex-1 w-full h-full transition-all duration-300", view_box: "0 0 100 100", preserve_aspect_ratio: "none",
                    line { x1: "0", y1: "50", x2: "100", y2: "50", stroke: "#22c55e", stroke_width: "2", stroke_dasharray: "4" }
                    path { d: "M 0 50 Q 50 {path_y} 100 50", fill: "none", stroke: "#ec4899", stroke_width: "2" }
                }
            }
        }
    }
}
