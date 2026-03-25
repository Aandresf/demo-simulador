use crate::models::{HornoFlashInput, ConvertidorInput, AfinoInput, ElectrolysisInput, AtomizationInput, PrintingInput, ProcessReport, SystemStatus};

/// Motor de Física y FSM
/// Interlocks basados en contexto_simulador.md Tabulador de Alarmas

pub fn evaluate_flash(input: &HornoFlashInput) -> ProcessReport {
    if input.o2_flow < 20 {
        return ProcessReport {
            output_purity: 25.0,
            byproducts: "Escoria Congelada".into(),
            status: SystemStatus::Critical("Mata fría. El Convertidor no tiene energía de activación. Bloqueo de línea.".into()),
        };
    }
    if input.silica_flux < 10 {
        return ProcessReport {
            output_purity: 40.0,
            byproducts: "Escoria Ultra-Viscosa".into(),
            status: SystemStatus::Critical("Taponamiento de salidas por escoria. Falla mecánica simulada.".into()),
        };
    }
    
    ProcessReport {
        output_purity: 62.0,
        byproducts: "Escoria Férrica (FeSiO3)".into(),
        status: SystemStatus::Normal,
    }
}

pub fn evaluate_conversion(input: &ConvertidorInput) -> ProcessReport {
    if input.scrap_added < 10 {
        return ProcessReport {
            output_purity: 0.0,
            byproducts: "Ninguno (Explosión)".into(),
            status: SystemStatus::Fatal("Fusión del refractario por falta de refrigerante de chatarra. Riesgo de explosión. Detención total.".into()),
        };
    }
    if input.o2_flow < 40 {
        return ProcessReport {
            output_purity: 90.0,
            byproducts: "SO2".into(),
            status: SystemStatus::Critical("Oxidación lenta. Blíster contaminado con azufre e hierro. Lote rechazado.".into()),
        };
    }
    if input.o2_flow > 80 {
        return ProcessReport {
            output_purity: 97.0,
            byproducts: "Cu2O (Pérdida de cobre en escoria)".into(),
            status: SystemStatus::Warning("Oxidación excesiva daña el rendimiento final quemando el cobre valioso.".into()),
        };
    }
    
    ProcessReport {
        output_purity: 99.0,
        byproducts: "Gas SO2 (Capturado para Ácido)".into(),
        status: SystemStatus::Normal,
    }
}

pub fn evaluate_afino(input: &AfinoInput) -> ProcessReport {
    if input.reducing_gas > 70 {
        return ProcessReport {
            output_purity: 99.7,
            byproducts: "Vapor de Agua".into(),
            status: SystemStatus::Critical("Exceso de gas. Absorción de hidrógeno: Ánodos esponjosos/deformes causarán cortocircuito.".into()),
        };
    }
    
    ProcessReport {
        output_purity: 99.7,
        byproducts: "H2O (Vapor)".into(),
        status: SystemStatus::Normal,
    }
}

pub fn evaluate_electrolysis(input: &ElectrolysisInput) -> ProcessReport {
    if input.current_amps > 80 {
        return ProcessReport {
            output_purity: 99.8,
            byproducts: "Lodos Anódicos (Ag, Au)".into(),
            status: SystemStatus::Fatal("Amperaje Excesivo: Cátodo arranca impurezas. Se pierde el grado 99.99%. No apto para 3D. Desvío a chatarra.".into()),
        };
    }
    if input.current_amps < 20 {
        return ProcessReport {
            output_purity: 99.99,
            byproducts: "Lodos Anódicos (Ag, Au)".into(),
            status: SystemStatus::Warning("Flujo de material estancado por amperaje bajo. Pérdida de productividad.".into()),
        };
    }
    
    ProcessReport {
        output_purity: 99.99,
        byproducts: "Lodos Anódicos Ricos en Plata/Oro".into(),
        status: SystemStatus::Normal,
    }
}

pub fn evaluate_atomization(input: &AtomizationInput) -> ProcessReport {
    if input.gas_pressure < 30 {
        return ProcessReport {
            output_purity: 99.99,
            byproducts: "Chatarra irregular".into(),
            status: SystemStatus::Critical("Falta de fuerza de impacto. Polvo gigante irregular. La impresora 3D se atascará.".into()),
        };
    }
    
    ProcessReport {
        output_purity: 99.99,
        byproducts: "Partículas Satélites Descartadas".into(),
        status: SystemStatus::Normal,
    }
}

pub fn evaluate_printing(input: &PrintingInput) -> ProcessReport {
    if input.laser_power < 30 {
        return ProcessReport {
            output_purity: 99.99,
            byproducts: "Polvo sin fundir".into(),
            status: SystemStatus::Fatal("Falta de fusión por reflectividad del cobre. Pieza porosa e inservible.".into()),
        };
    }
    if input.laser_power > 80 {
        return ProcessReport {
            output_purity: 99.99,
            byproducts: "Vapor de Cobre".into(),
            status: SystemStatus::Warning("Exceso de energía genera cráteres y distorsión superficial.".into()),
        };
    }
    
    ProcessReport {
        output_purity: 99.99,
        byproducts: "Pieza Industrial Finalizada".into(),
        status: SystemStatus::Normal,
    }
}

/// Fórmulas térmicas para visualización en GUI:
pub fn calculate_fusion_temp(base: f64, o2_input: i32) -> f64 {
    let heat_gen = o2_input as f64 * 2.5; 
    let rand_noise = rand::random::<f64>() * 10.0 - 5.0;
    base + heat_gen + rand_noise
}

pub fn calculate_conversion_temp(base: f64, scrap_rate: i32) -> f64 {
    let heat_loss = scrap_rate as f64 * 3.2;
    let rand_noise = rand::random::<f64>() * 8.0 - 4.0;
    base - heat_loss + rand_noise
}

pub fn calculate_slag_viscosity(temp: f64) -> f64 {
    if temp < 1000.0 {
        50.0
    } else {
        5.0 + (1150.0 - temp).max(0.0) * 0.15
    }
}

pub fn calculate_electrolytic_temp(voltage: i32) -> f64 {
    800.0 + (voltage as f64 * 0.8) + (rand::random::<f64>() * 4.0 - 2.0)
}

pub fn calculate_particle_size(gas_pressure: i32) -> f64 {
    (150.0 - (gas_pressure as f64 * 1.2)).max(10.0)
}

pub fn calculate_laser_temp(laser_power: i32) -> f64 {
    1500.0 + (laser_power as f64 * 12.0) + (rand::random::<f64>() * 15.0 - 7.5)
}

pub fn calculate_ved(laser_power: i32) -> f64 {
    laser_power as f64 * 1.8
}
