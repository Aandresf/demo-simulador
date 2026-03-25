use crate::models::{HornoFlashInput, ConvertidorInput, AfinoInput, ElectrolysisInput, AtomizationInput, PrintingInput, ProcessReport, SystemStatus};

pub fn evaluate_flash(input: &HornoFlashInput) -> ProcessReport {
    if input.o2_flow < 20 {
        return ProcessReport {
            primary_output: "Concentrado sin fundir".into(),
            output_purity: 25.0,
            byproducts: "Escoria Congelada".into(),
            status: SystemStatus::Critical("Defecto de O2: Mata fría. El Convertidor no tiene energía de activación. Bloqueo de línea.".into()),
        };
    }
    if input.o2_flow > 80 {
        return ProcessReport {
            primary_output: "Mata Oxidada (Magnetita)".into(),
            output_purity: 30.0,
            byproducts: "Fe3O4 Viscoso".into(),
            status: SystemStatus::Critical("Exceso O2: Oxidación excesiva. Se quema hierro útil formando Magnetita que bloquea la escoria.".into()),
        };
    }
    if input.silica_flux < 10 {
        return ProcessReport {
            primary_output: "Mata Impura".into(),
            output_purity: 40.0,
            byproducts: "Escoria Ultra-Viscosa".into(),
            status: SystemStatus::Critical("Defecto Sílice: Taponamiento de salidas por escoria. Falla mecánica simulada.".into()),
        };
    }
    if input.silica_flux > 80 {
        return ProcessReport {
            primary_output: "Mata de Baja Ley".into(),
            output_purity: 55.0,
            byproducts: "Abundante Escoria con Cobre".into(),
            status: SystemStatus::Warning("Exceso Sílice: Generación de volumen innecesario de escoria que atrapa gotas de cobre valioso (Rentabilidad baja).".into()),
        };
    }
    
    ProcessReport {
        primary_output: "Mata Líquida".into(),
        output_purity: 62.0,
        byproducts: "Escoria Férrica (FeSiO3)".into(),
        status: SystemStatus::Normal,
    }
}

pub fn evaluate_conversion(input: &ConvertidorInput) -> ProcessReport {
    if input.scrap_added < 10 {
        return ProcessReport {
            primary_output: "Ninguno (Explosión)".into(),
            output_purity: 0.0,
            byproducts: "Refractario Fundido".into(),
            status: SystemStatus::Fatal("Falta Chatarra: Fusión del refractario por falta de refrigerante. Riesgo de explosión. Detención total.".into()),
        };
    }
    if input.scrap_added > 60 {
        return ProcessReport {
            primary_output: "Baño Congelado".into(),
            output_purity: 62.0,
            byproducts: "Chatarra sin fundir".into(),
            status: SystemStatus::Fatal("Exceso Chatarra: Absorción térmica masiva. El baño de metal se enfrió, convertidor congelado (Pérdida total del equipo).".into()),
        };
    }
    if input.o2_flow < 40 {
        return ProcessReport {
            primary_output: "Blíster Sucio".into(),
            output_purity: 90.0,
            byproducts: "SO2".into(),
            status: SystemStatus::Critical("Defecto O2: Oxidación demasiada lenta. Blíster contaminado con azufre e hierro. Lote rechazado.".into()),
        };
    }
    if input.o2_flow > 80 {
        return ProcessReport {
            primary_output: "Blíster Pobre".into(),
            output_purity: 97.0,
            byproducts: "Cu2O (Pérdida de cobre)".into(),
            status: SystemStatus::Warning("Exceso O2: Quema el propio cobre generado (Cu2O), viéndose perdido en la escoria.".into()),
        };
    }
    
    ProcessReport {
        primary_output: "Cobre Blíster".into(),
        output_purity: 99.0,
        byproducts: "Gas SO2 (Capturado para Ácido)".into(),
        status: SystemStatus::Normal,
    }
}

pub fn evaluate_afino(input: &AfinoInput) -> ProcessReport {
    if input.reducing_gas > 70 {
        return ProcessReport {
            primary_output: "Ánodos Esponjosos".into(),
            output_purity: 90.0,
            byproducts: "Vapor de Agua / Hidrógeno Ocluido".into(),
            status: SystemStatus::Critical("Exceso Gas: El metal absorbió hidrógeno. Cobre hierve y solidifica con porosidad (ánodos deformes).".into()),
        };
    }
    if input.reducing_gas < 30 {
        return ProcessReport {
            primary_output: "Ánodos Quebradizos".into(),
            output_purity: 90.0,
            byproducts: "Oxígeno residual".into(),
            status: SystemStatus::Critical("Defecto Gas: El oxígeno no se elimina. Cobre oxidado y quebradizo (los ánodos se fracturan bajo peso).".into()),
        };
    }
    
    ProcessReport {
        primary_output: "Ánodos Sólidos".into(),
        output_purity: 99.7,
        byproducts: "H2O (Vapor)".into(),
        status: SystemStatus::Normal,
    }
}

pub fn evaluate_electrolysis(input: &ElectrolysisInput) -> ProcessReport {
    if input.current_amps > 80 {
        return ProcessReport {
            primary_output: "Cátodo Contaminado".into(),
            output_purity: 99.8,
            byproducts: "Lodos Anódicos (Ag, Au)".into(),
            status: SystemStatus::Fatal("Amperaje Excesivo: Proceso 'sucio'. Arranca impurezas (Zn/Pb) en el cátodo. Perdido el grado 99.99%.".into()),
        };
    }
    if input.current_amps < 20 {
        return ProcessReport {
            primary_output: "Cátodo Muy Delgado".into(),
            output_purity: 99.99,
            byproducts: "Tasa muy baja de lodo".into(),
            status: SystemStatus::Warning("Amperaje bajo: Tasa de deposición ineficiente. El flujo de material se estancó.".into()),
        };
    }
    
    ProcessReport {
        primary_output: "Cátodos Ultra Alta Pureza".into(),
        output_purity: 99.99,
        byproducts: "Lodos Anódicos Ricos (Ag, Au)".into(),
        status: SystemStatus::Normal,
    }
}

pub fn evaluate_atomization(input: &AtomizationInput) -> ProcessReport {
    if input.gas_pressure < 30 {
        return ProcessReport {
            primary_output: "Polvo Gigante (Falla)".into(),
            output_purity: 99.99,
            byproducts: "Chatarra irregular".into(),
            status: SystemStatus::Critical("Defecto Presión: Falta de fuerza de impacto. Polvo gigante o irregular que atasca la impresora 3D.".into()),
        };
    }
    if input.gas_pressure > 80 {
        return ProcessReport {
            primary_output: "Polvo Ultrafino Satélite".into(),
            output_purity: 99.99,
            byproducts: "Nubes de polvo fino".into(),
            status: SystemStatus::Critical("Exceso Presión: Pulverización excesiva. El polvo pierde fluidez y no se puede esparcir en la cama 3D.".into()),
        };
    }
    
    ProcessReport {
        primary_output: "Polvo Esférico 3D (15-45 μm)".into(),
        output_purity: 99.99,
        byproducts: "Partículas Satélites Descartadas".into(),
        status: SystemStatus::Normal,
    }
}

pub fn evaluate_printing(input: &PrintingInput) -> ProcessReport {
    if input.laser_power < 30 {
        return ProcessReport {
            primary_output: "Pieza Quebradiza/Porosa".into(),
            output_purity: 99.99,
            byproducts: "Polvo sin fundir".into(),
            status: SystemStatus::Fatal("Defecto Láser: No atraviesa conductividad del cobre. Partículas no se unen (falta de fusión).".into()),
        };
    }
    if input.laser_power > 80 {
        return ProcessReport {
            primary_output: "Pieza Deformada".into(),
            output_purity: 99.99,
            byproducts: "Vapor de Cobre".into(),
            status: SystemStatus::Critical("Exceso Láser: Evaporación térmica masiva. Desarrolla cráteres y distorsión superficial.".into()),
        };
    }
    
    ProcessReport {
        primary_output: "Pieza Industrial Homogénea".into(),
        output_purity: 99.99,
        byproducts: "Tensiones residuales aliviadas".into(),
        status: SystemStatus::Normal,
    }
}

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
