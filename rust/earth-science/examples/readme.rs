//! Earth Science: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p earth-science-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&earth_science_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "heat_index", "inputs": {"T": "305.15 K", "RH": "0.7"}}));
    println!();
    show(json!({"key": "inertial_period", "inputs": {"lat": "45"}}));
    println!();
    show(json!({"key": "brunt_vaisala_frequency", "inputs": {"theta": "300 K", "dtheta_dz": "0.003 K/m"}}));
    println!();
    show(json!({"key": "dewpoint_from_relative_humidity", "inputs": {"T": "303.15 K", "RH": "0.6"}}));
    println!();
    show(json!({"key": "scale_height", "inputs": {"T": "288.15 K"}}));
    println!();
    show(json!({"key": "energy_balance_response", "inputs": {"F": "3.7 W/m^2", "lam": "0.8 K*m^2/W", "C": "420000000 J/(m^2*K)", "t": "10"}}));
    println!();
    show(json!({"key": "co2_radiative_forcing", "inputs": {"C": "560", "C0": "280"}}));
    println!();
    show(json!({"key": "radiocarbon_age", "inputs": {"F": "0.5"}}));
    println!();
    show(json!({"key": "pratt_isostasy", "inputs": {"h": "3000 m", "rho0": "2800 kg/m^3", "D": "100000 m"}}));
    println!();
    show(json!({"key": "dipole_inclination", "inputs": {"lat": "45"}}));
    println!();
    show(json!({"key": "stream_power", "inputs": {"Q": "100 m^3/s", "S": "0.002"}}));
    println!();
    show(json!({"key": "gravity_at_latitude", "inputs": {"lat": "45", "z": "0 m"}}));
    println!();
    show(json!({"key": "geotherm_radiogenic", "inputs": {"T0": "283.15 K", "q0": "0.065 W/m^2", "k": "2.5 W/(m*K)", "A": "1e-06 W/m^3", "z": "10000 m"}}));
    println!();
    show(json!({"key": "darcy_flow", "inputs": {"K": "0.0001 m/s", "A": "50 m^2", "dh": "2 m", "L": "100 m"}}));
    println!();
    show(json!({"key": "map_gradient_relief", "inputs": {"z1": "1200 m", "z2": "1450 m", "d_map": "0.05 m", "scale": "24000"}}));
}
