#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Stage {
    Fusion,
    Conversion,
    Refining,
    Electrolysis,
    Atomization,
    Printing,
}

impl Stage {
    pub fn to_string(&self) -> &'static str {
        match self {
            Stage::Fusion => "Fusión Primaria",
            Stage::Conversion => "Conversión",
            Stage::Refining => "Afino Térmico",
            Stage::Electrolysis => "Refino Electrolítico",
            Stage::Atomization => "Atomización",
            Stage::Printing => "Impresión 3D",
        }
    }
    
    pub fn objective(&self) -> &'static str {
        match self {
            Stage::Fusion => "Objetivo: Transformar Concentrado (25% Cu) a Mata Líquida (62% Cu) separando la Escoria (FeSiO3).",
            Stage::Conversion => "Objetivo: Transformar Mata (62% Cu) a Cobre Blíster (99% Cu) inyectando O2 y refrigerando con chatarra.",
            Stage::Refining => "Objetivo: Eliminar oxígeno residual inyectando Gas Reductor para moldear Ánodos planos sin poros.",
            Stage::Electrolysis => "Objetivo: Aplicar Amperaje en ácido sulfúrico para depositar Cátodos de 99.99% Cu, dejando lodos de metales nobles.",
            Stage::Atomization => "Objetivo: Convertir Cátodos fundidos (99.99%) en Polvo Esférico 3D (15 µm - 45 µm) inyectando Argón.",
            Stage::Printing => "Objetivo: Fundir polvo térmicamente con Láser de alta potencia capa por capa para manufactura final.",
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum SystemStatus {
    Normal,
    Warning(String),
    Critical(String),
    Fatal(String),
}

impl SystemStatus {
    pub fn is_blocking(&self) -> bool {
        matches!(self, SystemStatus::Critical(_) | SystemStatus::Fatal(_))
    }
    
    pub fn message(&self) -> String {
        match self {
            SystemStatus::Normal => "Operación Normal".to_string(),
            SystemStatus::Warning(msg) => msg.clone(),
            SystemStatus::Critical(msg) => msg.clone(),
            SystemStatus::Fatal(msg) => msg.clone(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct HornoFlashInput {
    pub o2_flow: i32,
    pub silica_flux: i32,
}

#[derive(Clone, Copy, PartialEq)]
pub struct ConvertidorInput {
    pub o2_flow: i32,
    pub scrap_added: i32,
}

#[derive(Clone, Copy, PartialEq)]
pub struct AfinoInput {
    pub reducing_gas: i32,
}

#[derive(Clone, Copy, PartialEq)]
pub struct ElectrolysisInput {
    pub current_amps: i32,
}

#[derive(Clone, Copy, PartialEq)]
pub struct AtomizationInput {
    pub gas_pressure: i32,
}

#[derive(Clone, Copy, PartialEq)]
pub struct PrintingInput {
    pub laser_power: i32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ProcessReport {
    pub primary_output: String,
    pub output_purity: f64,
    pub byproducts: String,
    pub status: SystemStatus,
}
