/// Motor de Física Puro (No acoplado a Dioxus)
/// Contiene las Ecuaciones Diferenciales Ordinarias (EDO) y cálculos puramente matemáticos.

pub fn calculate_fusion_temp(base: f64, o2_input: i32) -> f64 {
    // Ecuación simplificada de Arrhenius (+ O2 = heat)
    let heat_gen = o2_input as f64 * 2.5; 
    let rand_noise = rand::random::<f64>() * 10.0 - 5.0;
    base + heat_gen + rand_noise
}

pub fn calculate_conversion_temp(base: f64, scrap_rate: i32) -> f64 {
    // Problema de Stefan simplificado (chatarra actúa como sumidero térmico)
    let heat_loss = scrap_rate as f64 * 3.2;
    let rand_noise = rand::random::<f64>() * 8.0 - 4.0;
    base - heat_loss + rand_noise
}

pub fn calculate_slag_viscosity(temp: f64) -> f64 {
    // Escoria se congela exponencialmente con temperatura baja
    if temp < 1000.0 {
        50.0 // Viscosidad máxima, horno trancado
    } else {
        5.0 + (1150.0 - temp).max(0.0) * 0.15
    }
}

pub fn calculate_electrolytic_temp(voltage: i32) -> f64 {
    // Ecuación térmica por sobrepotencial de Butler-Volmer
    800.0 + (voltage as f64 * 0.8) + (rand::random::<f64>() * 4.0 - 2.0)
}

pub fn calculate_particle_size(gas_pressure: i32) -> f64 {
    // Ecuación empírica de Lubanska
    (150.0 - (gas_pressure as f64 * 1.2)).max(10.0)
}

pub fn calculate_laser_temp(laser_power: i32) -> f64 {
    // Ecuación térmica para PBF
    1500.0 + (laser_power as f64 * 12.0) + (rand::random::<f64>() * 15.0 - 7.5)
}

pub fn calculate_ved(laser_power: i32) -> f64 {
    // Densidad de Energía Volumétrica
    laser_power as f64 * 1.8
}
