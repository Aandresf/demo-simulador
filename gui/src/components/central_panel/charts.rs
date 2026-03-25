#![allow(dead_code)]
use dioxus::prelude::*;

// ========================================================
// 1. HORNO FLASH (Fusión Autógena)
// ========================================================
#[derive(PartialEq, Props, Clone)]
pub struct FlashFurnaceProps {
    /// Array de (Tick, TempInterna, TempCarcasa)
    pub temp_history: Signal<Vec<(i32, f64, f64)>>,
    /// Porcentajes de volumen (0.0 a 100.0)
    pub mata_vol: Signal<f64>,
    pub escoria_vol: Signal<f64>,
}

#[component]
pub fn FlashFurnaceCharts(props: FlashFurnaceProps) -> Element {
    rsx! {
        div { class: "flex flex-row gap-4 w-full h-full",
            // Gráfico Izquierdo: Líneas de Temperatura
            div { class: "w-1/2 bg-slate-800/80 rounded-xl p-4 border border-slate-700 flex flex-col relative",
                h3 { class: "text-slate-400 font-bold mb-2 text-xs tracking-widest", "TEMPERATURA: INTERNA VS CARCASA" }
                svg { class: "flex-1 w-full h-full", view_box: "0 0 100 100", preserve_aspect_ratio: "none",
                    // SVGMock: Path con gradiente subiendo
                    path { d: "M 0 80 Q 25 50 50 70 T 100 20", fill: "none", stroke: "#fb923c", stroke_width: "2" }
                    path { d: "M 0 90 Q 20 80 50 85 T 100 70", fill: "none", stroke: "#94a3b8", stroke_width: "2", stroke_dasharray: "4" }
                }
            }
            // Gráfico Derecho: Áreas apiladas
            div { class: "w-1/2 bg-slate-800/80 rounded-xl p-4 border border-slate-700 flex flex-col",
                h3 { class: "text-slate-400 font-bold mb-2 text-xs tracking-widest", "NIVEL DE FASE: MATA VS ESCORIA" }
                svg { class: "flex-1 w-full h-full", view_box: "0 0 100 100", preserve_aspect_ratio: "none",
                    // SVGMock: Rectángulos apilados
                    rect { x: "20", y: "0", width: "60", height: "40", fill: "#52525b" } // Escoria
                    rect { x: "20", y: "40", width: "60", height: "60", fill: "#b45309" } // Mata (Fondo)
                    text { x: "50", y: "25", fill: "white", font_size: "10", text_anchor: "middle", "ESCORIA" }
                    text { x: "50", y: "75", fill: "white", font_size: "10", text_anchor: "middle", "MATA 62%" }
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
    pub temp_history: Signal<Vec<(i32, f64)>>,
    pub scrap_drop_events: Signal<Vec<i32>>, // Ticks donde cayó chatarra
    pub cu_purity: Signal<f64>, // % Cobre
    pub fe_purity: Signal<f64>, // % Hierro
    pub s_purity: Signal<f64>,  // % Azufre
}

#[component]
pub fn ConverterCharts(props: ConverterProps) -> Element {
    rsx! {
        div { class: "flex flex-row gap-4 w-full h-full",
            div { class: "w-1/2 bg-slate-800/80 rounded-xl p-4 border border-slate-700 flex flex-col",
                h3 { class: "text-slate-400 font-bold mb-2 text-xs tracking-widest", "CINÉTICA TÉRMICA (CAÍDAS POR CHATARRA)" }
                svg { class: "flex-1 w-full h-full", view_box: "0 0 100 100", preserve_aspect_ratio: "none",
                    // SVGMock
                    path { d: "M 0 50 L 20 40 L 40 45 L 60 20 L 80 80 L 100 20", fill: "none", stroke: "#ef4444", stroke_width: "2" }
                    circle { cx: "20", cy: "40", r: "3", fill: "red" }
                    circle { cx: "60", cy: "20", r: "3", fill: "red" }
                    text { x: "75", y: "90", fill: "#94a3b8", font_size: "8", "Chatarra Drop" }
                }
            }
            div { class: "w-1/2 bg-slate-800/80 rounded-xl p-4 border border-slate-700 flex flex-col justify-center",
                h3 { class: "text-slate-400 font-bold mb-4 text-xs tracking-widest text-center", "COMPOSICIÓN ESTEQUIOMÉTRICA (%)" }
                svg { class: "flex-1 w-full h-full", view_box: "0 0 100 100", preserve_aspect_ratio: "none",
                    // SVGMock
                    rect { x: "0", y: "10", width: "95", height: "15", fill: "#fbbf24" }
                    text { x: "5", y: "21", fill: "#000", font_size: "8", font_weight: "bold", "Cobre (Cu) 99%" }
                    
                    rect { x: "0", y: "40", width: "15", height: "15", fill: "#64748b" }
                    text { x: "5", y: "51", fill: "#fff", font_size: "8", "Hierro 1%" }
                    
                    rect { x: "0", y: "70", width: "5", height: "15", fill: "#fde047" }
                    text { x: "5", y: "81", fill: "#000", font_size: "8", "Azufre <1%" }
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
    pub residual_oxygen: Signal<f64>, // ppm
    pub absorbed_hydrogen: Signal<f64>, // ppm
}

#[component]
pub fn ThermalRefiningCharts(props: ThermalRefiningProps) -> Element {
    rsx! {
        div { class: "flex flex-row gap-4 w-full h-full",
            div { class: "w-1/2 bg-slate-800/80 rounded-xl p-4 border border-slate-700 flex flex-col items-center justify-center relative",
                h3 { class: "absolute top-4 left-4 text-slate-400 font-bold text-xs tracking-widest", "NIVEL DE O2 RESIDUAL" }
                svg { class: "w-full max-h-[80%]", view_box: "0 0 100 50", preserve_aspect_ratio: "xMidYMax meet",
                    // SVGMock: Half circle gauge
                    path { d: "M 10 40 A 30 30 0 0 1 90 40", fill: "none", stroke: "#334155", stroke_width: "10" }
                    path { d: "M 10 40 A 30 30 0 0 1 50 10", fill: "none", stroke: "#3b82f6", stroke_width: "10" }
                    line { x1: "50", y1: "40", x2: "20", y2: "20", stroke: "#fff", stroke_width: "2" }
                    circle { cx: "50", cy: "40", r: "4", fill: "#fff" }
                    text { x: "50", y: "48", fill: "#fff", font_size: "8", text_anchor: "middle", "200 ppm" }
                }
            }
            div { class: "w-1/2 bg-slate-800/80 rounded-xl p-4 border border-slate-700 flex flex-col items-center justify-center relative",
                h3 { class: "absolute top-4 left-4 text-slate-400 font-bold text-xs tracking-widest", "H2 ABSORBIDO (POROSIDAD)" }
                svg { class: "w-full max-h-[80%]", view_box: "0 0 100 50", preserve_aspect_ratio: "xMidYMax meet",
                    // SVGMock: Half circle gauge
                    path { d: "M 10 40 A 30 30 0 0 1 90 40", fill: "none", stroke: "#334155", stroke_width: "10" }
                    path { d: "M 70 18 A 30 30 0 0 1 90 40", fill: "none", stroke: "#ef4444", stroke_width: "10" } // Red zone
                    line { x1: "50", y1: "40", x2: "70", y2: "18", stroke: "#fff", stroke_width: "2" }
                    circle { cx: "50", cy: "40", r: "4", fill: "#fff" }
                    text { x: "50", y: "48", fill: "#fff", font_size: "8", text_anchor: "middle", "Peligro: Porosidad" }
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
    pub anode_mass: Signal<f64>,
    pub cathode_mass: Signal<f64>,
    pub impurity_history_ppm: Signal<Vec<(i32, f64)>>, // (Tick, PPM)
}

#[component]
pub fn ElectrolysisCharts(props: ElectrolysisProps) -> Element {
    rsx! {
        div { class: "flex flex-row gap-4 w-full h-full",
            div { class: "w-1/2 bg-slate-800/80 rounded-xl p-4 border border-slate-700 flex flex-col justify-end",
                h3 { class: "text-slate-400 font-bold mb-2 text-xs tracking-widest", "MIGRACIÓN: ÁNODO VS CÁTODO (Masa)" }
                svg { class: "flex-1 w-full h-full", view_box: "0 0 100 100", preserve_aspect_ratio: "none",
                    // SVGMock
                    rect { x: "20", y: "40", width: "20", height: "60", fill: "#64748b" }
                    text { x: "30", y: "90", fill: "#fff", font_size: "8", text_anchor: "middle", "ÁNODO" }
                    text { x: "30", y: "30", fill: "#fff", font_size: "8", text_anchor: "middle", "Baja (-)" }
                    
                    rect { x: "60", y: "10", width: "20", height: "90", fill: "#fbbf24" }
                    text { x: "70", y: "90", fill: "#000", font_size: "8", font_weight: "bold", text_anchor: "middle", "CÁTODO" }
                    text { x: "70", y: "8", fill: "#fbbf24", font_size: "8", text_anchor: "middle", "Sube (+)" }
                }
            }
            div { class: "w-1/2 bg-slate-800/80 rounded-xl p-4 border border-slate-700 flex flex-col",
                h3 { class: "text-slate-400 font-bold mb-2 text-xs tracking-widest", "ACUMULACIÓN DE LODOS E IMPUREZAS" }
                svg { class: "flex-1 w-full h-full", view_box: "0 0 100 100", preserve_aspect_ratio: "none",
                    // SVGMock: Area
                    polygon { points: "0,100 0,60 25,70 50,40 75,60 100,20 100,100", fill: "#1e293b" }
                    path { d: "M 0 60 L 25 70 L 50 40 L 75 60 L 100 20", fill: "none", stroke: "#cbd5e1", stroke_width: "2" }
                    text { x: "50", y: "90", fill: "#94a3b8", font_size: "10", text_anchor: "middle", "Ag / Au Lodos Generados" }
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
    pub particle_distribution: Signal<Vec<(f64, f64)>>, // (Tamaño, Frecuencia) para Campana Gauss
    pub gas_pressure: Signal<f64>, // PSI
}

#[component]
pub fn AtomizationCharts(props: AtomizationProps) -> Element {
    rsx! {
        div { class: "flex flex-row gap-4 w-full h-full",
            div { class: "w-1/2 bg-slate-800/80 rounded-xl p-4 border border-slate-700 flex flex-col justify-end",
                h3 { class: "text-slate-400 font-bold mb-2 text-xs tracking-widest", "HISTOGRAMA: PARTÍCULAS 15-45 μm" }
                svg { class: "flex-1 w-full h-full", view_box: "0 0 100 100", preserve_aspect_ratio: "none",
                    // SVGMock: Gauss Bell area
                    path { d: "M 0 100 Q 25 100 40 50 T 50 10 T 60 50 T 100 100", fill: "#4f46e5", opacity: "0.5" }
                    path { d: "M 0 100 Q 25 100 40 50 T 50 10 T 60 50 T 100 100", fill: "none", stroke: "#818cf8", stroke_width: "2" }
                    line { x1: "30", y1: "0", x2: "30", y2: "100", stroke: "#ef4444", stroke_dasharray: "2" }
                    text { x: "25", y: "10", fill: "#ef4444", font_size: "6", "15μm Límite" }
                }
            }
            div { class: "w-1/2 bg-slate-800/80 rounded-xl p-4 border border-slate-700 flex flex-col items-center justify-center relative",
                h3 { class: "absolute top-4 left-4 text-slate-400 font-bold mb-2 text-xs tracking-widest", "Dinámica P. ARGÓN (PSI)" }
                svg { class: "w-full max-h-[80%]", view_box: "0 0 100 50", preserve_aspect_ratio: "xMidYMax meet",
                    // SVGMock: Radial gauge for Pressure
                    path { d: "M 10 40 A 30 30 0 0 1 90 40", fill: "none", stroke: "#334155", stroke_width: "6" }
                    line { x1: "50", y1: "40", x2: "80", y2: "20", stroke: "#eab308", stroke_width: "3" }
                    circle { cx: "50", cy: "40", r: "5", fill: "#eab308" }
                    text { x: "50", y: "48", fill: "#fff", font_size: "6", text_anchor: "middle", "Presión Gas" }
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
    pub heat_matrix_2d: Signal<Vec<Vec<f64>>>, // Grid M x N con temperaturas
    pub ved_history: Signal<Vec<(i32, f64)>>, // (Tick, VED actual)
    pub ved_target: f64, // Línea Ideal estática
}

#[component]
pub fn AdditiveManufacturingCharts(props: AdditiveManufacturingProps) -> Element {
    rsx! {
        div { class: "flex flex-row gap-4 w-full h-full",
            div { class: "w-1/2 bg-slate-800/80 rounded-xl p-4 border border-slate-700 flex flex-col",
                h3 { class: "text-slate-400 font-bold mb-2 text-xs tracking-widest", "LECHO DE POLVO (HEATMAP 2D Láser)" }
                svg { class: "flex-1 w-full h-full", view_box: "0 0 100 100", preserve_aspect_ratio: "none",
                    // SVGMock: Rectangles matrix mock heat
                    rect { x: "0", y: "0", width: "33", height: "33", fill: "#ef4444" }
                    rect { x: "33", y: "0", width: "33", height: "33", fill: "#f97316" }
                    rect { x: "66", y: "0", width: "34", height: "33", fill: "#3b82f6" }
                    
                    rect { x: "0", y: "33", width: "33", height: "33", fill: "#fef08a" }
                    rect { x: "33", y: "33", width: "33", height: "33", fill: "#ffffff" } // Laser hot
                    rect { x: "66", y: "33", width: "34", height: "33", fill: "#1e3a8a" }
                    
                    rect { x: "0", y: "66", width: "33", height: "34", fill: "#3b82f6" }
                    rect { x: "33", y: "66", width: "33", height: "34", fill: "#1e3a8a" }
                    rect { x: "66", y: "66", width: "34", height: "34", fill: "#0f172a" }
                } 
            }
            div { class: "w-1/2 bg-slate-800/80 rounded-xl p-4 border border-slate-700 flex flex-col",
                h3 { class: "text-slate-400 font-bold mb-2 text-xs tracking-widest", "VED(t) vs LÍNEA OBJETIVO" }
                svg { class: "flex-1 w-full h-full", view_box: "0 0 100 100", preserve_aspect_ratio: "none",
                    // SVGMock: Ideal steady line vs Actual wavy line
                    line { x1: "0", y1: "50", x2: "100", y2: "50", stroke: "#22c55e", stroke_width: "2", stroke_dasharray: "4" }
                    path { d: "M 0 50 Q 20 20 40 60 T 80 50 T 100 30", fill: "none", stroke: "#ec4899", stroke_width: "2" }
                    text { x: "10", y: "45", fill: "#22c55e", font_size: "8", "VED Ideal" }
                }
            }
        }
    }
}
