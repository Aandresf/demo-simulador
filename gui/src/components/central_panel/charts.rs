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
                svg { class: "flex-1 w-full h-full",
                    // TODO: <path d="..." stroke="green" /> nativo dinámico de temp_history
                }
            }
            // Gráfico Derecho: Áreas apiladas
            div { class: "w-1/2 bg-slate-800/80 rounded-xl p-4 border border-slate-700 flex flex-col",
                h3 { class: "text-slate-400 font-bold mb-2 text-xs tracking-widest", "NIVEL DE FASE: MATA VS ESCORIA" }
                svg { class: "flex-1 w-full h-full",
                    // TODO: <rect> con height interpolando Mata y Escoria levels
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
                svg { class: "flex-1 w-full h-full",
                    // TODO: Linechart con Puntos (Circles SVG) superpuestos en tick = drop_event
                }
            }
            div { class: "w-1/2 bg-slate-800/80 rounded-xl p-4 border border-slate-700 flex flex-col justify-center",
                h3 { class: "text-slate-400 font-bold mb-4 text-xs tracking-widest text-center", "COMPOSICIÓN ESTEQUIOMÉTRICA (%)" }
                svg { class: "flex-1 w-full h-full",
                    // TODO: 3 Barras horizontales SVG dinámicas calculando width basado en *_purity
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
                svg { class: "w-full max-h-[80%]",
                    // TODO: GAUGE SVG con <circle stroke-dasharray> y Aguja dinámica según residual_oxygen
                }
            }
            div { class: "w-1/2 bg-slate-800/80 rounded-xl p-4 border border-slate-700 flex flex-col items-center justify-center relative",
                h3 { class: "absolute top-4 left-4 text-slate-400 font-bold text-xs tracking-widest", "H2 ABSORBIDO (POROSIDAD)" }
                svg { class: "w-full max-h-[80%]",
                    // TODO: GAUGE SVG análogo para absorción de Hidrógeno
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
                svg { class: "flex-1 w-full h-full",
                    // TODO: 2 Barras verticales opuestas `<rect>` regenerándose por height (Ánodo baja, Cátodo sube)
                }
            }
            div { class: "w-1/2 bg-slate-800/80 rounded-xl p-4 border border-slate-700 flex flex-col",
                h3 { class: "text-slate-400 font-bold mb-2 text-xs tracking-widest", "ACUMULACIÓN DE LODOS E IMPUREZAS" }
                svg { class: "flex-1 w-full h-full",
                    // TODO: Gráfico de áreas rellenas `<polygon>` mostrando historial de lodo depositado
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
                svg { class: "flex-1 w-full h-full",
                    // TODO: Histograma (Polígonos apilados) mostrando la traslación de la campana de Gauss
                }
            }
            div { class: "w-1/2 bg-slate-800/80 rounded-xl p-4 border border-slate-700 flex flex-col items-center justify-center relative",
                h3 { class: "absolute top-4 left-4 text-slate-400 font-bold mb-2 text-xs tracking-widest", "Dinámica P. ARGÓN (PSI)" }
                svg { class: "w-full max-h-[80%]",
                    // TODO: Indicador circular (Gauge) marcando la presión viva a intervalos bruscos
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
                svg { class: "flex-1 w-full h-full",
                    // TODO: Renderizar Grid de `<rect>` iterados. Interpolando relleno de Azul a Blanco brillante (Fusión)
                } 
            }
            div { class: "w-1/2 bg-slate-800/80 rounded-xl p-4 border border-slate-700 flex flex-col",
                h3 { class: "text-slate-400 font-bold mb-2 text-xs tracking-widest", "VED(t) vs LÍNEA OBJETIVO" }
                svg { class: "flex-1 w-full h-full",
                    // TODO: Multi-Línea mostrando el path activo real vs el path horizontal Target.
                }
            }
        }
    }
}
