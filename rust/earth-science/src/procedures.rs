//! earth_science: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/earth_science/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "airy_isostasy",
    "blackbody_flux",
    "bouguer_slab",
    "brunt_vaisala_frequency",
    "buried_sphere_anomaly",
    "co2_radiative_forcing",
    "coriolis_parameter",
    "daily_mean_insolation",
    "darcy_flow",
    "dewpoint_from_relative_humidity",
    "dewpoint_from_vapor_pressure",
    "dipole_inclination",
    "dry_adiabatic_ascent",
    "ekman_depth",
    "ekman_transport",
    "elastic_constants_from_velocities",
    "energy_balance_response",
    "epicentral_distance",
    "equivalent_potential_temperature",
    "geodesic_distance_azimuth",
    "geopotential_height",
    "geostrophic_wind",
    "geotherm_radiogenic",
    "gradient_wind",
    "gravity_at_latitude",
    "greenhouse_n_layers",
    "greenhouse_single_layer",
    "gutenberg_richter_b_value",
    "half_space_heat_flow",
    "heat_index",
    "hypsometric_thickness",
    "inertial_period",
    "isochron_age",
    "isothermal_pressure",
    "lifting_condensation_level",
    "local_magnitude",
    "mixing_ratio",
    "moist_adiabatic_lapse_rate",
    "moist_air_density",
    "moist_static_energy",
    "moment_magnitude",
    "nmo_velocity",
    "ocean_depth_from_pressure",
    "ocean_pressure_from_depth",
    "paleolatitude",
    "planetary_equilibrium_temperature",
    "plate_velocity",
    "potassium_argon_age",
    "potential_temperature",
    "pratt_isostasy",
    "pressure_with_lapse_rate",
    "pure_water_density",
    "radioactive_decay_remaining",
    "radiocarbon_age",
    "radiometric_age",
    "relative_humidity_from_dewpoint",
    "rossby_number",
    "rossby_wave_speed",
    "saturation_mixing_ratio",
    "saturation_vapor_pressure",
    "scale_height",
    "seafloor_depth_age",
    "seawater_adiabatic_gradient",
    "seawater_density",
    "seawater_density_surface",
    "seawater_freezing_point",
    "seawater_potential_temperature",
    "seismic_velocities_from_moduli",
    "seismic_velocities_from_young",
    "shallow_water_wave",
    "solar_elevation_angle",
    "sound_speed_seawater",
    "sp_epicentral_distance",
    "specific_humidity_from_dewpoint",
    "standard_atmosphere_height",
    "standard_atmosphere_pressure",
    "stokes_settling_velocity",
    "stream_power",
    "temperature_from_potential_temperature",
    "virtual_temperature",
    "wave_dispersion",
    "wien_peak_wavelength",
    "wind_chill",
    "wind_components",
    "wind_speed_direction",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "airy_isostasy" => {
            let r = f_airy_root(match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tc") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "blackbody_flux" => {
            let r = f_blackbody_flux(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "bouguer_slab" => {
            let r = f_bouguer_slab(match v.get("drho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "brunt_vaisala_frequency" => {
            let r = f_brunt_vaisala(match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dtheta_dz") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "buried_sphere_anomaly" => {
            let r = f_buried_sphere(match v.get("drho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "co2_radiative_forcing" => {
            let r = f_co2_forcing(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "coriolis_parameter" => {
            let r = f_coriolis(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "daily_mean_insolation" => {
            let r = f_daily_insolation(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("decl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "darcy_flow" => {
            let r = f_darcy_flow(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dh") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "dewpoint_from_relative_humidity" => {
            let r = f_dewpoint_from_rh(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RH") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "dewpoint_from_vapor_pressure" => {
            let r = f_dewpoint_from_vapor_pressure(match v.get("e") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "dipole_inclination" => {
            let r = f_dipole_field(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "dry_adiabatic_ascent" => {
            let r = f_dry_adiabatic_ascent(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dz") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "ekman_depth" => {
            let r = f_ekman_depth(match v.get("Az") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "ekman_transport" => {
            let r = f_ekman_transport(match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1025.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "elastic_constants_from_velocities" => {
            let r = f_elastic_from_velocities(match v.get("Vp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "energy_balance_response" => {
            let r = f_energy_balance_response(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "epicentral_distance" => {
            let r = f_epicentral_distance(match v.get("lat1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "equivalent_potential_temperature" => {
            let r = f_equivalent_potential_temperature(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Td") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "geodesic_distance_azimuth" => {
            let r = f_vincenty_inverse(match v.get("lat1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "geopotential_height" => {
            let r = f_geopotential(match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "geostrophic_wind" => {
            let r = f_geostrophic_wind(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dz") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dn") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "geotherm_radiogenic" => {
            let r = f_geotherm(match v.get("T0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "gradient_wind" => {
            let r = f_gradient_wind(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vg") { Some(x) => py::from_val(x), None => { return None } }, match v.get("flow") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "gravity_at_latitude" => {
            let r = f_gravity_latitude(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "greenhouse_n_layers" => {
            let r = f_greenhouse_n_layers(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("albedo") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "greenhouse_single_layer" => {
            let r = f_greenhouse_single_layer(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("albedo") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "gutenberg_richter_b_value" => {
            let r = f_b_value(match v.get("mags") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Mc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dM") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "half_space_heat_flow" => {
            let r = f_half_space_heat_flow(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dT") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1300.0)) })().ok()? } }, match v.get("k") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(3.3)) })().ok()? } }, match v.get("kappa") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1e-06)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "heat_index" => {
            let r = f_heat_index(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RH") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "hypsometric_thickness" => {
            let r = f_hypsometric_thickness(match v.get("p1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tv") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "inertial_period" => {
            let r = f_inertial_period(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "isochron_age" => {
            let r = f_isochron_age(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.42e-11)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "isothermal_pressure" => {
            let r = f_isothermal_pressure(match v.get("p0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "lifting_condensation_level" => {
            let r = f_lcl(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Td") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "local_magnitude" => {
            let r = f_local_magnitude(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "mixing_ratio" => {
            let r = f_mixing_ratio(match v.get("e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "moist_adiabatic_lapse_rate" => {
            let r = f_moist_lapse_rate(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "moist_air_density" => {
            let r = f_moist_air_density(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "moist_static_energy" => {
            let r = f_moist_static_energy(match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "moment_magnitude" => {
            let r = f_moment_magnitude(match v.get("M0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "nmo_velocity" => {
            let r = f_nmo_velocity(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "ocean_depth_from_pressure" => {
            let r = f_ocean_depth(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "ocean_pressure_from_depth" => {
            let r = f_ocean_pressure(match v.get("depth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "paleolatitude" => {
            let r = f_paleolatitude(match v.get("I_deg") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "planetary_equilibrium_temperature" => {
            let r = f_equilibrium_temperature(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("albedo") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "plate_velocity" => {
            let r = f_plate_velocity(match v.get("pole_lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pole_lon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "potassium_argon_age" => {
            let r = f_k_ar_age(match v.get("ratio") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(5.543e-10)) })().ok()? } }, match v.get("lam_e") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(5.81e-11)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "potential_temperature" => {
            let r = f_potential_temperature(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "pratt_isostasy" => {
            let r = f_pratt_density(match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "pressure_with_lapse_rate" => {
            let r = f_lapse_rate_pressure(match v.get("p0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "pure_water_density" => {
            let r = f_pure_water_density(match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "radioactive_decay_remaining" => {
            let r = f_decay_remaining(match v.get("half_life") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "radiocarbon_age" => {
            let r = f_radiocarbon_age(match v.get("F") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "radiometric_age" => {
            let r = f_radiometric_age(match v.get("ratio") { Some(x) => py::from_val(x), None => { return None } }, match v.get("half_life") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "relative_humidity_from_dewpoint" => {
            let r = f_relative_humidity(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Td") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "rossby_number" => {
            let r = f_rossby_number(match v.get("U") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "rossby_wave_speed" => {
            let r = f_rossby_wave(match v.get("U") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Lx") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "saturation_mixing_ratio" => {
            let r = f_saturation_mixing_ratio(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "saturation_vapor_pressure" => {
            let r = f_saturation_vapor_pressure(match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "scale_height" => {
            let r = f_scale_height(match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "seafloor_depth_age" => {
            let r = f_seafloor_depth(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dr") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(2500.0)) })().ok()? } }, match v.get("Tm") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1573.15)) })().ok()? } }, match v.get("Tw") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(273.15)) })().ok()? } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(3e-05)) })().ok()? } }, match v.get("kappa") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1e-06)) })().ok()? } }, match v.get("rho_m") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(3300.0)) })().ok()? } }, match v.get("rho_w") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1000.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "seawater_adiabatic_gradient" => {
            let r = f_seawater_adiabatic_gradient(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "seawater_density" => {
            let r = f_seawater_density(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "seawater_density_surface" => {
            let r = f_seawater_density_surface(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "seawater_freezing_point" => {
            let r = f_seawater_freezing_point(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "seawater_potential_temperature" => {
            let r = f_seawater_potential_temperature(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pr") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "seismic_velocities_from_moduli" => {
            let r = f_velocities_from_moduli(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "seismic_velocities_from_young" => {
            let r = f_velocities_from_young(match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "shallow_water_wave" => {
            let r = f_shallow_water_wave(match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("distance") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "solar_elevation_angle" => {
            let r = f_solar_elevation(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("decl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "sound_speed_seawater" => {
            let r = f_sound_speed_seawater(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "sp_epicentral_distance" => {
            let r = f_sp_distance(match v.get("dt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("vp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("vs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "specific_humidity_from_dewpoint" => {
            let r = f_specific_humidity_from_dewpoint(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Td") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "standard_atmosphere_height" => {
            let r = f_std_height(match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "standard_atmosphere_pressure" => {
            let r = f_std_pressure(match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "stokes_settling_velocity" => {
            let r = f_stokes_settling(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "stream_power" => {
            let r = f_stream_power(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("rho") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1000.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "temperature_from_potential_temperature" => {
            let r = f_temperature_from_theta(match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "virtual_temperature" => {
            let r = f_virtual_temperature(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "wave_dispersion" => {
            let r = f_wave_dispersion(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "wien_peak_wavelength" => {
            let r = f_wien_peak(match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "wind_chill" => {
            let r = f_wind_chill(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "wind_components" => {
            let r = f_wind_components(match v.get("speed") { Some(x) => py::from_val(x), None => { return None } }, match v.get("direction") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "wind_speed_direction" => {
            let r = f_wind_speed_direction(match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_CPD() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1004.6662184201462)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_CPL() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(4219.400000000001)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_CPV() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1860.078011865639)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_EPS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.6219569100577033)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_ERFINV_09() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.1630871536766743)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_ES0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(611.2)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_G0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(9.80665)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_G_NEWTON() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6.6743e-11)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_KAPPA() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.28571428571428564)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_LV0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(2500840.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_OMEGA() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(7.292115e-05)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_P0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(100000.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_RD() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(287.04749097718457)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_RE() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6371008.7714)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_RV() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(461.52311572606084)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_R_EARTH_KM() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6371.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_SIGMA() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(5.670374419e-08)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_T0TP() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(273.16)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_WIEN_B() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.002897771955)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_YEAR() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(31557600.0)) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__T_ok(mut v_T: Py, mut v_lo: Py, mut v_hi: Py, mut v_name: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("T"), v_T.clone())]))?;
    let _ = f__need({ let t1445 = v_lo.clone(); Py::Bool({ let t1446 = v_T.clone(); if py::le(&t1445, &t1446)? { { let t1447 = v_hi.clone(); if py::le(&t1446, &t1447)? { true } else { false } } } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must lie between ".to_string(), py::format(&py::sub(&v_lo.clone(), &Py::Float(273.15))?, "g")?, " and ".to_string(), py::format(&py::sub(&v_hi.clone(), &Py::Float(273.15))?, "g")?, " °C".to_string()].concat()))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__adtg(mut v_s: Py, mut v_t: Py, mut v_p: Py) -> R {
    let mut v_T: Py = Py::None;
    v_T = f__t68(v_t.clone())?;
    return Ok(py::add(&py::add(&py::add(&py::add(&Py::Float(3.5803e-05), &py::mul(&py::add(&Py::Float(8.5258e-06), &py::mul(&py::add(&py::neg(&Py::Float(6.836e-08))?, &py::mul(&Py::Float(6.6228e-10), &v_T.clone())?)?, &v_T.clone())?)?, &v_T.clone())?)?, &py::mul(&py::sub(&Py::Float(1.8932e-06), &py::mul(&Py::Float(4.2393e-08), &v_T.clone())?)?, &py::sub(&v_s.clone(), &Py::Int(35i64))?)?)?, &py::mul(&py::add(&py::add(&Py::Float(1.8741e-08), &py::mul(&py::add(&py::neg(&Py::Float(6.7795e-10))?, &py::mul(&py::sub(&Py::Float(8.733e-12), &py::mul(&Py::Float(5.4481e-14), &v_T.clone())?)?, &v_T.clone())?)?, &v_T.clone())?)?, &py::mul(&py::add(&py::neg(&Py::Float(1.1351e-10))?, &py::mul(&Py::Float(2.7759e-12), &v_T.clone())?)?, &py::sub(&v_s.clone(), &Py::Int(35i64))?)?)?, &v_p.clone())?)?, &py::mul(&py::mul(&py::add(&py::neg(&Py::Float(4.6206e-13))?, &py::mul(&py::sub(&Py::Float(1.8676e-14), &py::mul(&Py::Float(2.1687e-16), &v_T.clone())?)?, &v_T.clone())?)?, &v_p.clone())?, &v_p.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__coord_ok(mut v_kw: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_lim: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t839 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? {
        let t848 = (t839).items()?;
        if t848.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t848[0].clone();
        v_v = t848[1].clone();
        let _ = f__finite(Py::dict(vec![(v_k.clone(), v_v.clone())]))?;
        v_lim = (if (py::method(&v_k.clone(), "startswith", &[Py::str("lat")])?).truthy() { Py::Int(90i64) } else { Py::Int(360i64) });
        let _ = f__need({ let t845 = py::neg(&v_lim.clone())?; Py::Bool({ let t846 = v_v.clone(); if py::le(&t845, &t846)? { { let t847 = v_lim.clone(); if py::le(&t846, &t847)? { true } else { false } } } else { false } }) }, Py::str(&[py::format(&v_k.clone(), "")?, " must be between -".to_string(), py::format(&v_lim.clone(), "")?, " and ".to_string(), py::format(&v_lim.clone(), "")?, " degrees".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dens(mut v_s: Py, mut v_t: Py, mut v_p: Py) -> R {
    return Ok(py::truediv(&f__dens0(v_s.clone(), v_t.clone())?, &py::sub(&Py::Int(1i64), &py::truediv(&py::truediv(&v_p.clone(), &Py::Float(10.0))?, &f__seck(v_s.clone(), v_t.clone(), v_p.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dens0(mut v_s: Py, mut v_t: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    v_T = f__t68(v_t.clone())?;
    v_b = Py::tuple(vec![Py::Float(0.824493), py::neg(&Py::Float(0.0040899))?, Py::Float(7.6438e-05), py::neg(&Py::Float(8.2467e-07))?, Py::Float(5.3875e-09)]);
    v_c = Py::tuple(vec![py::neg(&Py::Float(0.00572466))?, Py::Float(0.00010227), py::neg(&Py::Float(1.6546e-06))?]);
    return Ok(py::add(&py::add(&py::add(&f__smow(v_t.clone())?, &py::mul(&py::add(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::mul(&py::add(&py::getitem(&v_b.clone(), &Py::Int(1i64))?, &py::mul(&py::add(&py::getitem(&v_b.clone(), &Py::Int(2i64))?, &py::mul(&py::add(&py::getitem(&v_b.clone(), &Py::Int(3i64))?, &py::mul(&py::getitem(&v_b.clone(), &Py::Int(4i64))?, &v_T.clone())?)?, &v_T.clone())?)?, &v_T.clone())?)?, &v_T.clone())?)?, &v_s.clone())?)?, &py::mul(&py::mul(&py::add(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &py::mul(&py::add(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &py::mul(&py::getitem(&v_c.clone(), &Py::Int(2i64))?, &v_T.clone())?)?, &v_T.clone())?)?, &v_s.clone())?, &py::math1("sqrt", &v_s.clone())?)?)?, &py::mul(&py::mul(&Py::Float(0.00048314), &v_s.clone())?, &v_s.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dewpoint_bolton(mut v_e: Py) -> R {
    let mut v_v: Py = Py::None;
    v_v = py::math1("log", &py::truediv(&v_e.clone(), &c_ES0()?)?)?;
    return Ok(py::add(&Py::Float(273.15), &py::truediv(&py::mul(&Py::Float(243.5), &v_v.clone())?, &py::sub(&Py::Float(17.67), &v_v.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__es(mut v_T: Py) -> R {
    return Ok(py::mul(&py::mul(&c_ES0()?, &py::pow(&py::truediv(&c_T0TP()?, &v_T.clone())?, &py::truediv(&py::sub(&c_CPL()?, &c_CPV()?)?, &c_RV()?)?)?)?, &py::math1("exp", &py::truediv(&py::sub(&py::truediv(&c_LV0()?, &c_T0TP()?)?, &py::truediv(&f__latent(v_T.clone())?, &v_T.clone())?)?, &c_RV()?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__finite(mut v_kw: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t1525 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? {
        let t1528 = (t1525).items()?;
        if t1528.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1528[0].clone();
        v_v = t1528[1].clone();
        if ({ let t1527 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t1527.truthy() { t1527 } else { { let t1526 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t1526.truthy() { t1526 } else { py::not(&py::math1("isfinite", &v_v.clone())?) } } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_k.clone(), "")?, " must be a finite number".to_string()].concat()))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__heat_index_F(mut v_TF: Py, mut v_RH: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_hi: Py = Py::None;
    v_a = py::add(&py::add(&py::neg(&Py::Float(10.3))?, &py::mul(&Py::Float(1.1), &v_TF.clone())?)?, &py::mul(&Py::Float(4.7), &v_RH.clone())?)?;
    if ({ let t537 = v_TF.clone(); Py::Bool({ let t538 = Py::Int(40i64); if py::le(&t537, &t538)? { true } else { false } }) }).truthy() {
        v_hi = v_TF.clone();
    } else {
        if ({ let t539 = v_a.clone(); Py::Bool({ let t540 = Py::Int(79i64); if py::lt(&t539, &t540)? { true } else { false } }) }).truthy() {
            v_hi = v_a.clone();
        } else {
            v_hi = py::sub(&py::add(&py::add(&py::sub(&py::sub(&py::sub(&py::add(&py::add(&py::neg(&Py::Float(42.379))?, &py::mul(&Py::Float(2.04901523), &v_TF.clone())?)?, &py::mul(&Py::Float(1014.333127), &v_RH.clone())?)?, &py::mul(&py::mul(&Py::Float(22.475541), &v_TF.clone())?, &v_RH.clone())?)?, &py::mul(&py::mul(&Py::Float(0.00683783), &v_TF.clone())?, &v_TF.clone())?)?, &py::mul(&py::mul(&Py::Float(548.1717), &v_RH.clone())?, &v_RH.clone())?)?, &py::mul(&py::mul(&py::mul(&Py::Float(0.122874), &v_TF.clone())?, &v_TF.clone())?, &v_RH.clone())?)?, &py::mul(&py::mul(&py::mul(&Py::Float(8.5282), &v_TF.clone())?, &v_RH.clone())?, &v_RH.clone())?)?, &py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(0.0199), &v_TF.clone())?, &v_TF.clone())?, &v_RH.clone())?, &v_RH.clone())?)?;
        }
    }
    if ({ let t546 = { let t541 = v_RH.clone(); Py::Bool({ let t542 = Py::Float(0.13); if py::le(&t541, &t542)? { true } else { false } }) }; if !t546.truthy() { t546 } else { { let t543 = Py::Int(80i64); Py::Bool({ let t544 = v_TF.clone(); if py::le(&t543, &t544)? { { let t545 = Py::Int(112i64); if py::le(&t544, &t545)? { true } else { false } } } else { false } }) } } }).truthy() {
        v_hi = py::sub(&v_hi, &py::mul(&py::truediv(&py::sub(&Py::Float(13.0), &py::mul(&v_RH.clone(), &Py::Float(100.0))?)?, &Py::Float(4.0))?, &py::math1("sqrt", &py::truediv(&py::sub(&Py::Float(17.0), &py::abs(&py::sub(&v_TF.clone(), &Py::Float(95.0))?)?)?, &Py::Float(17.0))?)?)?)?;
    }
    if ({ let t552 = { let t547 = v_RH.clone(); Py::Bool({ let t548 = Py::Float(0.85); if py::gt(&t547, &t548)? { true } else { false } }) }; if !t552.truthy() { t552 } else { { let t549 = Py::Int(80i64); Py::Bool({ let t550 = v_TF.clone(); if py::le(&t549, &t550)? { { let t551 = Py::Int(87i64); if py::le(&t550, &t551)? { true } else { false } } } else { false } }) } } }).truthy() {
        v_hi = py::iadd(&v_hi, &py::mul(&py::mul(&Py::Float(0.02), &py::sub(&py::mul(&v_RH.clone(), &Py::Float(100.0))?, &Py::Float(85.0))?)?, &py::sub(&Py::Float(87.0), &v_TF.clone())?)?)?;
    }
    return Ok(v_hi.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lambertw_m1(mut v_x: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_ew: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_l1: Py = Py::None;
    let mut v_l2: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_wn: Py = Py::None;
    let _ = f__need({ let t646 = py::truediv(&py::neg(&Py::Int(1i64))?, &Py::Float(std::f64::consts::E))?; Py::Bool({ let t647 = v_x.clone(); if py::le(&t646, &t647)? { { let t648 = Py::Int(0i64); if py::lt(&t647, &t648)? { true } else { false } } } else { false } }) }, Py::str("the Lambert W argument is outside [-1/e, 0)"))?;
    if ({ let t649 = v_x.clone(); Py::Bool({ let t650 = py::add(&py::truediv(&py::neg(&Py::Int(1i64))?, &Py::Float(std::f64::consts::E))?, &Py::Float(1e-300))?; if py::le(&t649, &t650)? { true } else { false } }) }).truthy() {
        return Ok(py::neg(&Py::Float(1.0))?);
    }
    if ({ let t651 = v_x.clone(); Py::Bool({ let t652 = py::neg(&Py::Float(0.25))?; if py::lt(&t651, &t652)? { true } else { false } }) }).truthy() {
        v_p = py::neg(&py::math1("sqrt", &py::mul(&Py::Int(2i64), &py::add(&Py::Int(1i64), &py::mul(&Py::Float(std::f64::consts::E), &v_x.clone())?)?)?)?)?;
        v_w = py::add(&py::sub(&py::add(&py::neg(&Py::Int(1i64))?, &v_p.clone())?, &py::truediv(&py::mul(&v_p.clone(), &v_p.clone())?, &Py::Int(3i64))?)?, &py::mul(&py::truediv(&Py::Int(11i64), &Py::Int(72i64))?, &py::pow(&v_p.clone(), &Py::Int(3i64))?)?)?;
    } else {
        v_l1 = py::math1("log", &py::neg(&v_x.clone())?)?;
        v_l2 = py::math1("log", &py::neg(&v_l1.clone())?)?;
        v_w = py::add(&py::sub(&v_l1.clone(), &v_l2.clone())?, &py::truediv(&v_l2.clone(), &v_l1.clone())?)?;
    }
    for t653 in py::range_iter(&[(Py::Int(100i64)).i()?]) {
        v__ = t653;
        v_ew = py::math1("exp", &v_w.clone())?;
        v_f = py::sub(&py::mul(&v_w.clone(), &v_ew.clone())?, &v_x.clone())?;
        v_wn = py::sub(&v_w.clone(), &py::truediv(&v_f.clone(), &py::sub(&py::mul(&v_ew.clone(), &py::add(&v_w.clone(), &Py::Int(1i64))?)?, &py::truediv(&py::mul(&py::add(&v_w.clone(), &Py::Int(2i64))?, &v_f.clone())?, &py::add(&py::mul(&Py::Int(2i64), &v_w.clone())?, &Py::Int(2i64))?)?)?)?)?;
        if ({ let t654 = py::abs(&py::sub(&v_wn.clone(), &v_w.clone())?)?; Py::Bool({ let t655 = py::mul(&Py::Float(4e-16), &py::abs(&v_wn.clone())?)?; if py::le(&t654, &t655)? { true } else { false } }) }).truthy() {
            return Ok(v_wn.clone());
        }
        v_w = v_wn.clone();
    }
    return Ok(v_w.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lat_ok(mut v_lat: Py, mut v_lim: Py, mut v_name: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("lat"), v_lat.clone())]))?;
    let _ = f__need({ let t1289 = py::neg(&v_lim.clone())?; Py::Bool({ let t1290 = v_lat.clone(); if py::le(&t1289, &t1290)? { { let t1291 = v_lim.clone(); if py::le(&t1290, &t1291)? { true } else { false } } } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be between -".to_string(), py::format(&v_lim.clone(), "g")?, " and ".to_string(), py::format(&v_lim.clone(), "g")?, " degrees".to_string()].concat()))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__latent(mut v_T: Py) -> R {
    return Ok(py::sub(&c_LV0()?, &py::mul(&py::sub(&c_CPL()?, &c_CPV()?)?, &py::sub(&v_T.clone(), &c_T0TP()?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lcl(mut v_p: Py, mut v_T: Py, mut v_Td: Py, mut v_rh_power: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cpm: Py = Py::None;
    let mut v_mhr: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_rh: Py = Py::None;
    let mut v_rm: Py = Py::None;
    let mut v_sd: Py = Py::None;
    let mut v_tl: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_wm1: Py = Py::None;
    v_w = f__ws(v_p.clone(), v_Td.clone())?;
    v_q = py::truediv(&v_w.clone(), &py::add(&Py::Int(1i64), &v_w.clone())?)?;
    v_cpm = py::add(&c_CPD()?, &py::mul(&v_q.clone(), &py::sub(&c_CPV()?, &c_CPD()?)?)?)?;
    v_rm = py::add(&c_RD()?, &py::mul(&v_q.clone(), &py::sub(&c_RV()?, &c_RD()?)?)?)?;
    v_mhr = py::truediv(&v_cpm.clone(), &v_rm.clone())?;
    v_sd = py::sub(&c_CPL()?, &c_CPV()?)?;
    v_a = py::add(&v_mhr.clone(), &py::truediv(&v_sd.clone(), &c_RV()?)?)?;
    v_b = py::truediv(&py::neg(&py::add(&c_LV0()?, &py::mul(&v_sd.clone(), &c_T0TP()?)?)?)?, &py::mul(&c_RV()?, &v_T.clone())?)?;
    v_c = py::truediv(&v_b.clone(), &v_a.clone())?;
    v_rh = py::truediv(&f__es(v_Td.clone())?, &f__es(v_T.clone())?)?;
    v_wm1 = f__lambertw_m1(py::mul(&py::mul(&py::pow(&v_rh.clone(), &(if ({ let t656 = v_rh_power.clone(); Py::Bool({ let t657 = Py::None; if py::is(&t656, &t657) { true } else { false } }) }).truthy() { py::truediv(&Py::Int(1i64), &v_a.clone())? } else { py::call(&v_rh_power.clone(), &[v_a.clone()])? }))?, &v_c.clone())?, &py::math1("exp", &v_c.clone())?)?)?;
    v_tl = py::mul(&py::truediv(&v_c.clone(), &v_wm1.clone())?, &v_T.clone())?;
    return Ok(Py::tuple(vec![py::mul(&v_p.clone(), &py::pow(&py::truediv(&v_tl.clone(), &v_T.clone())?, &v_mhr.clone())?)?, v_tl.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need(mut v_cond: Py, mut v_message: Py) -> R {
    if (py::not(&v_cond.clone())).truthy() {
        return Err(py::value_error(py::to_str(&v_message.clone())));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__p_ok(mut v_p: Py, mut v_lo: Py, mut v_hi: Py, mut v_name: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("p"), v_p.clone())]))?;
    let _ = f__need({ let t1433 = v_lo.clone(); Py::Bool({ let t1434 = v_p.clone(); if py::le(&t1433, &t1434)? { { let t1435 = v_hi.clone(); if py::le(&t1434, &t1435)? { true } else { false } } } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must lie between ".to_string(), py::format(&py::truediv(&v_lo.clone(), &Py::Int(100i64))?, "g")?, " and ".to_string(), py::format(&py::truediv(&v_hi.clone(), &Py::Int(100i64))?, "g")?, " hPa".to_string()].concat()))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ptmp(mut v_s: Py, mut v_t: Py, mut v_p: Py, mut v_pr: Py) -> R {
    let mut v_dp: Py = Py::None;
    let mut v_dth: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_th: Py = Py::None;
    v_r2 = py::math1("sqrt", &Py::Float(2.0))?;
    v_dp = py::sub(&v_pr.clone(), &v_p.clone())?;
    v_dth = py::mul(&v_dp.clone(), &f__adtg(v_s.clone(), v_t.clone(), v_p.clone())?)?;
    v_th = py::add(&f__t68(v_t.clone())?, &py::mul(&Py::Float(0.5), &v_dth.clone())?)?;
    v_q = v_dth.clone();
    v_dth = py::mul(&v_dp.clone(), &f__adtg(v_s.clone(), py::truediv(&v_th.clone(), &Py::Float(1.00024))?, py::add(&v_p.clone(), &py::mul(&Py::Float(0.5), &v_dp.clone())?)?)?)?;
    v_th = py::add(&v_th.clone(), &py::mul(&py::sub(&Py::Int(1i64), &py::truediv(&Py::Int(1i64), &v_r2.clone())?)?, &py::sub(&v_dth.clone(), &v_q.clone())?)?)?;
    v_q = py::add(&py::mul(&py::sub(&Py::Int(2i64), &v_r2.clone())?, &v_dth.clone())?, &py::mul(&py::add(&py::neg(&Py::Int(2i64))?, &py::truediv(&Py::Int(3i64), &v_r2.clone())?)?, &v_q.clone())?)?;
    v_dth = py::mul(&v_dp.clone(), &f__adtg(v_s.clone(), py::truediv(&v_th.clone(), &Py::Float(1.00024))?, py::add(&v_p.clone(), &py::mul(&Py::Float(0.5), &v_dp.clone())?)?)?)?;
    v_th = py::add(&v_th.clone(), &py::mul(&py::add(&Py::Int(1i64), &py::truediv(&Py::Int(1i64), &v_r2.clone())?)?, &py::sub(&v_dth.clone(), &v_q.clone())?)?)?;
    v_q = py::add(&py::mul(&py::add(&Py::Int(2i64), &v_r2.clone())?, &v_dth.clone())?, &py::mul(&py::sub(&py::neg(&Py::Int(2i64))?, &py::truediv(&Py::Int(3i64), &v_r2.clone())?)?, &v_q.clone())?)?;
    v_dth = py::mul(&v_dp.clone(), &f__adtg(v_s.clone(), py::truediv(&v_th.clone(), &Py::Float(1.00024))?, py::add(&v_p.clone(), &v_dp.clone())?)?)?;
    return Ok(py::truediv(&py::add(&v_th.clone(), &py::truediv(&py::sub(&v_dth.clone(), &py::mul(&Py::Int(2i64), &v_q.clone())?)?, &Py::Int(6i64))?)?, &Py::Float(1.00024))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    v_x = py::float(&v_x.clone())?;
    if ({ let t1538 = { let t1536 = v_x.clone(); Py::Bool({ let t1537 = Py::Float(0.0); if py::eq(&t1536, &t1537) { true } else { false } }) }; if t1538.truthy() { t1538 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t1539 = v_x.clone(); Py::Bool({ let t1540 = Py::Float(0.0); if py::eq(&t1539, &t1540) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sea_ok(mut v_S: Py, mut v_T: Py, mut v_p: Py) -> R {
    if ({ let t1306 = v_S.clone(); Py::Bool({ let t1307 = Py::None; if !py::is(&t1306, &t1307) { true } else { false } }) }).truthy() {
        let _ = f__finite(Py::dict(vec![(Py::str("S"), v_S.clone())]))?;
        let _ = f__need({ let t1312 = Py::Int(0i64); Py::Bool({ let t1313 = v_S.clone(); if py::le(&t1312, &t1313)? { { let t1314 = Py::Int(42i64); if py::le(&t1313, &t1314)? { true } else { false } } } else { false } }) }, Py::str("practical salinity must be between 0 and 42"))?;
    }
    if ({ let t1315 = v_T.clone(); Py::Bool({ let t1316 = Py::None; if !py::is(&t1315, &t1316) { true } else { false } }) }).truthy() {
        let _ = f__finite(Py::dict(vec![(Py::str("T"), v_T.clone())]))?;
        let _ = f__need({ let t1317 = Py::Float(271.15); Py::Bool({ let t1318 = v_T.clone(); if py::le(&t1317, &t1318)? { { let t1319 = Py::Float(313.15); if py::le(&t1318, &t1319)? { true } else { false } } } else { false } }) }, Py::str("temperature must be between -2 and 40 °C"))?;
    }
    if ({ let t1320 = v_p.clone(); Py::Bool({ let t1321 = Py::None; if !py::is(&t1320, &t1321) { true } else { false } }) }).truthy() {
        let _ = f__finite(Py::dict(vec![(Py::str("p"), v_p.clone())]))?;
        let _ = f__need({ let t1322 = Py::Int(0i64); Py::Bool({ let t1323 = v_p.clone(); if py::le(&t1322, &t1323)? { { let t1324 = Py::Float(100000000.0); if py::le(&t1323, &t1324)? { true } else { false } } } else { false } }) }, Py::str("sea pressure must be between 0 and 10,000 dbar"))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__seck(mut v_s: Py, mut v_t: Py, mut v_p: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_aw: Py = Py::None;
    let mut v_bw: Py = Py::None;
    let mut v_k0: Py = Py::None;
    let mut v_kw: Py = Py::None;
    let t1149 = (Py::tuple(vec![py::truediv(&v_p.clone(), &Py::Float(10.0))?, f__t68(v_t.clone())?])).items()?;
    if t1149.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1149[0].clone();
    v_T = t1149[1].clone();
    v_aw = py::add(&Py::Float(3.239908), &py::mul(&py::add(&Py::Float(0.00143713), &py::mul(&py::sub(&Py::Float(0.000116092), &py::mul(&Py::Float(5.77905e-07), &v_T.clone())?)?, &v_T.clone())?)?, &v_T.clone())?)?;
    v_bw = py::add(&Py::Float(8.50935e-05), &py::mul(&py::add(&py::neg(&Py::Float(6.12293e-06))?, &py::mul(&Py::Float(5.2787e-08), &v_T.clone())?)?, &v_T.clone())?)?;
    v_kw = py::add(&Py::Float(19652.21), &py::mul(&py::add(&Py::Float(148.4206), &py::mul(&py::add(&py::neg(&Py::Float(2.327105))?, &py::mul(&py::sub(&Py::Float(0.01360477), &py::mul(&Py::Float(5.155288e-05), &v_T.clone())?)?, &v_T.clone())?)?, &v_T.clone())?)?, &v_T.clone())?)?;
    v_A = py::add(&v_aw.clone(), &py::mul(&py::add(&py::add(&Py::Float(0.0022838), &py::mul(&py::sub(&py::neg(&Py::Float(1.0981e-05))?, &py::mul(&Py::Float(1.6078e-06), &v_T.clone())?)?, &v_T.clone())?)?, &py::mul(&Py::Float(0.000191075), &py::math1("sqrt", &v_s.clone())?)?)?, &v_s.clone())?)?;
    v_B = py::add(&v_bw.clone(), &py::mul(&py::add(&py::neg(&Py::Float(9.9348e-07))?, &py::mul(&py::add(&Py::Float(2.0816e-08), &py::mul(&Py::Float(9.1697e-10), &v_T.clone())?)?, &v_T.clone())?)?, &v_s.clone())?)?;
    v_k0 = py::add(&v_kw.clone(), &py::mul(&py::add(&py::add(&Py::Float(54.6746), &py::mul(&py::add(&py::neg(&Py::Float(0.603459))?, &py::mul(&py::sub(&Py::Float(0.0109987), &py::mul(&Py::Float(6.167e-05), &v_T.clone())?)?, &v_T.clone())?)?, &v_T.clone())?)?, &py::mul(&py::add(&Py::Float(0.07944), &py::mul(&py::sub(&Py::Float(0.016483), &py::mul(&Py::Float(0.00053009), &v_T.clone())?)?, &v_T.clone())?)?, &py::math1("sqrt", &v_s.clone())?)?)?, &v_s.clone())?)?;
    return Ok(py::add(&v_k0.clone(), &py::mul(&py::add(&v_A.clone(), &py::mul(&v_B.clone(), &v_P.clone())?)?, &v_P.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__smow(mut v_t: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_a: Py = Py::None;
    v_T = f__t68(v_t.clone())?;
    v_a = Py::tuple(vec![Py::Float(999.842594), Py::Float(0.06793952), py::neg(&Py::Float(0.00909529))?, Py::Float(0.0001001685), py::neg(&Py::Float(1.120083e-06))?, Py::Float(6.536332e-09)]);
    return Ok(py::add(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::mul(&py::add(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::mul(&py::add(&py::getitem(&v_a.clone(), &Py::Int(2i64))?, &py::mul(&py::add(&py::getitem(&v_a.clone(), &Py::Int(3i64))?, &py::mul(&py::add(&py::getitem(&v_a.clone(), &Py::Int(4i64))?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(5i64))?, &v_T.clone())?)?, &v_T.clone())?)?, &v_T.clone())?)?, &v_T.clone())?)?, &v_T.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__svel(mut v_s: Py, mut v_t: Py, mut v_p: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_cw: Py = Py::None;
    let mut v_d: Py = Py::None;
    let t1330 = (Py::tuple(vec![py::truediv(&v_p.clone(), &Py::Float(10.0))?, f__t68(v_t.clone())?])).items()?;
    if t1330.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1330[0].clone();
    v_T = t1330[1].clone();
    v_cw = py::add(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::sub(&py::mul(&py::add(&py::mul(&py::neg(&Py::Float(2.3643e-12))?, &v_T.clone())?, &Py::Float(3.8504e-10))?, &v_T.clone())?, &Py::Float(9.7729e-09))?, &v_P.clone())?, &py::add(&py::mul(&py::sub(&py::mul(&py::add(&py::mul(&py::sub(&py::mul(&Py::Float(1.0405e-12), &v_T.clone())?, &Py::Float(2.5335e-10))?, &v_T.clone())?, &Py::Float(2.5974e-08))?, &v_T.clone())?, &Py::Float(1.7107e-06))?, &v_T.clone())?, &Py::Float(3.126e-05))?)?, &v_P.clone())?, &py::add(&py::mul(&py::add(&py::mul(&py::sub(&py::mul(&py::add(&py::mul(&py::neg(&Py::Float(6.1185e-10))?, &v_T.clone())?, &Py::Float(1.3621e-07))?, &v_T.clone())?, &Py::Float(8.1788e-06))?, &v_T.clone())?, &Py::Float(0.00068982))?, &v_T.clone())?, &Py::Float(0.153563))?)?, &v_P.clone())?, &py::mul(&py::add(&py::mul(&py::sub(&py::mul(&py::add(&py::mul(&py::sub(&py::mul(&Py::Float(3.1464e-09), &v_T.clone())?, &Py::Float(1.478e-06))?, &v_T.clone())?, &Py::Float(0.0003342))?, &v_T.clone())?, &Py::Float(0.0580852))?, &v_T.clone())?, &Py::Float(5.03711))?, &v_T.clone())?)?, &Py::Float(1402.388))?;
    v_a = py::add(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::neg(&Py::Float(3.389e-13))?, &v_T.clone())?, &Py::Float(6.649e-12))?, &v_T.clone())?, &Py::Float(1.1e-10))?, &v_P.clone())?, &py::sub(&py::mul(&py::add(&py::mul(&py::sub(&py::mul(&Py::Float(7.988e-12), &v_T.clone())?, &Py::Float(1.6002e-10))?, &v_T.clone())?, &Py::Float(9.1041e-09))?, &v_T.clone())?, &Py::Float(3.9064e-07))?)?, &v_P.clone())?, &py::add(&py::mul(&py::sub(&py::mul(&py::sub(&py::mul(&py::add(&py::mul(&py::neg(&Py::Float(2.0122e-10))?, &v_T.clone())?, &Py::Float(1.0507e-08))?, &v_T.clone())?, &Py::Float(6.4885e-08))?, &v_T.clone())?, &Py::Float(1.258e-05))?, &v_T.clone())?, &Py::Float(9.4742e-05))?)?, &v_P.clone())?, &py::mul(&py::sub(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::neg(&Py::Float(3.21e-08))?, &v_T.clone())?, &Py::Float(2.006e-06))?, &v_T.clone())?, &Py::Float(7.164e-05))?, &v_T.clone())?, &Py::Float(0.01262))?, &v_T.clone())?)?, &Py::Float(1.389))?;
    v_b = py::add(&py::sub(&py::neg(&Py::Float(0.01922))?, &py::mul(&Py::Float(4.42e-05), &v_T.clone())?)?, &py::mul(&py::add(&Py::Float(7.3637e-05), &py::mul(&Py::Float(1.7945e-07), &v_T.clone())?)?, &v_P.clone())?)?;
    v_d = py::sub(&Py::Float(0.001727), &py::mul(&Py::Float(7.9836e-06), &v_P.clone())?)?;
    return Ok(py::add(&py::add(&py::add(&v_cw.clone(), &py::mul(&v_a.clone(), &v_s.clone())?)?, &py::mul(&py::mul(&v_b.clone(), &v_s.clone())?, &py::math1("sqrt", &v_s.clone())?)?)?, &py::mul(&py::mul(&v_d.clone(), &v_s.clone())?, &v_s.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__t68(mut v_t: Py) -> R {
    return Ok(py::mul(&v_t.clone(), &Py::Float(1.00024))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__theta_e(mut v_p: Py, mut v_T: Py, mut v_Td: Py, mut v_use_partial: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_thl: Py = Py::None;
    let mut v_tl: Py = Py::None;
    v_r = f__ws(v_p.clone(), v_Td.clone())?;
    v_e = f__es(v_Td.clone())?;
    v_tl = py::add(&Py::Int(56i64), &py::truediv(&Py::Float(1.0), &py::add(&py::truediv(&Py::Float(1.0), &py::sub(&v_Td.clone(), &Py::Int(56i64))?)?, &py::truediv(&py::math1("log", &py::truediv(&v_T.clone(), &v_Td.clone())?)?, &Py::Float(800.0))?)?)?)?;
    v_thl = py::mul(&py::mul(&v_T.clone(), &py::pow(&py::truediv(&c_P0()?, &(if (v_use_partial.clone()).truthy() { py::sub(&v_p.clone(), &v_e.clone())? } else { v_p.clone() }))?, &c_KAPPA()?)?)?, &py::pow(&py::truediv(&v_T.clone(), &v_tl.clone())?, &py::mul(&Py::Float(0.28), &v_r.clone())?)?)?;
    return Ok(py::mul(&v_thl.clone(), &py::math1("exp", &py::mul(&py::mul(&v_r.clone(), &py::add(&Py::Int(1i64), &py::mul(&Py::Float(0.448), &v_r.clone())?)?)?, &py::sub(&py::truediv(&Py::Float(3036.0), &v_tl.clone())?, &Py::Float(1.78))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ws(mut v_p: Py, mut v_T: Py) -> R {
    let mut v_e: Py = Py::None;
    v_e = f__es(v_T.clone())?;
    let _ = f__need({ let t1355 = v_e.clone(); Py::Bool({ let t1356 = v_p.clone(); if py::lt(&t1355, &t1356)? { true } else { false } }) }, Py::str("the saturation vapour pressure must be below the total pressure"))?;
    return Ok(py::truediv(&py::mul(&c_EPS()?, &v_e.clone())?, &py::sub(&v_p.clone(), &v_e.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_airy_root(mut v_h: Py, mut v_rho_c: Py, mut v_rho_m: Py, mut v_tc: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_r: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("h"), v_h.clone()), (Py::str("rho_c"), v_rho_c.clone()), (Py::str("rho_m"), v_rho_m.clone())]))?;
    let _ = f__need({ let t5 = v_h.clone(); Py::Bool({ let t6 = Py::Int(0i64); if py::gt(&t5, &t6)? { true } else { false } }) }, Py::str("the elevation must be positive"))?;
    let _ = f__need({ let t7 = v_rho_m.clone(); Py::Bool({ let t8 = v_rho_c.clone(); if py::gt(&t7, &t8)? { { let t9 = Py::Int(0i64); if py::gt(&t8, &t9)? { true } else { false } } } else { false } }) }, Py::str("need rho_m > rho_c > 0"))?;
    v_r = py::truediv(&py::mul(&v_h.clone(), &v_rho_c.clone())?, &py::sub(&v_rho_m.clone(), &v_rho_c.clone())?)?;
    v_out = Py::dict(vec![(Py::str("root"), f__r(v_r.clone())?), (Py::str("root_km"), f__r(py::truediv(&v_r.clone(), &Py::Float(1000.0))?)?)]);
    if ({ let t15 = v_tc.clone(); Py::Bool({ let t16 = Py::None; if !py::is(&t15, &t16) { true } else { false } }) }).truthy() {
        let _ = f__finite(Py::dict(vec![(Py::str("tc"), v_tc.clone())]))?;
        let _ = f__need({ let t17 = v_tc.clone(); Py::Bool({ let t18 = Py::Int(0i64); if py::gt(&t17, &t18)? { true } else { false } }) }, Py::str("the normal crustal thickness must be positive"))?;
        py::setitem(&v_out.clone(), &Py::str("crust_thickness"), f__r(py::add(&py::add(&v_tc.clone(), &v_h.clone())?, &v_r.clone())?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_b_value(mut v_mags: Py, mut v_Mc: Py, mut v_dM: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_mean: Py = Py::None;
    let mut v_sel: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("Mc"), v_Mc.clone()), (Py::str("dM"), v_dM.clone())]))?;
    let _ = f__need({ let t492 = Py::Int(0i64); Py::Bool({ let t493 = v_dM.clone(); if py::le(&t492, &t493)? { { let t494 = Py::Float(0.5); if py::le(&t493, &t494)? { true } else { false } } } else { false } }) }, Py::str("the magnitude bin width must be between 0 and 0.5"))?;
    v_sel = { let mut out = vec![]; for t495 in py::iter(&(v_mags.clone()))? { let mut v_m: Py; 
v_m = t495;
if ({ let t496 = v_m.clone(); Py::Bool({ let t497 = py::sub(&v_Mc.clone(), &Py::Float(1e-09))?; if py::ge(&t496, &t497)? { true } else { false } }) }).truthy() { out.push(v_m.clone()); }} Py::list(out) };
    let _ = f__need({ let t498 = py::len(&v_sel.clone())?; Py::Bool({ let t499 = Py::Int(10i64); if py::ge(&t498, &t499)? { true } else { false } }) }, Py::str("at least 10 magnitudes at or above the completeness magnitude are needed"))?;
    v_mean = py::truediv(&py::sum(&v_sel.clone(), None)?, &py::len(&v_sel.clone())?)?;
    v_den = py::sub(&v_mean.clone(), &py::sub(&v_Mc.clone(), &py::truediv(&v_dM.clone(), &Py::Int(2i64))?)?)?;
    let _ = f__need({ let t500 = v_den.clone(); Py::Bool({ let t501 = Py::Int(0i64); if py::gt(&t500, &t501)? { true } else { false } }) }, Py::str("the mean magnitude must exceed Mc - dM/2"))?;
    v_b = py::truediv(&py::math1("log10", &Py::Float(std::f64::consts::E))?, &v_den.clone())?;
    return Ok(Py::dict(vec![(Py::str("b"), f__r(v_b.clone())?), (Py::str("a"), f__r(py::add(&py::math1("log10", &py::len(&v_sel.clone())?)?, &py::mul(&v_b.clone(), &v_Mc.clone())?)?)?), (Py::str("n"), py::float(&py::len(&v_sel.clone())?)?), (Py::str("mean"), f__r(v_mean.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_blackbody_flux(mut v_T: Py, mut v_eps: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("T"), v_T.clone()), (Py::str("eps"), v_eps.clone())]))?;
    let _ = f__need({ let t23 = Py::Int(1i64); Py::Bool({ let t24 = v_T.clone(); if py::le(&t23, &t24)? { { let t25 = Py::Float(100000.0); if py::le(&t24, &t25)? { true } else { false } } } else { false } }) }, Py::str("temperature must be between 1 K and 100,000 K"))?;
    let _ = f__need({ let t26 = Py::Int(0i64); Py::Bool({ let t27 = v_eps.clone(); if py::lt(&t26, &t27)? { { let t28 = Py::Int(1i64); if py::le(&t27, &t28)? { true } else { false } } } else { false } }) }, Py::str("emissivity must be in (0, 1]"))?;
    return Ok(Py::dict(vec![(Py::str("F"), f__r(py::mul(&py::mul(&v_eps.clone(), &c_SIGMA()?)?, &py::pow(&v_T.clone(), &Py::Int(4i64))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bouguer_slab(mut v_drho: Py, mut v_h: Py) -> R {
    let mut v_g: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("drho"), v_drho.clone()), (Py::str("h"), v_h.clone())]))?;
    let _ = f__need({ let t42 = { let t38 = v_drho.clone(); Py::Bool({ let t39 = Py::Int(0i64); if py::ne(&t38, &t39) { true } else { false } }) }; if !t42.truthy() { t42 } else { { let t40 = py::abs(&v_drho.clone())?; Py::Bool({ let t41 = Py::Int(5000i64); if py::le(&t40, &t41)? { true } else { false } }) } } }, Py::str("the density contrast must be non-zero and at most 5000 kg/m^3"))?;
    let _ = f__need({ let t43 = Py::Int(0i64); Py::Bool({ let t44 = v_h.clone(); if py::lt(&t43, &t44)? { { let t45 = Py::Int(50000i64); if py::le(&t44, &t45)? { true } else { false } } } else { false } }) }, Py::str("the slab thickness must be between 0 and 50 km"))?;
    v_g = py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &c_G_NEWTON()?)?, &v_drho.clone())?, &v_h.clone())?;
    return Ok(Py::dict(vec![(Py::str("dg_mGal"), f__r(py::mul(&v_g.clone(), &Py::Float(100000.0))?)?), (Py::str("dg"), f__r(v_g.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_brunt_vaisala(mut v_theta: Py, mut v_dtheta_dz: Py) -> R {
    let mut v_n: Py = Py::None;
    let _ = f__T_ok(v_theta.clone(), Py::Int(200i64), Py::Int(600i64), Py::str("potential temperature"))?;
    let _ = f__finite(Py::dict(vec![(Py::str("dtheta_dz"), v_dtheta_dz.clone())]))?;
    let _ = f__need({ let t58 = Py::Int(0i64); Py::Bool({ let t59 = v_dtheta_dz.clone(); if py::lt(&t58, &t59)? { { let t60 = Py::Float(0.1); if py::le(&t59, &t60)? { true } else { false } } } else { false } }) }, Py::str("the potential-temperature gradient must be positive (stable) and at most 0.1 K/m"))?;
    v_n = py::math1("sqrt", &py::mul(&py::truediv(&c_G0()?, &v_theta.clone())?, &v_dtheta_dz.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("N"), f__r(v_n.clone())?), (Py::str("period"), f__r(py::truediv(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_n.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_buried_sphere(mut v_drho: Py, mut v_R: Py, mut v_z: Py) -> R {
    let mut v_g: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("drho"), v_drho.clone()), (Py::str("R"), v_R.clone()), (Py::str("z"), v_z.clone())]))?;
    let _ = f__need({ let t74 = { let t70 = v_drho.clone(); Py::Bool({ let t71 = Py::Int(0i64); if py::ne(&t70, &t71) { true } else { false } }) }; if !t74.truthy() { t74 } else { { let t72 = py::abs(&v_drho.clone())?; Py::Bool({ let t73 = Py::Int(10000i64); if py::le(&t72, &t73)? { true } else { false } }) } } }, Py::str("the density contrast must be non-zero and at most 10000 kg/m^3"))?;
    let _ = f__need({ let t75 = Py::Int(0i64); Py::Bool({ let t76 = v_R.clone(); if py::lt(&t75, &t76)? { { let t77 = v_z.clone(); if py::lt(&t76, &t77)? { { let t78 = Py::Float(1000000.0); if py::le(&t77, &t78)? { true } else { false } } } else { false } } } else { false } }) }, Py::str("the sphere must lie wholly below the surface (0 < R < depth of its centre)"))?;
    v_g = py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&py::truediv(&Py::Int(4i64), &Py::Int(3i64))?, &Py::Float(std::f64::consts::PI))?, &c_G_NEWTON()?)?, &v_drho.clone())?, &py::pow(&v_R.clone(), &Py::Int(3i64))?)?, &py::mul(&v_z.clone(), &v_z.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("dg_mGal"), f__r(py::mul(&v_g.clone(), &Py::Float(100000.0))?)?), (Py::str("dg"), f__r(v_g.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_co2_forcing(mut v_C: Py, mut v_C0: Py, mut v_lam: Py) -> R {
    let mut v_dF: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("C0"), v_C0.clone())]))?;
    let _ = f__need({ let t94 = { let t88 = Py::Int(1i64); Py::Bool({ let t89 = v_C.clone(); if py::le(&t88, &t89)? { { let t90 = Py::Float(100000.0); if py::le(&t89, &t90)? { true } else { false } } } else { false } }) }; if !t94.truthy() { t94 } else { { let t91 = Py::Int(1i64); Py::Bool({ let t92 = v_C0.clone(); if py::le(&t91, &t92)? { { let t93 = Py::Float(100000.0); if py::le(&t92, &t93)? { true } else { false } } } else { false } }) } } }, Py::str("CO2 concentrations must be between 1 and 100,000 ppm"))?;
    v_dF = py::mul(&Py::Float(5.35), &py::math1("log", &py::truediv(&v_C.clone(), &v_C0.clone())?)?)?;
    v_out = Py::dict(vec![(Py::str("dF"), f__r(v_dF.clone())?), (Py::str("doublings"), f__r(py::math1("log2", &py::truediv(&v_C.clone(), &v_C0.clone())?)?)?)]);
    if ({ let t100 = v_lam.clone(); Py::Bool({ let t101 = Py::None; if !py::is(&t100, &t101) { true } else { false } }) }).truthy() {
        let _ = f__finite(Py::dict(vec![(Py::str("lam"), v_lam.clone())]))?;
        let _ = f__need({ let t102 = v_lam.clone(); Py::Bool({ let t103 = Py::Int(0i64); if py::gt(&t102, &t103)? { true } else { false } }) }, Py::str("the climate sensitivity parameter must be positive"))?;
        py::setitem(&v_out.clone(), &Py::str("dT_eq"), f__r(py::mul(&v_lam.clone(), &v_dF.clone())?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_coriolis(mut v_lat: Py) -> R {
    let _ = f__lat_ok(v_lat.clone(), Py::Float(90.0), Py::str("latitude"))?;
    return Ok(Py::dict(vec![(Py::str("f"), f__r(py::mul(&py::mul(&Py::Int(2i64), &c_OMEGA()?)?, &py::math1("sin", &py::math1("radians", &v_lat.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_daily_insolation(mut v_lat: Py, mut v_decl: Py, mut v_S0: Py) -> R {
    let mut v_de: Py = Py::None;
    let mut v_h0: Py = Py::None;
    let mut v_ph: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__lat_ok(v_lat.clone(), Py::Float(89.9), Py::str("latitude"))?;
    let _ = f__finite(Py::dict(vec![(Py::str("decl"), v_decl.clone()), (Py::str("S0"), v_S0.clone())]))?;
    let _ = f__need({ let t123 = py::neg(&Py::Float(23.5))?; Py::Bool({ let t124 = v_decl.clone(); if py::le(&t123, &t124)? { { let t125 = Py::Float(23.5); if py::le(&t124, &t125)? { true } else { false } } } else { false } }) }, Py::str("the solar declination must be between -23.5 and 23.5 degrees"))?;
    let _ = f__need({ let t126 = v_S0.clone(); Py::Bool({ let t127 = Py::Int(0i64); if py::gt(&t126, &t127)? { true } else { false } }) }, Py::str("the solar constant must be positive"))?;
    let t128 = (Py::tuple(vec![py::math1("radians", &v_lat.clone())?, py::math1("radians", &v_decl.clone())?])).items()?;
    if t128.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ph = t128[0].clone();
    v_de = t128[1].clone();
    v_x = py::mul(&py::neg(&py::math1("tan", &v_ph.clone())?)?, &py::math1("tan", &v_de.clone())?)?;
    v_h0 = (if ({ let t129 = v_x.clone(); Py::Bool({ let t130 = Py::Int(1i64); if py::ge(&t129, &t130)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { (if ({ let t131 = v_x.clone(); Py::Bool({ let t132 = py::neg(&Py::Int(1i64))?; if py::le(&t131, &t132)? { true } else { false } }) }).truthy() { Py::Float(std::f64::consts::PI) } else { py::math1("acos", &v_x.clone())? }) });
    v_q = py::mul(&py::truediv(&v_S0.clone(), &Py::Float(std::f64::consts::PI))?, &py::add(&py::mul(&py::mul(&v_h0.clone(), &py::math1("sin", &v_ph.clone())?)?, &py::math1("sin", &v_de.clone())?)?, &py::mul(&py::mul(&py::math1("cos", &v_ph.clone())?, &py::math1("cos", &v_de.clone())?)?, &py::math1("sin", &v_h0.clone())?)?)?)?;
    v_q = (if ({ let t133 = py::abs(&v_q.clone())?; Py::Bool({ let t134 = py::mul(&Py::Float(1e-12), &v_S0.clone())?; if py::le(&t133, &t134)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_q.clone() });
    return Ok(Py::dict(vec![(Py::str("Q"), f__r(v_q.clone())?), (Py::str("day_length_h"), f__r(py::truediv(&py::mul(&Py::Int(24i64), &v_h0.clone())?, &Py::Float(std::f64::consts::PI))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_darcy_flow(mut v_K: Py, mut v_A: Py, mut v_dh: Py, mut v_L: Py, mut v_n: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_q: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("K"), v_K.clone()), (Py::str("A"), v_A.clone()), (Py::str("dh"), v_dh.clone()), (Py::str("L"), v_L.clone())]))?;
    let _ = f__need({ let t151 = { let t144 = v_K.clone(); Py::Bool({ let t145 = Py::Int(0i64); if py::gt(&t144, &t145)? { true } else { false } }) }; if !t151.truthy() { t151 } else { { let t150 = { let t146 = v_A.clone(); Py::Bool({ let t147 = Py::Int(0i64); if py::gt(&t146, &t147)? { true } else { false } }) }; if !t150.truthy() { t150 } else { { let t148 = v_L.clone(); Py::Bool({ let t149 = Py::Int(0i64); if py::gt(&t148, &t149)? { true } else { false } }) } } } } }, Py::str("K, A and L must be positive"))?;
    let _ = f__need({ let t152 = v_dh.clone(); Py::Bool({ let t153 = Py::Int(0i64); if py::gt(&t152, &t153)? { true } else { false } }) }, Py::str("the head difference must be positive (flow from high to low head)"))?;
    v_q = py::truediv(&py::mul(&v_K.clone(), &v_dh.clone())?, &v_L.clone())?;
    v_out = Py::dict(vec![(Py::str("Q"), f__r(py::mul(&v_q.clone(), &v_A.clone())?)?), (Py::str("q"), f__r(v_q.clone())?), (Py::str("gradient"), f__r(py::truediv(&v_dh.clone(), &v_L.clone())?)?)]);
    if ({ let t159 = v_n.clone(); Py::Bool({ let t160 = Py::None; if !py::is(&t159, &t160) { true } else { false } }) }).truthy() {
        let _ = f__finite(Py::dict(vec![(Py::str("n"), v_n.clone())]))?;
        let _ = f__need({ let t161 = Py::Int(0i64); Py::Bool({ let t162 = v_n.clone(); if py::lt(&t161, &t162)? { { let t163 = Py::Int(1i64); if py::le(&t162, &t163)? { true } else { false } } } else { false } }) }, Py::str("the porosity must be between 0 and 1"))?;
        py::setitem(&v_out.clone(), &Py::str("v"), f__r(py::truediv(&v_q.clone(), &v_n.clone())?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_decay_remaining(mut v_half_life: Py, mut v_t: Py) -> R {
    let mut v_frac: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("half_life"), v_half_life.clone()), (Py::str("t"), v_t.clone())]))?;
    let _ = f__need({ let t952 = v_half_life.clone(); Py::Bool({ let t953 = Py::Int(0i64); if py::gt(&t952, &t953)? { true } else { false } }) }, Py::str("the half-life must be positive"))?;
    let _ = f__need({ let t954 = Py::Int(0i64); Py::Bool({ let t955 = v_t.clone(); if py::le(&t954, &t955)? { { let t956 = py::mul(&Py::Int(60i64), &v_half_life.clone())?; if py::le(&t955, &t956)? { true } else { false } } } else { false } }) }, Py::str("time must be between 0 and 60 half-lives"))?;
    v_lam = py::truediv(&py::math1("log", &Py::Int(2i64))?, &v_half_life.clone())?;
    v_frac = py::pow(&Py::Float(2.0), &py::truediv(&py::neg(&v_t.clone())?, &v_half_life.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("fraction"), f__r(v_frac.clone())?), (Py::str("decayed"), f__r(py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_lam.clone())?, &v_t.clone())?)?)?)?), (Py::str("lambda"), f__r(v_lam.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dewpoint_from_rh(mut v_T: Py, mut v_RH: Py) -> R {
    let _ = f__T_ok(v_T.clone(), Py::Float(228.15), Py::Float(333.15), Py::str("temperature"))?;
    let _ = f__finite(Py::dict(vec![(Py::str("RH"), v_RH.clone())]))?;
    let _ = f__need({ let t171 = Py::Float(0.01); Py::Bool({ let t172 = v_RH.clone(); if py::le(&t171, &t172)? { { let t173 = Py::Float(1.0); if py::le(&t172, &t173)? { true } else { false } } } else { false } }) }, Py::str("relative humidity must be a fraction between 0.01 and 1 (1% to 100%)"))?;
    return Ok(Py::dict(vec![(Py::str("Td"), f__r(f__dewpoint_bolton(py::mul(&v_RH.clone(), &f__es(v_T.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dewpoint_from_vapor_pressure(mut v_e: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("e"), v_e.clone())]))?;
    let _ = f__need({ let t183 = Py::Int(10i64); Py::Bool({ let t184 = v_e.clone(); if py::le(&t183, &t184)? { { let t185 = Py::Int(20000i64); if py::le(&t184, &t185)? { true } else { false } } } else { false } }) }, Py::str("the vapour pressure must lie between 0.1 and 200 hPa"))?;
    return Ok(Py::dict(vec![(Py::str("Td"), f__r(f__dewpoint_bolton(v_e.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dipole_field(mut v_lat: Py) -> R {
    let mut v_ph: Py = Py::None;
    let _ = f__lat_ok(v_lat.clone(), Py::Float(89.999), Py::str("magnetic latitude"))?;
    v_ph = py::math1("radians", &v_lat.clone())?;
    return Ok(Py::dict(vec![(Py::str("inclination"), f__r(py::math1("degrees", &py::math1("atan", &py::mul(&Py::Int(2i64), &py::math1("tan", &v_ph.clone())?)?)?)?)?), (Py::str("F_ratio"), f__r(py::math1("sqrt", &py::add(&Py::Int(1i64), &py::mul(&Py::Int(3i64), &py::pow(&py::math1("sin", &v_ph.clone())?, &Py::Int(2i64))?)?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dry_adiabatic_ascent(mut v_T: Py, mut v_dz: Py) -> R {
    let mut v_T2: Py = Py::None;
    let mut v_gam: Py = Py::None;
    let _ = f__T_ok(v_T.clone(), Py::Int(150i64), Py::Int(350i64), Py::str("temperature"))?;
    let _ = f__finite(Py::dict(vec![(Py::str("dz"), v_dz.clone())]))?;
    let _ = f__need({ let t210 = Py::Int(0i64); Py::Bool({ let t211 = v_dz.clone(); if py::lt(&t210, &t211)? { { let t212 = Py::Int(20000i64); if py::le(&t211, &t212)? { true } else { false } } } else { false } }) }, Py::str("the ascent dz must be between 0 and 20 km"))?;
    v_gam = py::truediv(&c_G0()?, &c_CPD()?)?;
    v_T2 = py::sub(&v_T.clone(), &py::mul(&v_gam.clone(), &v_dz.clone())?)?;
    let _ = f__need({ let t213 = v_T2.clone(); Py::Bool({ let t214 = Py::Int(100i64); if py::gt(&t213, &t214)? { true } else { false } }) }, Py::str("the parcel would cool below 100 K"))?;
    return Ok(Py::dict(vec![(Py::str("T2"), f__r(v_T2.clone())?), (Py::str("lapse_rate"), f__r(v_gam.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ekman_depth(mut v_Az: Py, mut v_lat: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_f: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("Az"), v_Az.clone())]))?;
    let _ = f__lat_ok(v_lat.clone(), Py::Float(90.0), Py::str("latitude"))?;
    let _ = f__need({ let t227 = Py::Int(5i64); Py::Bool({ let t228 = py::abs(&v_lat.clone())?; if py::le(&t227, &t228)? { { let t229 = Py::Int(89i64); if py::le(&t228, &t229)? { true } else { false } } } else { false } }) }, Py::str("Ekman layers are used here for 5 to 89 degrees latitude"))?;
    let _ = f__need({ let t230 = Py::Float(1e-05); Py::Bool({ let t231 = v_Az.clone(); if py::le(&t230, &t231)? { { let t232 = Py::Int(10i64); if py::le(&t231, &t232)? { true } else { false } } } else { false } }) }, Py::str("the vertical eddy viscosity must be between 1e-5 and 10 m^2/s"))?;
    v_f = py::abs(&py::mul(&py::mul(&Py::Int(2i64), &c_OMEGA()?)?, &py::math1("sin", &py::math1("radians", &v_lat.clone())?)?)?)?;
    v_d = py::math1("sqrt", &py::truediv(&py::mul(&Py::Int(2i64), &v_Az.clone())?, &v_f.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("D_E"), f__r(py::mul(&Py::Float(std::f64::consts::PI), &v_d.clone())?)?), (Py::str("delta"), f__r(v_d.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ekman_transport(mut v_tau: Py, mut v_lat: Py, mut v_rho: Py) -> R {
    let mut v_f: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("tau"), v_tau.clone()), (Py::str("rho"), v_rho.clone())]))?;
    let _ = f__lat_ok(v_lat.clone(), Py::Float(90.0), Py::str("latitude"))?;
    let _ = f__need({ let t245 = Py::Int(5i64); Py::Bool({ let t246 = py::abs(&v_lat.clone())?; if py::le(&t245, &t246)? { { let t247 = Py::Int(89i64); if py::le(&t246, &t247)? { true } else { false } } } else { false } }) }, Py::str("Ekman transport is used here for 5 to 89 degrees latitude"))?;
    let _ = f__need({ let t248 = v_tau.clone(); Py::Bool({ let t249 = Py::Int(0i64); if py::gt(&t248, &t249)? { true } else { false } }) }, Py::str("the wind stress must be positive"))?;
    let _ = f__need({ let t250 = Py::Int(900i64); Py::Bool({ let t251 = v_rho.clone(); if py::le(&t250, &t251)? { { let t252 = Py::Int(1100i64); if py::le(&t251, &t252)? { true } else { false } } } else { false } }) }, Py::str("the water density must be between 900 and 1100 kg/m^3"))?;
    v_f = py::abs(&py::mul(&py::mul(&Py::Int(2i64), &c_OMEGA()?)?, &py::math1("sin", &py::math1("radians", &v_lat.clone())?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("M"), f__r(py::truediv(&v_tau.clone(), &py::mul(&v_rho.clone(), &v_f.clone())?)?)?), (Py::str("mass_transport"), f__r(py::truediv(&v_tau.clone(), &v_f.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_elastic_from_velocities(mut v_Vp: Py, mut v_Vs: Py, mut v_rho: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_nu: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("Vp"), v_Vp.clone()), (Py::str("Vs"), v_Vs.clone()), (Py::str("rho"), v_rho.clone())]))?;
    let _ = f__need({ let t266 = { let t262 = v_Vs.clone(); Py::Bool({ let t263 = Py::Int(0i64); if py::gt(&t262, &t263)? { true } else { false } }) }; if !t266.truthy() { t266 } else { { let t264 = v_rho.clone(); Py::Bool({ let t265 = Py::Int(0i64); if py::gt(&t264, &t265)? { true } else { false } }) } } }, Py::str("Vs and density must be positive"))?;
    let _ = f__need({ let t267 = py::mul(&v_Vp.clone(), &v_Vp.clone())?; Py::Bool({ let t268 = py::mul(&py::truediv(&py::mul(&py::mul(&Py::Int(4i64), &v_Vs.clone())?, &v_Vs.clone())?, &Py::Int(3i64))?, &Py::Float(1.0000001))?; if py::gt(&t267, &t268)? { true } else { false } }) }, Py::str("Vp must exceed sqrt(4/3) Vs (a positive bulk modulus)"))?;
    v_mu = py::mul(&py::mul(&v_rho.clone(), &v_Vs.clone())?, &v_Vs.clone())?;
    v_lam = py::mul(&v_rho.clone(), &py::sub(&py::mul(&v_Vp.clone(), &v_Vp.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_Vs.clone())?, &v_Vs.clone())?)?)?;
    v_K = py::add(&v_lam.clone(), &py::truediv(&py::mul(&Py::Int(2i64), &v_mu.clone())?, &Py::Int(3i64))?)?;
    v_nu = py::truediv(&py::sub(&py::mul(&v_Vp.clone(), &v_Vp.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_Vs.clone())?, &v_Vs.clone())?)?, &py::mul(&Py::Int(2i64), &py::sub(&py::mul(&v_Vp.clone(), &v_Vp.clone())?, &py::mul(&v_Vs.clone(), &v_Vs.clone())?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("mu"), f__r(v_mu.clone())?), (Py::str("lam"), f__r(v_lam.clone())?), (Py::str("K"), f__r(v_K.clone())?), (Py::str("E"), f__r(py::mul(&py::mul(&Py::Int(2i64), &v_mu.clone())?, &py::add(&Py::Int(1i64), &v_nu.clone())?)?)?), (Py::str("nu"), f__r(v_nu.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_energy_balance_response(mut v_F: Py, mut v_lam: Py, mut v_C: Py, mut v_t: Py) -> R {
    let mut v_tau: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("F"), v_F.clone()), (Py::str("lam"), v_lam.clone()), (Py::str("C"), v_C.clone()), (Py::str("t"), v_t.clone())]))?;
    let _ = f__need({ let t282 = { let t278 = v_lam.clone(); Py::Bool({ let t279 = Py::Int(0i64); if py::gt(&t278, &t279)? { true } else { false } }) }; if !t282.truthy() { t282 } else { { let t280 = v_C.clone(); Py::Bool({ let t281 = Py::Int(0i64); if py::gt(&t280, &t281)? { true } else { false } }) } } }, Py::str("the sensitivity and the heat capacity must be positive"))?;
    let _ = f__need({ let t283 = Py::Int(0i64); Py::Bool({ let t284 = v_t.clone(); if py::le(&t283, &t284)? { { let t285 = Py::Float(100000.0); if py::le(&t284, &t285)? { true } else { false } } } else { false } }) }, Py::str("time must be between 0 and 100,000 years"))?;
    let _ = f__need({ let t286 = v_F.clone(); Py::Bool({ let t287 = Py::Int(0i64); if py::ne(&t286, &t287) { true } else { false } }) }, Py::str("a non-zero forcing is needed"))?;
    v_tau = py::truediv(&py::mul(&v_lam.clone(), &v_C.clone())?, &c_YEAR()?)?;
    return Ok(Py::dict(vec![(Py::str("dT"), f__r(py::mul(&py::mul(&v_lam.clone(), &v_F.clone())?, &py::neg(&py::math1("expm1", &py::truediv(&py::neg(&v_t.clone())?, &v_tau.clone())?)?)?)?)?), (Py::str("dT_eq"), f__r(py::mul(&v_lam.clone(), &v_F.clone())?)?), (Py::str("tau_years"), f__r(v_tau.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_epicentral_distance(mut v_lat1: Py, mut v_lon1: Py, mut v_lat2: Py, mut v_lon2: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_dl: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_p1: Py = Py::None;
    let mut v_p2: Py = Py::None;
    let _ = f__coord_ok(Py::dict(vec![(Py::str("lat1"), v_lat1.clone()), (Py::str("lon1"), v_lon1.clone()), (Py::str("lat2"), v_lat2.clone()), (Py::str("lon2"), v_lon2.clone())]))?;
    let t854 = (Py::tuple(vec![py::math1("radians", &v_lat1.clone())?, py::math1("radians", &v_lat2.clone())?, py::math1("radians", &py::sub(&v_lon2.clone(), &v_lon1.clone())?)?])).items()?;
    if t854.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_p1 = t854[0].clone();
    v_p2 = t854[1].clone();
    v_dl = t854[2].clone();
    v_num = py::hypot(&[py::mul(&py::math1("cos", &v_p2.clone())?, &py::math1("sin", &v_dl.clone())?)?, py::sub(&py::mul(&py::math1("cos", &v_p1.clone())?, &py::math1("sin", &v_p2.clone())?)?, &py::mul(&py::mul(&py::math1("sin", &v_p1.clone())?, &py::math1("cos", &v_p2.clone())?)?, &py::math1("cos", &v_dl.clone())?)?)?])?;
    v_den = py::add(&py::mul(&py::math1("sin", &v_p1.clone())?, &py::math1("sin", &v_p2.clone())?)?, &py::mul(&py::mul(&py::math1("cos", &v_p1.clone())?, &py::math1("cos", &v_p2.clone())?)?, &py::math1("cos", &v_dl.clone())?)?)?;
    v_d = py::math1("degrees", &py::atan2(&v_num.clone(), &v_den.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("delta"), f__r(v_d.clone())?), (Py::str("distance_km"), f__r(py::truediv(&py::mul(&py::mul(&py::mul(&v_d.clone(), &Py::Int(2i64))?, &Py::Float(std::f64::consts::PI))?, &c_R_EARTH_KM()?)?, &Py::Int(360i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_equilibrium_temperature(mut v_S: Py, mut v_albedo: Py) -> R {
    let mut v_q: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("S"), v_S.clone()), (Py::str("albedo"), v_albedo.clone())]))?;
    let _ = f__need({ let t829 = v_S.clone(); Py::Bool({ let t830 = Py::Int(0i64); if py::gt(&t829, &t830)? { true } else { false } }) }, Py::str("the solar flux must be positive"))?;
    let _ = f__need({ let t831 = Py::Int(0i64); Py::Bool({ let t832 = v_albedo.clone(); if py::le(&t831, &t832)? { { let t833 = Py::Int(1i64); if py::lt(&t832, &t833)? { true } else { false } } } else { false } }) }, Py::str("albedo must be in [0, 1)"))?;
    v_q = py::truediv(&py::mul(&v_S.clone(), &py::sub(&Py::Int(1i64), &v_albedo.clone())?)?, &Py::Int(4i64))?;
    return Ok(Py::dict(vec![(Py::str("Te"), f__r(py::pow(&py::truediv(&v_q.clone(), &c_SIGMA()?)?, &Py::Float(0.25))?)?), (Py::str("absorbed"), f__r(v_q.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_equivalent_potential_temperature(mut v_p: Py, mut v_T: Py, mut v_Td: Py) -> R {
    let _ = f__T_ok(v_T.clone(), Py::Float(228.15), Py::Float(333.15), Py::str("temperature"))?;
    let _ = f__T_ok(v_Td.clone(), Py::Float(228.15), Py::Float(333.15), Py::str("dewpoint"))?;
    let _ = f__p_ok(v_p.clone(), Py::Int(10000i64), Py::Int(110000i64), Py::str("pressure"))?;
    let _ = f__need({ let t319 = v_Td.clone(); Py::Bool({ let t320 = v_T.clone(); if py::le(&t319, &t320)? { true } else { false } }) }, Py::str("the dewpoint cannot exceed the temperature"))?;
    return Ok(Py::dict(vec![(Py::str("theta_e"), f__r(f__theta_e(v_p.clone(), v_T.clone(), v_Td.clone(), Py::Bool(true))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_geopotential(mut v_z: Py) -> R {
    let mut v_phi: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("z"), v_z.clone())]))?;
    let _ = f__need({ let t365 = py::neg(&Py::Int(1000i64))?; Py::Bool({ let t366 = v_z.clone(); if py::le(&t365, &t366)? { { let t367 = Py::Int(200000i64); if py::le(&t366, &t367)? { true } else { false } } } else { false } }) }, Py::str("height must be between -1 km and 200 km"))?;
    v_phi = py::truediv(&py::mul(&py::mul(&c_G0()?, &c_RE()?)?, &v_z.clone())?, &py::add(&c_RE()?, &v_z.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("phi"), f__r(v_phi.clone())?), (Py::str("Z"), f__r(py::truediv(&v_phi.clone(), &c_G0()?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_geostrophic_wind(mut v_lat: Py, mut v_dz: Py, mut v_dn: Py) -> R {
    let mut v_f: Py = Py::None;
    let _ = f__lat_ok(v_lat.clone(), Py::Float(90.0), Py::str("latitude"))?;
    let _ = f__finite(Py::dict(vec![(Py::str("dz"), v_dz.clone()), (Py::str("dn"), v_dn.clone())]))?;
    let _ = f__need({ let t380 = Py::Int(5i64); Py::Bool({ let t381 = py::abs(&v_lat.clone())?; if py::le(&t380, &t381)? { { let t382 = Py::Int(85i64); if py::le(&t381, &t382)? { true } else { false } } } else { false } }) }, Py::str("geostrophic balance is used here for 5 to 85 degrees latitude"))?;
    let _ = f__need({ let t383 = v_dn.clone(); Py::Bool({ let t384 = Py::Int(0i64); if py::gt(&t383, &t384)? { true } else { false } }) }, Py::str("the distance must be positive"))?;
    let _ = f__need({ let t385 = v_dz.clone(); Py::Bool({ let t386 = Py::Int(0i64); if py::ne(&t385, &t386) { true } else { false } }) }, Py::str("a height difference is needed (no gradient, no geostrophic wind)"))?;
    v_f = py::mul(&py::mul(&Py::Int(2i64), &c_OMEGA()?)?, &py::math1("sin", &py::math1("radians", &v_lat.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("Vg"), f__r(py::truediv(&py::mul(&py::truediv(&c_G0()?, &py::abs(&v_f.clone())?)?, &py::abs(&v_dz.clone())?)?, &v_dn.clone())?)?), (Py::str("f"), f__r(v_f.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_geotherm(mut v_T0: Py, mut v_q0: Py, mut v_k: Py, mut v_A: Py, mut v_z: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("T0"), v_T0.clone()), (Py::str("q0"), v_q0.clone()), (Py::str("k"), v_k.clone()), (Py::str("A"), v_A.clone()), (Py::str("z"), v_z.clone())]))?;
    let _ = f__need({ let t396 = v_k.clone(); Py::Bool({ let t397 = Py::Int(0i64); if py::gt(&t396, &t397)? { true } else { false } }) }, Py::str("the thermal conductivity must be positive"))?;
    let _ = f__need({ let t402 = { let t398 = v_q0.clone(); Py::Bool({ let t399 = Py::Int(0i64); if py::gt(&t398, &t399)? { true } else { false } }) }; if !t402.truthy() { t402 } else { { let t400 = v_A.clone(); Py::Bool({ let t401 = Py::Int(0i64); if py::ge(&t400, &t401)? { true } else { false } }) } } }, Py::str("surface heat flow must be positive and heat production non-negative"))?;
    let _ = f__need({ let t403 = Py::Int(0i64); Py::Bool({ let t404 = v_z.clone(); if py::le(&t403, &t404)? { { let t405 = Py::Float(200000.0); if py::le(&t404, &t405)? { true } else { false } } } else { false } }) }, Py::str("depth must be between 0 and 200 km"))?;
    return Ok(Py::dict(vec![(Py::str("T"), f__r(py::sub(&py::add(&v_T0.clone(), &py::truediv(&py::mul(&v_q0.clone(), &v_z.clone())?, &v_k.clone())?)?, &py::truediv(&py::mul(&py::mul(&v_A.clone(), &v_z.clone())?, &v_z.clone())?, &py::mul(&Py::Int(2i64), &v_k.clone())?)?)?)?), (Py::str("q"), f__r(py::sub(&v_q0.clone(), &py::mul(&v_A.clone(), &v_z.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gradient_wind(mut v_lat: Py, mut v_R: Py, mut v_Vg: Py, mut v_flow: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_disc: Py = Py::None;
    let mut v_f: Py = Py::None;
    let _ = f__lat_ok(v_lat.clone(), Py::Float(90.0), Py::str("latitude"))?;
    let _ = f__finite(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("Vg"), v_Vg.clone())]))?;
    let _ = f__need({ let t418 = Py::Int(5i64); Py::Bool({ let t419 = py::abs(&v_lat.clone())?; if py::le(&t418, &t419)? { { let t420 = Py::Int(85i64); if py::le(&t419, &t420)? { true } else { false } } } else { false } }) }, Py::str("gradient balance is used here for 5 to 85 degrees latitude"))?;
    let _ = f__need({ let t425 = { let t421 = v_R.clone(); Py::Bool({ let t422 = Py::Int(0i64); if py::gt(&t421, &t422)? { true } else { false } }) }; if !t425.truthy() { t425 } else { { let t423 = v_Vg.clone(); Py::Bool({ let t424 = Py::Int(0i64); if py::gt(&t423, &t424)? { true } else { false } }) } } }, Py::str("the radius of curvature and the geostrophic wind must be positive"))?;
    let _ = f__need({ let t426 = v_flow.clone(); Py::Bool({ let t427 = Py::tuple(vec![Py::str("cyclonic"), Py::str("anticyclonic")]); if py::contains(&t427, &t426)? { true } else { false } }) }, Py::str("the flow must be cyclonic or anticyclonic"))?;
    v_f = py::abs(&py::mul(&py::mul(&Py::Int(2i64), &c_OMEGA()?)?, &py::math1("sin", &py::math1("radians", &v_lat.clone())?)?)?)?;
    v_a = py::truediv(&py::mul(&v_f.clone(), &v_R.clone())?, &Py::Int(2i64))?;
    if ({ let t428 = v_flow.clone(); Py::Bool({ let t429 = Py::str("cyclonic"); if py::eq(&t428, &t429) { true } else { false } }) }).truthy() {
        v_V = py::truediv(&py::mul(&py::mul(&v_f.clone(), &v_R.clone())?, &v_Vg.clone())?, &py::add(&v_a.clone(), &py::math1("sqrt", &py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&py::mul(&v_f.clone(), &v_R.clone())?, &v_Vg.clone())?)?)?)?)?;
    } else {
        v_disc = py::sub(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&py::mul(&v_f.clone(), &v_R.clone())?, &v_Vg.clone())?)?;
        let _ = f__need({ let t430 = v_disc.clone(); Py::Bool({ let t431 = Py::Int(0i64); if py::ge(&t430, &t431)? { true } else { false } }) }, Py::str("no regular anticyclonic gradient wind: the geostrophic wind exceeds f R/4"))?;
        v_V = py::truediv(&py::mul(&py::mul(&v_f.clone(), &v_R.clone())?, &v_Vg.clone())?, &py::add(&v_a.clone(), &py::math1("sqrt", &v_disc.clone())?)?)?;
    }
    return Ok(Py::dict(vec![(Py::str("V"), f__r(v_V.clone())?), (Py::str("ratio"), f__r(py::truediv(&v_V.clone(), &v_Vg.clone())?)?), (Py::str("f"), f__r(v_f.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gravity_latitude(mut v_lat: Py, mut v_z: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__lat_ok(v_lat.clone(), Py::Float(90.0), Py::str("latitude"))?;
    let _ = f__finite(Py::dict(vec![(Py::str("z"), v_z.clone())]))?;
    let _ = f__need({ let t444 = py::neg(&Py::Int(11000i64))?; Py::Bool({ let t445 = v_z.clone(); if py::le(&t444, &t445)? { { let t446 = Py::Int(100000i64); if py::le(&t445, &t446)? { true } else { false } } } else { false } }) }, Py::str("height must be between -11 km and 100 km"))?;
    v_x = py::pow(&py::math1("sin", &py::math1("radians", &py::abs(&v_lat.clone())?)?)?, &Py::Int(2i64))?;
    v_g = py::truediv(&py::mul(&Py::Float(9.780318), &py::add(&Py::Float(1.0), &py::mul(&py::add(&Py::Float(0.0052788), &py::mul(&Py::Float(2.36e-05), &v_x.clone())?)?, &v_x.clone())?)?)?, &py::pow(&py::add(&Py::Int(1i64), &py::truediv(&v_z.clone(), &Py::Float(6371000.0))?)?, &Py::Int(2i64))?)?;
    return Ok(Py::dict(vec![(Py::str("g"), f__r(v_g.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_greenhouse_n_layers(mut v_S: Py, mut v_albedo: Py, mut v_N: Py) -> R {
    let mut v_te: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("S"), v_S.clone()), (Py::str("albedo"), v_albedo.clone())]))?;
    let _ = f__need({ let t461 = { let t456 = v_S.clone(); Py::Bool({ let t457 = Py::Int(0i64); if py::gt(&t456, &t457)? { true } else { false } }) }; if !t461.truthy() { t461 } else { { let t458 = Py::Int(0i64); Py::Bool({ let t459 = v_albedo.clone(); if py::le(&t458, &t459)? { { let t460 = Py::Int(1i64); if py::lt(&t459, &t460)? { true } else { false } } } else { false } }) } } }, Py::str("S must be positive and albedo in [0, 1)"))?;
    let _ = f__need({ let t465 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t465.truthy() { t465 } else { { let t462 = Py::Int(1i64); Py::Bool({ let t463 = v_N.clone(); if py::le(&t462, &t463)? { { let t464 = Py::Int(100i64); if py::le(&t463, &t464)? { true } else { false } } } else { false } }) } } }, Py::str("the number of layers must be a whole number from 1 to 100"))?;
    v_te = py::pow(&py::truediv(&py::mul(&v_S.clone(), &py::sub(&Py::Int(1i64), &v_albedo.clone())?)?, &py::mul(&Py::Int(4i64), &c_SIGMA()?)?)?, &Py::Float(0.25))?;
    return Ok(Py::dict(vec![(Py::str("Ts"), f__r(py::mul(&py::pow(&py::add(&v_N.clone(), &Py::Int(1i64))?, &Py::Float(0.25))?, &v_te.clone())?)?), (Py::str("T_top"), f__r(v_te.clone())?), (Py::str("T_bottom_layer"), f__r(py::mul(&py::pow(&v_N.clone(), &Py::Float(0.25))?, &v_te.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_greenhouse_single_layer(mut v_S: Py, mut v_albedo: Py, mut v_eps: Py) -> R {
    let mut v_q: Py = Py::None;
    let mut v_ts: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("S"), v_S.clone()), (Py::str("albedo"), v_albedo.clone()), (Py::str("eps"), v_eps.clone())]))?;
    let _ = f__need({ let t475 = v_S.clone(); Py::Bool({ let t476 = Py::Int(0i64); if py::gt(&t475, &t476)? { true } else { false } }) }, Py::str("the solar flux must be positive"))?;
    let _ = f__need({ let t477 = Py::Int(0i64); Py::Bool({ let t478 = v_albedo.clone(); if py::le(&t477, &t478)? { { let t479 = Py::Int(1i64); if py::lt(&t478, &t479)? { true } else { false } } } else { false } }) }, Py::str("albedo must be in [0, 1)"))?;
    let _ = f__need({ let t480 = Py::Int(0i64); Py::Bool({ let t481 = v_eps.clone(); if py::lt(&t480, &t481)? { { let t482 = Py::Int(1i64); if py::le(&t481, &t482)? { true } else { false } } } else { false } }) }, Py::str("the layer's emissivity must be in (0, 1]"))?;
    v_q = py::truediv(&py::mul(&v_S.clone(), &py::sub(&Py::Int(1i64), &v_albedo.clone())?)?, &Py::Int(4i64))?;
    v_ts = py::pow(&py::truediv(&v_q.clone(), &py::mul(&c_SIGMA()?, &py::sub(&Py::Int(1i64), &py::truediv(&v_eps.clone(), &Py::Int(2i64))?)?)?)?, &Py::Float(0.25))?;
    return Ok(Py::dict(vec![(Py::str("Ts"), f__r(v_ts.clone())?), (Py::str("Ta"), f__r(py::truediv(&v_ts.clone(), &py::pow(&Py::Int(2i64), &Py::Float(0.25))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_half_space_heat_flow(mut v_t: Py, mut v_dT: Py, mut v_k: Py, mut v_kappa: Py) -> R {
    let mut v_q: Py = Py::None;
    let mut v_ts: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("t"), v_t.clone()), (Py::str("dT"), v_dT.clone()), (Py::str("k"), v_k.clone()), (Py::str("kappa"), v_kappa.clone())]))?;
    let _ = f__need({ let t511 = Py::Int(0i64); Py::Bool({ let t512 = v_t.clone(); if py::lt(&t511, &t512)? { { let t513 = Py::Int(200i64); if py::le(&t512, &t513)? { true } else { false } } } else { false } }) }, Py::str("the seafloor age must be between 0 and 200 Myr"))?;
    let _ = f__need({ let t521 = { let t514 = v_dT.clone(); Py::Bool({ let t515 = Py::Int(0i64); if py::gt(&t514, &t515)? { true } else { false } }) }; if !t521.truthy() { t521 } else { { let t520 = { let t516 = v_k.clone(); Py::Bool({ let t517 = Py::Int(0i64); if py::gt(&t516, &t517)? { true } else { false } }) }; if !t520.truthy() { t520 } else { { let t518 = v_kappa.clone(); Py::Bool({ let t519 = Py::Int(0i64); if py::gt(&t518, &t519)? { true } else { false } }) } } } } }, Py::str("the temperature difference, conductivity and diffusivity must be positive"))?;
    v_ts = py::mul(&py::mul(&v_t.clone(), &Py::Float(1000000.0))?, &c_YEAR()?)?;
    v_q = py::truediv(&py::mul(&v_k.clone(), &v_dT.clone())?, &py::math1("sqrt", &py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_kappa.clone())?, &v_ts.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("q"), f__r(v_q.clone())?), (Py::str("q_mW"), f__r(py::mul(&v_q.clone(), &Py::Float(1000.0))?)?), (Py::str("thickness"), f__r(py::mul(&py::mul(&Py::Int(2i64), &c_ERFINV_09()?)?, &py::math1("sqrt", &py::mul(&v_kappa.clone(), &v_ts.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_heat_index(mut v_T: Py, mut v_RH: Py) -> R {
    let mut v_TF: Py = Py::None;
    let mut v_hf: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("T"), v_T.clone()), (Py::str("RH"), v_RH.clone())]))?;
    v_TF = py::add(&py::truediv(&py::mul(&py::sub(&v_T.clone(), &Py::Float(273.15))?, &Py::Int(9i64))?, &Py::Int(5i64))?, &Py::Int(32i64))?;
    let _ = f__need({ let t531 = Py::Int(80i64); Py::Bool({ let t532 = v_TF.clone(); if py::le(&t531, &t532)? { { let t533 = Py::Int(130i64); if py::le(&t532, &t533)? { true } else { false } } } else { false } }) }, Py::str("the heat index is defined for temperatures from 80 °F (26.7 °C) to 130 °F"))?;
    let _ = f__need({ let t534 = Py::Int(0i64); Py::Bool({ let t535 = v_RH.clone(); if py::le(&t534, &t535)? { { let t536 = Py::Int(1i64); if py::le(&t535, &t536)? { true } else { false } } } else { false } }) }, Py::str("relative humidity must be a fraction between 0 and 1 (0% to 100%)"))?;
    v_hf = f__heat_index_F(v_TF.clone(), v_RH.clone())?;
    return Ok(Py::dict(vec![(Py::str("HI"), f__r(py::add(&py::truediv(&py::mul(&py::sub(&v_hf.clone(), &Py::Int(32i64))?, &Py::Int(5i64))?, &Py::Int(9i64))?, &Py::Float(273.15))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hypsometric_thickness(mut v_p1: Py, mut v_p2: Py, mut v_Tv: Py) -> R {
    let _ = f__p_ok(v_p1.clone(), Py::Int(100i64), Py::Int(110000i64), Py::str("bottom pressure"))?;
    let _ = f__p_ok(v_p2.clone(), Py::Int(100i64), Py::Int(110000i64), Py::str("top pressure"))?;
    let _ = f__T_ok(v_Tv.clone(), Py::Int(150i64), Py::Int(350i64), Py::str("mean virtual temperature"))?;
    let _ = f__need({ let t568 = v_p1.clone(); Py::Bool({ let t569 = v_p2.clone(); if py::gt(&t568, &t569)? { true } else { false } }) }, Py::str("the bottom pressure p1 must exceed the top pressure p2"))?;
    return Ok(Py::dict(vec![(Py::str("dz"), f__r(py::mul(&py::truediv(&py::mul(&c_RD()?, &v_Tv.clone())?, &c_G0()?)?, &py::math1("log", &py::truediv(&v_p1.clone(), &v_p2.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_inertial_period(mut v_lat: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_f: Py = Py::None;
    let _ = f__lat_ok(v_lat.clone(), Py::Float(90.0), Py::str("latitude"))?;
    let _ = f__need({ let t582 = py::abs(&v_lat.clone())?; Py::Bool({ let t583 = Py::Float(0.5); if py::ge(&t582, &t583)? { true } else { false } }) }, Py::str("inertial oscillations need |latitude| >= 0.5 degrees (f = 0 at the equator)"))?;
    v_f = py::mul(&py::mul(&Py::Int(2i64), &c_OMEGA()?)?, &py::math1("sin", &py::math1("radians", &v_lat.clone())?)?)?;
    v_T = py::truediv(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::abs(&v_f.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("T_inertial"), f__r(v_T.clone())?), (Py::str("T_hours"), f__r(py::truediv(&v_T.clone(), &Py::Int(3600i64))?)?), (Py::str("f"), f__r(v_f.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_isochron_age(mut v_x: Py, mut v_y: Py, mut v_lam: Py) -> R {
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("lam"), v_lam.clone())]))?;
    let _ = f__need({ let t597 = { let t593 = py::len(&v_x.clone())?; Py::Bool({ let t594 = py::len(&v_y.clone())?; if py::eq(&t593, &t594) { true } else { false } }) }; if !t597.truthy() { t597 } else { { let t595 = py::len(&v_x.clone())?; Py::Bool({ let t596 = Py::Int(3i64); if py::ge(&t595, &t596)? { true } else { false } }) } } }, Py::str("give at least three samples, as many x as y values"))?;
    let _ = f__need({ let t598 = Py::Int(0i64); Py::Bool({ let t599 = v_lam.clone(); if py::lt(&t598, &t599)? { { let t600 = Py::Float(0.001); if py::le(&t599, &t600)? { true } else { false } } } else { false } }) }, Py::str("the decay constant must be positive"))?;
    v_n = py::len(&v_x.clone())?;
    let t601 = (Py::tuple(vec![py::truediv(&py::sum(&v_x.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_y.clone(), None)?, &v_n.clone())?])).items()?;
    if t601.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t601[0].clone();
    v_my = t601[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t602 in py::iter(&(v_x.clone()))? { let mut v_a: Py; 
v_a = t602;
out.push(py::pow(&py::sub(&v_a.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    let _ = f__need({ let t603 = v_sxx.clone(); Py::Bool({ let t604 = Py::Int(0i64); if py::gt(&t603, &t604)? { true } else { false } }) }, Py::str("the parent/daughter ratios must differ between samples"))?;
    v_slope = py::truediv(&py::sum(&{ let mut out = vec![]; for t605 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t606 = (t605).items()?;
if t606.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t606[0].clone();
v_b = t606[1].clone();
out.push(py::mul(&py::sub(&v_a.clone(), &v_mx.clone())?, &py::sub(&v_b.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?, &v_sxx.clone())?;
    let _ = f__need({ let t607 = v_slope.clone(); Py::Bool({ let t608 = Py::Int(0i64); if py::gt(&t607, &t608)? { true } else { false } }) }, Py::str("the isochron slope must be positive"))?;
    return Ok(Py::dict(vec![(Py::str("age"), f__r(py::truediv(&py::math1("log1p", &v_slope.clone())?, &v_lam.clone())?)?), (Py::str("slope"), f__r(v_slope.clone())?), (Py::str("initial_ratio"), f__r(py::sub(&v_my.clone(), &py::mul(&v_slope.clone(), &v_mx.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_isothermal_pressure(mut v_p0: Py, mut v_T: Py, mut v_z: Py) -> R {
    let mut v_H: Py = Py::None;
    let _ = f__p_ok(v_p0.clone(), Py::Int(1000i64), Py::Int(110000i64), Py::str("surface pressure"))?;
    let _ = f__T_ok(v_T.clone(), Py::Int(150i64), Py::Int(350i64), Py::str("temperature"))?;
    let _ = f__finite(Py::dict(vec![(Py::str("z"), v_z.clone())]))?;
    let _ = f__need({ let t624 = Py::Int(0i64); Py::Bool({ let t625 = v_z.clone(); if py::le(&t624, &t625)? { { let t626 = Py::Int(50000i64); if py::le(&t625, &t626)? { true } else { false } } } else { false } }) }, Py::str("height must be between 0 and 50 km"))?;
    v_H = py::truediv(&py::mul(&c_RD()?, &v_T.clone())?, &c_G0()?)?;
    return Ok(Py::dict(vec![(Py::str("p"), f__r(py::mul(&v_p0.clone(), &py::math1("exp", &py::truediv(&py::neg(&v_z.clone())?, &v_H.clone())?)?)?)?), (Py::str("H"), f__r(v_H.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_k_ar_age(mut v_ratio: Py, mut v_lam: Py, mut v_lam_e: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("ratio"), v_ratio.clone()), (Py::str("lam"), v_lam.clone()), (Py::str("lam_e"), v_lam_e.clone())]))?;
    let _ = f__need({ let t870 = Py::Int(0i64); Py::Bool({ let t871 = v_ratio.clone(); if py::lt(&t870, &t871)? { { let t872 = Py::Int(100i64); if py::le(&t871, &t872)? { true } else { false } } } else { false } }) }, Py::str("the 40Ar*/40K ratio must be positive"))?;
    let _ = f__need({ let t873 = Py::Int(0i64); Py::Bool({ let t874 = v_lam_e.clone(); if py::lt(&t873, &t874)? { { let t875 = v_lam.clone(); if py::lt(&t874, &t875)? { { let t876 = Py::Float(1e-06); if py::le(&t875, &t876)? { true } else { false } } } else { false } } } else { false } }) }, Py::str("need 0 < lambda_e < lambda"))?;
    return Ok(Py::dict(vec![(Py::str("age"), f__r(py::truediv(&py::math1("log1p", &py::mul(&py::truediv(&v_lam.clone(), &v_lam_e.clone())?, &v_ratio.clone())?)?, &v_lam.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lapse_rate_pressure(mut v_p0: Py, mut v_T0: Py, mut v_gamma: Py, mut v_z: Py) -> R {
    let mut v_T: Py = Py::None;
    let _ = f__p_ok(v_p0.clone(), Py::Int(1000i64), Py::Int(110000i64), Py::str("surface pressure"))?;
    let _ = f__T_ok(v_T0.clone(), Py::Int(150i64), Py::Int(350i64), Py::str("surface temperature"))?;
    let _ = f__finite(Py::dict(vec![(Py::str("gamma"), v_gamma.clone()), (Py::str("z"), v_z.clone())]))?;
    let _ = f__need({ let t923 = Py::Int(0i64); Py::Bool({ let t924 = v_gamma.clone(); if py::lt(&t923, &t924)? { { let t925 = Py::Float(0.0098); if py::le(&t924, &t925)? { true } else { false } } } else { false } }) }, Py::str("the lapse rate must be positive and at most 9.8 K/km"))?;
    let _ = f__need({ let t926 = Py::Int(0i64); Py::Bool({ let t927 = v_z.clone(); if py::le(&t926, &t927)? { { let t928 = Py::Int(20000i64); if py::le(&t927, &t928)? { true } else { false } } } else { false } }) }, Py::str("height must be between 0 and 20 km"))?;
    v_T = py::sub(&v_T0.clone(), &py::mul(&v_gamma.clone(), &v_z.clone())?)?;
    let _ = f__need({ let t929 = v_T.clone(); Py::Bool({ let t930 = Py::Int(150i64); if py::gt(&t929, &t930)? { true } else { false } }) }, Py::str("the temperature at that height would fall below 150 K"))?;
    return Ok(Py::dict(vec![(Py::str("p"), f__r(py::mul(&v_p0.clone(), &py::pow(&py::truediv(&v_T.clone(), &v_T0.clone())?, &py::truediv(&c_G0()?, &py::mul(&c_RD()?, &v_gamma.clone())?)?)?)?)?), (Py::str("T"), f__r(v_T.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lcl(mut v_p: Py, mut v_T: Py, mut v_Td: Py) -> R {
    let mut v_pl: Py = Py::None;
    let mut v_tl: Py = Py::None;
    let _ = f__T_ok(v_T.clone(), Py::Float(228.15), Py::Float(333.15), Py::str("temperature"))?;
    let _ = f__T_ok(v_Td.clone(), Py::Float(228.15), Py::Float(333.15), Py::str("dewpoint"))?;
    let _ = f__p_ok(v_p.clone(), Py::Int(10000i64), Py::Int(110000i64), Py::str("pressure"))?;
    let _ = f__need({ let t642 = v_Td.clone(); Py::Bool({ let t643 = v_T.clone(); if py::le(&t642, &t643)? { true } else { false } }) }, Py::str("the dewpoint cannot exceed the temperature"))?;
    let t658 = (f__lcl(v_p.clone(), v_T.clone(), v_Td.clone(), Py::None)?).items()?;
    if t658.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pl = t658[0].clone();
    v_tl = t658[1].clone();
    return Ok(Py::dict(vec![(Py::str("p_lcl"), f__r(v_pl.clone())?), (Py::str("T_lcl"), f__r(v_tl.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_local_magnitude(mut v_A: Py, mut v_r: Py) -> R {
    let mut v_a_mm: Py = Py::None;
    let mut v_r_km: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("r"), v_r.clone())]))?;
    let _ = f__need({ let t668 = v_A.clone(); Py::Bool({ let t669 = Py::Int(0i64); if py::gt(&t668, &t669)? { true } else { false } }) }, Py::str("the amplitude must be positive"))?;
    let _ = f__need({ let t670 = Py::Float(10000.0); Py::Bool({ let t671 = v_r.clone(); if py::le(&t670, &t671)? { { let t672 = Py::Float(600000.0); if py::le(&t671, &t672)? { true } else { false } } } else { false } }) }, Py::str("the hypocentral distance must be between 10 and 600 km"))?;
    let t673 = (Py::tuple(vec![py::mul(&v_A.clone(), &Py::Float(1000.0))?, py::truediv(&v_r.clone(), &Py::Float(1000.0))?])).items()?;
    if t673.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a_mm = t673[0].clone();
    v_r_km = t673[1].clone();
    return Ok(Py::dict(vec![(Py::str("ML"), f__r(py::add(&py::add(&py::add(&py::math1("log10", &v_a_mm.clone())?, &py::math1("log10", &py::truediv(&v_r_km.clone(), &Py::Float(100.0))?)?)?, &py::mul(&Py::Float(0.00301), &py::sub(&v_r_km.clone(), &Py::Float(100.0))?)?)?, &Py::Float(3.0))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mixing_ratio(mut v_e: Py, mut v_p: Py) -> R {
    let mut v_w: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("e"), v_e.clone()), (Py::str("p"), v_p.clone())]))?;
    let _ = f__need({ let t683 = Py::Int(0i64); Py::Bool({ let t684 = v_e.clone(); if py::lt(&t683, &t684)? { { let t685 = v_p.clone(); if py::lt(&t684, &t685)? { true } else { false } } } else { false } }) }, Py::str("the vapour pressure must be positive and below the total pressure"))?;
    let _ = f__p_ok(v_p.clone(), Py::Int(100i64), Py::Int(110000i64), Py::str("pressure"))?;
    v_w = py::truediv(&py::mul(&c_EPS()?, &v_e.clone())?, &py::sub(&v_p.clone(), &v_e.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("w"), f__r(v_w.clone())?), (Py::str("w_g_per_kg"), f__r(py::mul(&Py::Int(1000i64), &v_w.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_moist_air_density(mut v_p: Py, mut v_T: Py, mut v_w: Py) -> R {
    let mut v_Tv: Py = Py::None;
    let _ = f__T_ok(v_T.clone(), Py::Int(150i64), Py::Int(350i64), Py::str("temperature"))?;
    let _ = f__p_ok(v_p.clone(), Py::Int(100i64), Py::Int(110000i64), Py::str("pressure"))?;
    let _ = f__finite(Py::dict(vec![(Py::str("w"), v_w.clone())]))?;
    let _ = f__need({ let t721 = Py::Int(0i64); Py::Bool({ let t722 = v_w.clone(); if py::le(&t721, &t722)? { { let t723 = Py::Float(0.06); if py::le(&t722, &t723)? { true } else { false } } } else { false } }) }, Py::str("the mixing ratio must be a fraction between 0 and 0.06 kg/kg"))?;
    v_Tv = py::truediv(&py::mul(&v_T.clone(), &py::add(&v_w.clone(), &c_EPS()?)?)?, &py::mul(&c_EPS()?, &py::add(&Py::Int(1i64), &v_w.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("rho"), f__r(py::truediv(&v_p.clone(), &py::mul(&c_RD()?, &v_Tv.clone())?)?)?), (Py::str("Tv"), f__r(v_Tv.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_moist_lapse_rate(mut v_T: Py, mut v_p: Py) -> R {
    let mut v_gam: Py = Py::None;
    let mut v_rs: Py = Py::None;
    let _ = f__T_ok(v_T.clone(), Py::Float(233.15), Py::Float(323.15), Py::str("temperature"))?;
    let _ = f__p_ok(v_p.clone(), Py::Int(10000i64), Py::Int(110000i64), Py::str("pressure"))?;
    v_rs = f__ws(v_p.clone(), v_T.clone())?;
    v_gam = py::truediv(&py::mul(&c_G0()?, &py::add(&Py::Int(1i64), &py::truediv(&py::mul(&c_LV0()?, &v_rs.clone())?, &py::mul(&c_RD()?, &v_T.clone())?)?)?)?, &py::add(&c_CPD()?, &py::truediv(&py::mul(&py::mul(&c_LV0()?, &c_LV0()?)?, &v_rs.clone())?, &py::mul(&py::mul(&c_RV()?, &v_T.clone())?, &v_T.clone())?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("lapse_rate"), f__r(v_gam.clone())?), (Py::str("rs"), f__r(v_rs.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_moist_static_energy(mut v_z: Py, mut v_T: Py, mut v_q: Py) -> R {
    let mut v_s: Py = Py::None;
    let _ = f__T_ok(v_T.clone(), Py::Int(150i64), Py::Int(350i64), Py::str("temperature"))?;
    let _ = f__finite(Py::dict(vec![(Py::str("z"), v_z.clone()), (Py::str("q"), v_q.clone())]))?;
    let _ = f__need({ let t736 = py::neg(&Py::Int(500i64))?; Py::Bool({ let t737 = v_z.clone(); if py::le(&t736, &t737)? { { let t738 = Py::Int(30000i64); if py::le(&t737, &t738)? { true } else { false } } } else { false } }) }, Py::str("height must be between -500 m and 30 km"))?;
    let _ = f__need({ let t739 = Py::Int(0i64); Py::Bool({ let t740 = v_q.clone(); if py::le(&t739, &t740)? { { let t741 = Py::Float(0.06); if py::le(&t740, &t741)? { true } else { false } } } else { false } }) }, Py::str("specific humidity must be a fraction between 0 and 0.06 kg/kg"))?;
    v_s = py::add(&py::mul(&c_G0()?, &v_z.clone())?, &py::mul(&c_CPD()?, &v_T.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("h"), f__r(py::add(&v_s.clone(), &py::mul(&c_LV0()?, &v_q.clone())?)?)?), (Py::str("s"), f__r(v_s.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_moment_magnitude(mut v_M0: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("M0"), v_M0.clone())]))?;
    let _ = f__need({ let t751 = Py::Float(1000000.0); Py::Bool({ let t752 = v_M0.clone(); if py::le(&t751, &t752)? { { let t753 = Py::Float(1e+25); if py::le(&t752, &t753)? { true } else { false } } } else { false } }) }, Py::str("the seismic moment must be between 1e6 and 1e25 N m"))?;
    return Ok(Py::dict(vec![(Py::str("Mw"), f__r(py::sub(&py::truediv(&py::math1("log10", &py::mul(&v_M0.clone(), &Py::Float(10000000.0))?)?, &Py::Float(1.5))?, &Py::Float(10.7))?)?), (Py::str("M0_dyne_cm"), f__r(py::mul(&v_M0.clone(), &Py::Float(10000000.0))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nmo_velocity(mut v_x: Py, mut v_t: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_icpt: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_t0: Py = Py::None;
    let _ = f__need({ let t763 = { let t759 = py::len(&v_x.clone())?; Py::Bool({ let t760 = py::len(&v_t.clone())?; if py::eq(&t759, &t760) { true } else { false } }) }; if !t763.truthy() { t763 } else { { let t761 = py::len(&v_x.clone())?; Py::Bool({ let t762 = Py::Int(2i64); if py::ge(&t761, &t762)? { true } else { false } }) } } }, Py::str("give as many offsets as times (at least two)"))?;
    v_X = { let mut out = vec![]; for t764 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t764;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) };
    v_Y = { let mut out = vec![]; for t765 in py::iter(&(v_t.clone()))? { let mut v_v: Py; 
v_v = t765;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) };
    v_n = py::len(&v_X.clone())?;
    let t766 = (Py::tuple(vec![py::truediv(&py::sum(&v_X.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_Y.clone(), None)?, &v_n.clone())?])).items()?;
    if t766.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t766[0].clone();
    v_my = t766[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t767 in py::iter(&(v_X.clone()))? { let mut v_a: Py; 
v_a = t767;
out.push(py::pow(&py::sub(&v_a.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    let _ = f__need({ let t768 = v_sxx.clone(); Py::Bool({ let t769 = Py::Int(0i64); if py::gt(&t768, &t769)? { true } else { false } }) }, Py::str("at least two different offsets are needed"))?;
    v_slope = py::truediv(&py::sum(&{ let mut out = vec![]; for t770 in py::iter(&(py::zip(&[v_X.clone(), v_Y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t771 = (t770).items()?;
if t771.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t771[0].clone();
v_b = t771[1].clone();
out.push(py::mul(&py::sub(&v_a.clone(), &v_mx.clone())?, &py::sub(&v_b.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?, &v_sxx.clone())?;
    v_icpt = py::sub(&v_my.clone(), &py::mul(&v_slope.clone(), &v_mx.clone())?)?;
    let _ = f__need({ let t776 = { let t772 = v_slope.clone(); Py::Bool({ let t773 = Py::Int(0i64); if py::gt(&t772, &t773)? { true } else { false } }) }; if !t776.truthy() { t776 } else { { let t774 = v_icpt.clone(); Py::Bool({ let t775 = Py::Int(0i64); if py::gt(&t774, &t775)? { true } else { false } }) } } }, Py::str("the picks do not fit a reflection hyperbola (t^2 must grow with x^2 from t0^2 > 0)"))?;
    let t777 = (Py::tuple(vec![py::truediv(&Py::Int(1i64), &py::math1("sqrt", &v_slope.clone())?)?, py::math1("sqrt", &v_icpt.clone())?])).items()?;
    if t777.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_V = t777[0].clone();
    v_t0 = t777[1].clone();
    return Ok(Py::dict(vec![(Py::str("V_nmo"), f__r(v_V.clone())?), (Py::str("t0"), f__r(v_t0.clone())?), (Py::str("depth"), f__r(py::truediv(&py::mul(&v_V.clone(), &v_t0.clone())?, &Py::Int(2i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ocean_depth(mut v_p: Py, mut v_lat: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_bot: Py = Py::None;
    let mut v_top: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("p"), v_p.clone())]))?;
    let _ = f__lat_ok(v_lat.clone(), Py::Float(90.0), Py::str("latitude"))?;
    let _ = f__need({ let t790 = Py::Int(0i64); Py::Bool({ let t791 = v_p.clone(); if py::le(&t790, &t791)? { { let t792 = Py::Float(120000000.0); if py::le(&t791, &t792)? { true } else { false } } } else { false } }) }, Py::str("sea pressure must be between 0 and 12,000 dbar"))?;
    v_P = py::truediv(&v_p.clone(), &Py::Float(10000.0))?;
    v_x = py::pow(&py::math1("sin", &py::math1("radians", &py::abs(&v_lat.clone())?)?)?, &Py::Int(2i64))?;
    v_bot = py::add(&py::mul(&Py::Float(9.780318), &py::add(&Py::Float(1.0), &py::mul(&py::add(&Py::Float(0.0052788), &py::mul(&Py::Float(2.36e-05), &v_x.clone())?)?, &v_x.clone())?)?)?, &py::mul(&py::mul(&Py::Float(2.184e-06), &Py::Float(0.5))?, &v_P.clone())?)?;
    v_top = py::mul(&py::add(&py::mul(&py::sub(&py::mul(&py::add(&py::mul(&py::neg(&Py::Float(1.82e-15))?, &v_P.clone())?, &Py::Float(2.279e-10))?, &v_P.clone())?, &Py::Float(2.2512e-05))?, &v_P.clone())?, &Py::Float(9.72659))?, &v_P.clone())?;
    return Ok(Py::dict(vec![(Py::str("depth"), f__r(py::truediv(&v_top.clone(), &v_bot.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ocean_pressure(mut v_depth: Py, mut v_lat: Py) -> R {
    let mut v_c1: Py = Py::None;
    let mut v_pd: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("depth"), v_depth.clone())]))?;
    let _ = f__lat_ok(v_lat.clone(), Py::Float(90.0), Py::str("latitude"))?;
    let _ = f__need({ let t805 = Py::Int(0i64); Py::Bool({ let t806 = v_depth.clone(); if py::le(&t805, &t806)? { { let t807 = Py::Int(11000i64); if py::le(&t806, &t807)? { true } else { false } } } else { false } }) }, Py::str("depth must be between 0 and 11,000 m"))?;
    v_c1 = py::add(&Py::Float(0.00592), &py::mul(&py::pow(&py::math1("sin", &py::abs(&py::math1("radians", &v_lat.clone())?)?)?, &Py::Int(2i64))?, &Py::Float(0.00525))?)?;
    v_pd = py::truediv(&py::sub(&py::sub(&Py::Int(1i64), &v_c1.clone())?, &py::math1("sqrt", &py::sub(&py::pow(&py::sub(&Py::Int(1i64), &v_c1.clone())?, &Py::Int(2i64))?, &py::mul(&Py::Float(8.84e-06), &v_depth.clone())?)?)?)?, &Py::Float(4.42e-06))?;
    return Ok(Py::dict(vec![(Py::str("p"), f__r(py::mul(&v_pd.clone(), &Py::Float(10000.0))?)?), (Py::str("p_dbar"), f__r(v_pd.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_paleolatitude(mut v_I_deg: Py) -> R {
    let mut v_lat: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("I_deg"), v_I_deg.clone())]))?;
    let _ = f__need({ let t817 = py::neg(&Py::Float(89.999))?; Py::Bool({ let t818 = v_I_deg.clone(); if py::le(&t817, &t818)? { { let t819 = Py::Float(89.999); if py::le(&t818, &t819)? { true } else { false } } } else { false } }) }, Py::str("the inclination must be strictly between -90 and 90 degrees"))?;
    v_lat = py::math1("degrees", &py::math1("atan", &py::truediv(&py::math1("tan", &py::math1("radians", &v_I_deg.clone())?)?, &Py::Int(2i64))?)?)?;
    return Ok(Py::dict(vec![(Py::str("paleolatitude"), f__r(v_lat.clone())?), (Py::str("pole_distance"), f__r(py::sub(&Py::Int(90i64), &v_lat.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_plate_velocity(mut v_pole_lat: Py, mut v_pole_lon: Py, mut v_omega: Py, mut v_lat: Py, mut v_lon: Py) -> R {
    let mut v_az: Py = Py::None;
    let mut v_az_pole: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dl: Py = Py::None;
    let mut v_p1: Py = Py::None;
    let mut v_p2: Py = Py::None;
    let mut v_speed: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__coord_ok(Py::dict(vec![(Py::str("pole_lat"), v_pole_lat.clone()), (Py::str("pole_lon"), v_pole_lon.clone()), (Py::str("lat"), v_lat.clone()), (Py::str("lon"), v_lon.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("omega"), v_omega.clone())]))?;
    let _ = f__need({ let t853 = { let t849 = v_omega.clone(); Py::Bool({ let t850 = Py::Int(0i64); if py::ne(&t849, &t850) { true } else { false } }) }; if !t853.truthy() { t853 } else { { let t851 = py::abs(&v_omega.clone())?; Py::Bool({ let t852 = Py::Int(10i64); if py::le(&t851, &t852)? { true } else { false } }) } } }, Py::str("the rotation rate must be non-zero and at most 10 degrees per Myr"))?;
    v_d = py::getitem(&f_epicentral_distance(v_lat.clone(), v_lon.clone(), v_pole_lat.clone(), v_pole_lon.clone())?, &Py::str("delta"))?;
    let _ = f__need({ let t860 = Py::Float(0.5); Py::Bool({ let t861 = v_d.clone(); if py::le(&t860, &t861)? { { let t862 = Py::Float(179.5); if py::le(&t861, &t862)? { true } else { false } } } else { false } }) }, Py::str("the site must not lie within 0.5 degrees of the pole or its antipode"))?;
    let t863 = (Py::tuple(vec![py::math1("radians", &v_lat.clone())?, py::math1("radians", &v_pole_lat.clone())?, py::math1("radians", &py::sub(&v_pole_lon.clone(), &v_lon.clone())?)?])).items()?;
    if t863.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_p1 = t863[0].clone();
    v_p2 = t863[1].clone();
    v_dl = t863[2].clone();
    v_az_pole = py::math1("degrees", &py::atan2(&py::mul(&py::math1("sin", &v_dl.clone())?, &py::math1("cos", &v_p2.clone())?)?, &py::sub(&py::mul(&py::math1("cos", &v_p1.clone())?, &py::math1("sin", &v_p2.clone())?)?, &py::mul(&py::mul(&py::math1("sin", &v_p1.clone())?, &py::math1("cos", &v_p2.clone())?)?, &py::math1("cos", &v_dl.clone())?)?)?)?)?;
    v_w = py::truediv(&py::math1("radians", &py::abs(&v_omega.clone())?)?, &Py::Float(1000000.0))?;
    v_speed = py::mul(&py::mul(&py::mul(&v_w.clone(), &c_R_EARTH_KM()?)?, &Py::Float(1000000.0))?, &py::math1("sin", &py::math1("radians", &v_d.clone())?)?)?;
    v_az = py::modulo(&py::add(&v_az_pole.clone(), &(if ({ let t864 = v_omega.clone(); Py::Bool({ let t865 = Py::Int(0i64); if py::gt(&t864, &t865)? { true } else { false } }) }).truthy() { Py::Float(90.0) } else { py::neg(&Py::Float(90.0))? }))?, &Py::Float(360.0))?;
    return Ok(Py::dict(vec![(Py::str("speed"), f__r(v_speed.clone())?), (Py::str("azimuth"), f__r(v_az.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_potential_temperature(mut v_T: Py, mut v_p: Py) -> R {
    let _ = f__T_ok(v_T.clone(), Py::Int(150i64), Py::Int(350i64), Py::str("temperature"))?;
    let _ = f__p_ok(v_p.clone(), Py::Int(100i64), Py::Int(110000i64), Py::str("pressure"))?;
    return Ok(Py::dict(vec![(Py::str("theta"), f__r(py::mul(&v_T.clone(), &py::pow(&py::truediv(&c_P0()?, &v_p.clone())?, &c_KAPPA()?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pratt_density(mut v_h: Py, mut v_rho0: Py, mut v_D: Py) -> R {
    let mut v_rho: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("h"), v_h.clone()), (Py::str("rho0"), v_rho0.clone()), (Py::str("D"), v_D.clone())]))?;
    let _ = f__need({ let t901 = v_h.clone(); Py::Bool({ let t902 = Py::Int(0i64); if py::gt(&t901, &t902)? { true } else { false } }) }, Py::str("the elevation must be positive"))?;
    let _ = f__need({ let t907 = { let t903 = v_rho0.clone(); Py::Bool({ let t904 = Py::Int(0i64); if py::gt(&t903, &t904)? { true } else { false } }) }; if !t907.truthy() { t907 } else { { let t905 = v_D.clone(); Py::Bool({ let t906 = Py::Int(0i64); if py::gt(&t905, &t906)? { true } else { false } }) } } }, Py::str("the reference density and the compensation depth must be positive"))?;
    v_rho = py::truediv(&py::mul(&v_rho0.clone(), &v_D.clone())?, &py::add(&v_D.clone(), &v_h.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("rho"), f__r(v_rho.clone())?), (Py::str("drho"), f__r(py::truediv(&py::mul(&v_rho0.clone(), &v_h.clone())?, &py::add(&v_D.clone(), &v_h.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pure_water_density(mut v_T: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("T"), v_T.clone())]))?;
    let _ = f__need({ let t940 = Py::Float(273.15); Py::Bool({ let t941 = v_T.clone(); if py::le(&t940, &t941)? { { let t942 = Py::Float(313.15); if py::le(&t941, &t942)? { true } else { false } } } else { false } }) }, Py::str("temperature must be between 0 and 40 °C"))?;
    return Ok(Py::dict(vec![(Py::str("rho"), f__r(f__smow(py::sub(&v_T.clone(), &Py::Float(273.15))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_radiocarbon_age(mut v_F: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("F"), v_F.clone())]))?;
    let _ = f__need({ let t966 = Py::Float(1e-05); Py::Bool({ let t967 = v_F.clone(); if py::le(&t966, &t967)? { { let t968 = Py::Float(1.0); if py::le(&t967, &t968)? { true } else { false } } } else { false } }) }, Py::str("the fraction modern must be between 1e-5 and 1"))?;
    return Ok(Py::dict(vec![(Py::str("age"), f__r(py::mul(&py::neg(&Py::Float(8033.0))?, &py::math1("log", &v_F.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_radiometric_age(mut v_ratio: Py, mut v_half_life: Py) -> R {
    let mut v_lam: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("ratio"), v_ratio.clone()), (Py::str("half_life"), v_half_life.clone())]))?;
    let _ = f__need({ let t978 = v_half_life.clone(); Py::Bool({ let t979 = Py::Int(0i64); if py::gt(&t978, &t979)? { true } else { false } }) }, Py::str("the half-life must be positive"))?;
    let _ = f__need({ let t980 = Py::Int(0i64); Py::Bool({ let t981 = v_ratio.clone(); if py::lt(&t980, &t981)? { { let t982 = Py::Float(1000000.0); if py::le(&t981, &t982)? { true } else { false } } } else { false } }) }, Py::str("the daughter/parent ratio must be positive"))?;
    v_lam = py::truediv(&py::math1("log", &Py::Int(2i64))?, &v_half_life.clone())?;
    return Ok(Py::dict(vec![(Py::str("age"), f__r(py::truediv(&py::math1("log1p", &v_ratio.clone())?, &v_lam.clone())?)?), (Py::str("lambda"), f__r(v_lam.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_relative_humidity(mut v_T: Py, mut v_Td: Py) -> R {
    let mut v_rh: Py = Py::None;
    let _ = f__T_ok(v_T.clone(), Py::Float(228.15), Py::Float(333.15), Py::str("temperature"))?;
    let _ = f__T_ok(v_Td.clone(), Py::Float(228.15), Py::Float(333.15), Py::str("dewpoint"))?;
    let _ = f__need({ let t995 = v_Td.clone(); Py::Bool({ let t996 = v_T.clone(); if py::le(&t995, &t996)? { true } else { false } }) }, Py::str("the dewpoint cannot exceed the temperature"))?;
    v_rh = py::truediv(&f__es(v_Td.clone())?, &f__es(v_T.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("RH"), f__r(v_rh.clone())?), (Py::str("RH_percent"), f__r(py::mul(&Py::Int(100i64), &v_rh.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rossby_number(mut v_U: Py, mut v_L: Py, mut v_lat: Py) -> R {
    let mut v_f: Py = Py::None;
    let _ = f__lat_ok(v_lat.clone(), Py::Float(90.0), Py::str("latitude"))?;
    let _ = f__finite(Py::dict(vec![(Py::str("U"), v_U.clone()), (Py::str("L"), v_L.clone())]))?;
    let _ = f__need({ let t1009 = py::abs(&v_lat.clone())?; Py::Bool({ let t1010 = Py::Int(1i64); if py::ge(&t1009, &t1010)? { true } else { false } }) }, Py::str("the Rossby number is not defined at the equator (|latitude| >= 1 degree)"))?;
    let _ = f__need({ let t1015 = { let t1011 = v_U.clone(); Py::Bool({ let t1012 = Py::Int(0i64); if py::gt(&t1011, &t1012)? { true } else { false } }) }; if !t1015.truthy() { t1015 } else { { let t1013 = v_L.clone(); Py::Bool({ let t1014 = Py::Int(0i64); if py::gt(&t1013, &t1014)? { true } else { false } }) } } }, Py::str("the speed and the length scale must be positive"))?;
    v_f = py::abs(&py::mul(&py::mul(&Py::Int(2i64), &c_OMEGA()?)?, &py::math1("sin", &py::math1("radians", &v_lat.clone())?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("Ro"), f__r(py::truediv(&v_U.clone(), &py::mul(&v_f.clone(), &v_L.clone())?)?)?), (Py::str("f"), f__r(v_f.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rossby_wave(mut v_U: Py, mut v_lat: Py, mut v_Lx: Py) -> R {
    let mut v_beta: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("U"), v_U.clone()), (Py::str("Lx"), v_Lx.clone())]))?;
    let _ = f__lat_ok(v_lat.clone(), Py::Float(90.0), Py::str("latitude"))?;
    let _ = f__need({ let t1028 = Py::Int(5i64); Py::Bool({ let t1029 = py::abs(&v_lat.clone())?; if py::le(&t1028, &t1029)? { { let t1030 = Py::Int(85i64); if py::le(&t1029, &t1030)? { true } else { false } } } else { false } }) }, Py::str("the beta-plane is used here for 5 to 85 degrees latitude"))?;
    let _ = f__need({ let t1031 = v_U.clone(); Py::Bool({ let t1032 = Py::Int(0i64); if py::gt(&t1031, &t1032)? { true } else { false } }) }, Py::str("a westerly mean wind U > 0 is required (it sets the stationary wavelength)"))?;
    let _ = f__need({ let t1033 = Py::Float(100000.0); Py::Bool({ let t1034 = v_Lx.clone(); if py::le(&t1033, &t1034)? { { let t1035 = Py::Float(40000000.0); if py::le(&t1034, &t1035)? { true } else { false } } } else { false } }) }, Py::str("the zonal wavelength must be between 100 km and 40,000 km"))?;
    v_beta = py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &c_OMEGA()?)?, &py::math1("cos", &py::math1("radians", &v_lat.clone())?)?)?, &c_RE()?)?;
    v_k = py::truediv(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_Lx.clone())?;
    return Ok(Py::dict(vec![(Py::str("c"), f__r(py::sub(&v_U.clone(), &py::truediv(&v_beta.clone(), &py::mul(&v_k.clone(), &v_k.clone())?)?)?)?), (Py::str("beta"), f__r(v_beta.clone())?), (Py::str("Ls"), f__r(py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::math1("sqrt", &py::truediv(&v_U.clone(), &v_beta.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_saturation_mixing_ratio(mut v_p: Py, mut v_T: Py) -> R {
    let mut v_w: Py = Py::None;
    let _ = f__T_ok(v_T.clone(), Py::Float(228.15), Py::Float(333.15), Py::str("temperature"))?;
    let _ = f__p_ok(v_p.clone(), Py::Int(100i64), Py::Int(110000i64), Py::str("pressure"))?;
    v_w = f__ws(v_p.clone(), v_T.clone())?;
    return Ok(Py::dict(vec![(Py::str("ws"), f__r(v_w.clone())?), (Py::str("ws_g_per_kg"), f__r(py::mul(&Py::Int(1000i64), &v_w.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_saturation_vapor_pressure(mut v_T: Py) -> R {
    let _ = f__T_ok(v_T.clone(), Py::Float(228.15), Py::Float(333.15), Py::str("temperature"))?;
    return Ok(Py::dict(vec![(Py::str("es"), f__r(f__es(v_T.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_scale_height(mut v_T: Py) -> R {
    let _ = f__T_ok(v_T.clone(), Py::Int(150i64), Py::Int(350i64), Py::str("temperature"))?;
    return Ok(Py::dict(vec![(Py::str("H"), f__r(py::truediv(&py::mul(&c_RD()?, &v_T.clone())?, &c_G0()?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_seafloor_depth(mut v_t: Py, mut v_dr: Py, mut v_Tm: Py, mut v_Tw: Py, mut v_alpha: Py, mut v_kappa: Py, mut v_rho_m: Py, mut v_rho_w: Py) -> R {
    let mut v_ts: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("t"), v_t.clone()), (Py::str("dr"), v_dr.clone()), (Py::str("Tm"), v_Tm.clone()), (Py::str("Tw"), v_Tw.clone()), (Py::str("alpha"), v_alpha.clone()), (Py::str("kappa"), v_kappa.clone()), (Py::str("rho_m"), v_rho_m.clone()), (Py::str("rho_w"), v_rho_w.clone())]))?;
    let _ = f__need({ let t1086 = Py::Int(0i64); Py::Bool({ let t1087 = v_t.clone(); if py::lt(&t1086, &t1087)? { { let t1088 = Py::Int(200i64); if py::le(&t1087, &t1088)? { true } else { false } } } else { false } }) }, Py::str("the seafloor age must be between 0 and 200 Myr"))?;
    let _ = f__need({ let t1100 = { let t1089 = v_Tm.clone(); Py::Bool({ let t1090 = v_Tw.clone(); if py::gt(&t1089, &t1090)? { true } else { false } }) }; if !t1100.truthy() { t1100 } else { { let t1099 = { let t1091 = v_alpha.clone(); Py::Bool({ let t1092 = Py::Int(0i64); if py::gt(&t1091, &t1092)? { true } else { false } }) }; if !t1099.truthy() { t1099 } else { { let t1098 = { let t1093 = v_kappa.clone(); Py::Bool({ let t1094 = Py::Int(0i64); if py::gt(&t1093, &t1094)? { true } else { false } }) }; if !t1098.truthy() { t1098 } else { { let t1095 = v_rho_m.clone(); Py::Bool({ let t1096 = v_rho_w.clone(); if py::gt(&t1095, &t1096)? { { let t1097 = Py::Int(0i64); if py::gt(&t1096, &t1097)? { true } else { false } } } else { false } }) } } } } } } }, Py::str("need Tm > Tw, alpha > 0, kappa > 0, rho_m > rho_w"))?;
    v_ts = py::mul(&py::mul(&v_t.clone(), &Py::Float(1000000.0))?, &c_YEAR()?)?;
    v_w = py::mul(&py::truediv(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_rho_m.clone())?, &v_alpha.clone())?, &py::sub(&v_Tm.clone(), &v_Tw.clone())?)?, &py::sub(&v_rho_m.clone(), &v_rho_w.clone())?)?, &py::math1("sqrt", &py::truediv(&py::mul(&v_kappa.clone(), &v_ts.clone())?, &Py::Float(std::f64::consts::PI))?)?)?;
    return Ok(Py::dict(vec![(Py::str("depth"), f__r(py::add(&v_dr.clone(), &v_w.clone())?)?), (Py::str("subsidence"), f__r(v_w.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_seawater_adiabatic_gradient(mut v_S: Py, mut v_T: Py, mut v_p: Py) -> R {
    let _ = f__sea_ok(v_S.clone(), v_T.clone(), v_p.clone())?;
    return Ok(Py::dict(vec![(Py::str("gamma"), f__r(f__adtg(v_S.clone(), py::sub(&v_T.clone(), &Py::Float(273.15))?, py::truediv(&v_p.clone(), &Py::Float(10000.0))?)?)?), (Py::str("gamma_per_km_dbar"), f__r(py::mul(&Py::Int(1000i64), &f__adtg(v_S.clone(), py::sub(&v_T.clone(), &Py::Float(273.15))?, py::truediv(&v_p.clone(), &Py::Float(10000.0))?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_seawater_density(mut v_S: Py, mut v_T: Py, mut v_p: Py) -> R {
    let mut v_rho: Py = Py::None;
    let _ = f__sea_ok(v_S.clone(), v_T.clone(), v_p.clone())?;
    v_rho = f__dens(v_S.clone(), py::sub(&v_T.clone(), &Py::Float(273.15))?, py::truediv(&v_p.clone(), &Py::Float(10000.0))?)?;
    return Ok(Py::dict(vec![(Py::str("rho"), f__r(v_rho.clone())?), (Py::str("sigma"), f__r(py::sub(&v_rho.clone(), &Py::Float(1000.0))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_seawater_density_surface(mut v_S: Py, mut v_T: Py) -> R {
    let mut v_rho: Py = Py::None;
    let _ = f__sea_ok(v_S.clone(), v_T.clone(), Py::None)?;
    v_rho = f__dens0(v_S.clone(), py::sub(&v_T.clone(), &Py::Float(273.15))?)?;
    return Ok(Py::dict(vec![(Py::str("rho"), f__r(v_rho.clone())?), (Py::str("sigma_t"), f__r(py::sub(&v_rho.clone(), &Py::Float(1000.0))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_seawater_freezing_point(mut v_S: Py, mut v_p: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_tf: Py = Py::None;
    let _ = f__sea_ok(v_S.clone(), Py::None, v_p.clone())?;
    let _ = f__need({ let t1198 = Py::Int(4i64); Py::Bool({ let t1199 = v_S.clone(); if py::le(&t1198, &t1199)? { { let t1200 = Py::Int(40i64); if py::le(&t1199, &t1200)? { true } else { false } } } else { false } }) }, Py::str("the freezing-point formula holds for salinities 4 to 40"))?;
    let _ = f__need({ let t1201 = v_p.clone(); Py::Bool({ let t1202 = Py::Float(5000000.0); if py::le(&t1201, &t1202)? { true } else { false } }) }, Py::str("the freezing-point formula holds to 500 dbar"))?;
    v_P = py::truediv(&v_p.clone(), &Py::Float(10000.0))?;
    v_tf = py::truediv(&py::sub(&py::sub(&py::add(&py::mul(&py::neg(&Py::Float(0.0575))?, &v_S.clone())?, &py::mul(&py::mul(&Py::Float(0.001710523), &v_S.clone())?, &py::math1("sqrt", &v_S.clone())?)?)?, &py::mul(&py::mul(&Py::Float(0.0002154996), &v_S.clone())?, &v_S.clone())?)?, &py::mul(&Py::Float(0.000753), &v_P.clone())?)?, &Py::Float(1.00024))?;
    return Ok(Py::dict(vec![(Py::str("Tf"), f__r(py::add(&v_tf.clone(), &Py::Float(273.15))?)?), (Py::str("Tf_C"), f__r(v_tf.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_seawater_potential_temperature(mut v_S: Py, mut v_T: Py, mut v_p: Py, mut v_pr: Py) -> R {
    let mut v_t: Py = Py::None;
    let mut v_th: Py = Py::None;
    let _ = f__sea_ok(v_S.clone(), v_T.clone(), v_p.clone())?;
    let _ = f__sea_ok(Py::None, Py::None, v_pr.clone())?;
    v_t = py::sub(&v_T.clone(), &Py::Float(273.15))?;
    v_th = f__ptmp(v_S.clone(), v_t.clone(), py::truediv(&v_p.clone(), &Py::Float(10000.0))?, py::truediv(&v_pr.clone(), &Py::Float(10000.0))?)?;
    return Ok(Py::dict(vec![(Py::str("theta"), f__r(py::add(&v_th.clone(), &Py::Float(273.15))?)?), (Py::str("theta_C"), f__r(v_th.clone())?), (Py::str("dtheta"), f__r(py::sub(&v_th.clone(), &v_t.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_shallow_water_wave(mut v_h: Py, mut v_distance: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("h"), v_h.clone())]))?;
    let _ = f__need({ let t1273 = Py::Int(0i64); Py::Bool({ let t1274 = v_h.clone(); if py::lt(&t1273, &t1274)? { { let t1275 = Py::Int(11000i64); if py::le(&t1274, &t1275)? { true } else { false } } } else { false } }) }, Py::str("the depth must be between 0 and 11,000 m"))?;
    v_c = py::math1("sqrt", &py::mul(&c_G0()?, &v_h.clone())?)?;
    v_out = Py::dict(vec![(Py::str("c"), f__r(v_c.clone())?)]);
    if ({ let t1281 = v_distance.clone(); Py::Bool({ let t1282 = Py::None; if !py::is(&t1281, &t1282) { true } else { false } }) }).truthy() {
        let _ = f__finite(Py::dict(vec![(Py::str("distance"), v_distance.clone())]))?;
        let _ = f__need({ let t1283 = v_distance.clone(); Py::Bool({ let t1284 = Py::Int(0i64); if py::gt(&t1283, &t1284)? { true } else { false } }) }, Py::str("the distance must be positive"))?;
        py::setitem(&v_out.clone(), &Py::str("travel_time"), f__r(py::truediv(&v_distance.clone(), &v_c.clone())?)?)?;
        py::setitem(&v_out.clone(), &Py::str("travel_time_h"), f__r(py::truediv(&py::truediv(&v_distance.clone(), &v_c.clone())?, &Py::Int(3600i64))?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_solar_elevation(mut v_lat: Py, mut v_decl: Py, mut v_H: Py) -> R {
    let mut v_az: Py = Py::None;
    let mut v_de: Py = Py::None;
    let mut v_east: Py = Py::None;
    let mut v_el: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_north: Py = Py::None;
    let mut v_ph: Py = Py::None;
    let mut v_up: Py = Py::None;
    let _ = f__lat_ok(v_lat.clone(), Py::Float(90.0), Py::str("latitude"))?;
    let _ = f__finite(Py::dict(vec![(Py::str("decl"), v_decl.clone()), (Py::str("H"), v_H.clone())]))?;
    let _ = f__need({ let t1292 = py::neg(&Py::Float(23.5))?; Py::Bool({ let t1293 = v_decl.clone(); if py::le(&t1292, &t1293)? { { let t1294 = Py::Float(23.5); if py::le(&t1293, &t1294)? { true } else { false } } } else { false } }) }, Py::str("the solar declination must be between -23.5 and 23.5 degrees"))?;
    let _ = f__need({ let t1295 = py::neg(&Py::Int(180i64))?; Py::Bool({ let t1296 = v_H.clone(); if py::le(&t1295, &t1296)? { { let t1297 = Py::Int(180i64); if py::le(&t1296, &t1297)? { true } else { false } } } else { false } }) }, Py::str("the hour angle must be between -180 and 180 degrees (negative before solar noon)"))?;
    let t1298 = (Py::tuple(vec![py::math1("radians", &v_lat.clone())?, py::math1("radians", &v_decl.clone())?, py::math1("radians", &v_H.clone())?])).items()?;
    if t1298.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_ph = t1298[0].clone();
    v_de = t1298[1].clone();
    v_h = t1298[2].clone();
    v_up = py::add(&py::mul(&py::math1("sin", &v_ph.clone())?, &py::math1("sin", &v_de.clone())?)?, &py::mul(&py::mul(&py::math1("cos", &v_ph.clone())?, &py::math1("cos", &v_de.clone())?)?, &py::math1("cos", &v_h.clone())?)?)?;
    v_east = py::mul(&py::neg(&py::math1("cos", &v_de.clone())?)?, &py::math1("sin", &v_h.clone())?)?;
    v_north = py::sub(&py::mul(&py::math1("cos", &v_ph.clone())?, &py::math1("sin", &v_de.clone())?)?, &py::mul(&py::mul(&py::math1("sin", &v_ph.clone())?, &py::math1("cos", &v_de.clone())?)?, &py::math1("cos", &v_h.clone())?)?)?;
    v_el = py::math1("degrees", &py::atan2(&v_up.clone(), &py::hypot(&[v_east.clone(), v_north.clone()])?)?)?;
    let _ = f__need({ let t1299 = py::hypot(&[v_east.clone(), v_north.clone()])?; Py::Bool({ let t1300 = Py::Float(1e-09); if py::gt(&t1299, &t1300)? { true } else { false } }) }, Py::str("the sun is at the zenith: the azimuth is undefined"))?;
    v_az = py::modulo(&py::math1("degrees", &py::atan2(&v_east.clone(), &v_north.clone())?)?, &Py::Float(360.0))?;
    return Ok(Py::dict(vec![(Py::str("elevation"), f__r(v_el.clone())?), (Py::str("zenith"), f__r(py::sub(&Py::Float(90.0), &v_el.clone())?)?), (Py::str("azimuth"), f__r(v_az.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sound_speed_seawater(mut v_S: Py, mut v_T: Py, mut v_p: Py) -> R {
    let _ = f__sea_ok(v_S.clone(), v_T.clone(), v_p.clone())?;
    return Ok(Py::dict(vec![(Py::str("c"), f__r(f__svel(v_S.clone(), py::sub(&v_T.clone(), &Py::Float(273.15))?, py::truediv(&v_p.clone(), &Py::Float(10000.0))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sp_distance(mut v_dt: Py, mut v_vp: Py, mut v_vs: Py) -> R {
    let mut v_d: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("dt"), v_dt.clone()), (Py::str("vp"), v_vp.clone()), (Py::str("vs"), v_vs.clone())]))?;
    let _ = f__need({ let t1335 = v_dt.clone(); Py::Bool({ let t1336 = Py::Int(0i64); if py::gt(&t1335, &t1336)? { true } else { false } }) }, Py::str("the S-P interval must be positive"))?;
    let _ = f__need({ let t1337 = v_vp.clone(); Py::Bool({ let t1338 = v_vs.clone(); if py::gt(&t1337, &t1338)? { { let t1339 = Py::Int(0i64); if py::gt(&t1338, &t1339)? { true } else { false } } } else { false } }) }, Py::str("need vp > vs > 0"))?;
    v_d = py::truediv(&py::mul(&py::mul(&v_dt.clone(), &v_vp.clone())?, &v_vs.clone())?, &py::sub(&v_vp.clone(), &v_vs.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("distance"), f__r(v_d.clone())?), (Py::str("distance_km"), f__r(py::truediv(&v_d.clone(), &Py::Float(1000.0))?)?), (Py::str("tp"), f__r(py::truediv(&v_d.clone(), &v_vp.clone())?)?), (Py::str("ts"), f__r(py::truediv(&v_d.clone(), &v_vs.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_specific_humidity_from_dewpoint(mut v_p: Py, mut v_Td: Py) -> R {
    let mut v_w: Py = Py::None;
    let _ = f__T_ok(v_Td.clone(), Py::Float(228.15), Py::Float(333.15), Py::str("dewpoint"))?;
    let _ = f__p_ok(v_p.clone(), Py::Int(100i64), Py::Int(110000i64), Py::str("pressure"))?;
    v_w = f__ws(v_p.clone(), v_Td.clone())?;
    return Ok(Py::dict(vec![(Py::str("q"), f__r(py::truediv(&v_w.clone(), &py::add(&Py::Int(1i64), &v_w.clone())?)?)?), (Py::str("w"), f__r(v_w.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_std_height(mut v_p: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("p"), v_p.clone())]))?;
    let _ = f__need({ let t1366 = Py::Float(22632.0); Py::Bool({ let t1367 = v_p.clone(); if py::le(&t1366, &t1367)? { { let t1368 = Py::Float(107500.0); if py::le(&t1367, &t1368)? { true } else { false } } } else { false } }) }, Py::str("the tropospheric standard atmosphere covers about 1075 hPa to 226 hPa"))?;
    return Ok(Py::dict(vec![(Py::str("z"), f__r(py::mul(&py::truediv(&Py::Float(288.0), &Py::Float(0.0065))?, &py::sub(&Py::Int(1i64), &py::pow(&py::truediv(&v_p.clone(), &Py::Float(101325.0))?, &py::truediv(&py::mul(&c_RD()?, &Py::Float(0.0065))?, &c_G0()?)?)?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_std_pressure(mut v_z: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("z"), v_z.clone())]))?;
    let _ = f__need({ let t1378 = py::neg(&Py::Int(500i64))?; Py::Bool({ let t1379 = v_z.clone(); if py::le(&t1378, &t1379)? { { let t1380 = Py::Int(11000i64); if py::le(&t1379, &t1380)? { true } else { false } } } else { false } }) }, Py::str("the tropospheric standard atmosphere covers -500 m to 11 km"))?;
    return Ok(Py::dict(vec![(Py::str("p"), f__r(py::mul(&Py::Float(101325.0), &py::pow(&py::sub(&Py::Int(1i64), &py::mul(&py::truediv(&Py::Float(0.0065), &Py::Float(288.0))?, &v_z.clone())?)?, &py::truediv(&c_G0()?, &py::mul(&c_RD()?, &Py::Float(0.0065))?)?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stokes_settling(mut v_d: Py, mut v_rho_s: Py, mut v_rho_f: Py, mut v_mu: Py) -> R {
    let mut v_re: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("d"), v_d.clone()), (Py::str("rho_s"), v_rho_s.clone()), (Py::str("rho_f"), v_rho_f.clone()), (Py::str("mu"), v_mu.clone())]))?;
    let _ = f__need({ let t1390 = Py::Int(0i64); Py::Bool({ let t1391 = v_d.clone(); if py::lt(&t1390, &t1391)? { { let t1392 = Py::Float(0.001); if py::le(&t1391, &t1392)? { true } else { false } } } else { false } }) }, Py::str("grain diameter must be between 0 and 1 mm"))?;
    let _ = f__need({ let t1398 = { let t1393 = v_rho_s.clone(); Py::Bool({ let t1394 = v_rho_f.clone(); if py::gt(&t1393, &t1394)? { { let t1395 = Py::Int(0i64); if py::gt(&t1394, &t1395)? { true } else { false } } } else { false } }) }; if !t1398.truthy() { t1398 } else { { let t1396 = v_mu.clone(); Py::Bool({ let t1397 = Py::Int(0i64); if py::gt(&t1396, &t1397)? { true } else { false } }) } } }, Py::str("need rho_s > rho_f > 0 and mu > 0"))?;
    v_w = py::truediv(&py::mul(&py::mul(&py::mul(&py::sub(&v_rho_s.clone(), &v_rho_f.clone())?, &c_G0()?)?, &v_d.clone())?, &v_d.clone())?, &py::mul(&Py::Int(18i64), &v_mu.clone())?)?;
    v_re = py::truediv(&py::mul(&py::mul(&v_rho_f.clone(), &v_w.clone())?, &v_d.clone())?, &v_mu.clone())?;
    let _ = f__need({ let t1399 = v_re.clone(); Py::Bool({ let t1400 = Py::Float(1.0); if py::le(&t1399, &t1400)? { true } else { false } }) }, Py::str("the grain Reynolds number exceeds 1: Stokes' law does not apply"))?;
    return Ok(Py::dict(vec![(Py::str("w"), f__r(v_w.clone())?), (Py::str("Re"), f__r(v_re.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stream_power(mut v_Q: Py, mut v_S: Py, mut v_w: Py, mut v_rho: Py) -> R {
    let mut v_om: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("S"), v_S.clone()), (Py::str("rho"), v_rho.clone())]))?;
    let _ = f__need({ let t1410 = v_Q.clone(); Py::Bool({ let t1411 = Py::Int(0i64); if py::gt(&t1410, &t1411)? { true } else { false } }) }, Py::str("the discharge must be positive"))?;
    let _ = f__need({ let t1412 = Py::Int(0i64); Py::Bool({ let t1413 = v_S.clone(); if py::lt(&t1412, &t1413)? { { let t1414 = Py::Int(1i64); if py::le(&t1413, &t1414)? { true } else { false } } } else { false } }) }, Py::str("the slope must be between 0 and 1 (m/m)"))?;
    let _ = f__need({ let t1415 = v_rho.clone(); Py::Bool({ let t1416 = Py::Int(0i64); if py::gt(&t1415, &t1416)? { true } else { false } }) }, Py::str("the water density must be positive"))?;
    v_om = py::mul(&py::mul(&py::mul(&v_rho.clone(), &c_G0()?)?, &v_Q.clone())?, &v_S.clone())?;
    v_out = Py::dict(vec![(Py::str("Omega"), f__r(v_om.clone())?)]);
    if ({ let t1422 = v_w.clone(); Py::Bool({ let t1423 = Py::None; if !py::is(&t1422, &t1423) { true } else { false } }) }).truthy() {
        let _ = f__finite(Py::dict(vec![(Py::str("w"), v_w.clone())]))?;
        let _ = f__need({ let t1424 = v_w.clone(); Py::Bool({ let t1425 = Py::Int(0i64); if py::gt(&t1424, &t1425)? { true } else { false } }) }, Py::str("the channel width must be positive"))?;
        py::setitem(&v_out.clone(), &Py::str("omega"), f__r(py::truediv(&v_om.clone(), &v_w.clone())?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_temperature_from_theta(mut v_theta: Py, mut v_p: Py) -> R {
    let _ = f__T_ok(v_theta.clone(), Py::Int(150i64), Py::Int(600i64), Py::str("potential temperature"))?;
    let _ = f__p_ok(v_p.clone(), Py::Int(100i64), Py::Int(110000i64), Py::str("pressure"))?;
    return Ok(Py::dict(vec![(Py::str("T"), f__r(py::mul(&v_theta.clone(), &py::pow(&py::truediv(&v_p.clone(), &c_P0()?)?, &c_KAPPA()?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_velocities_from_moduli(mut v_K: Py, mut v_mu: Py, mut v_rho: Py) -> R {
    let mut v_vp: Py = Py::None;
    let mut v_vs: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("K"), v_K.clone()), (Py::str("mu"), v_mu.clone()), (Py::str("rho"), v_rho.clone())]))?;
    let _ = f__need({ let t1243 = { let t1236 = v_K.clone(); Py::Bool({ let t1237 = Py::Int(0i64); if py::gt(&t1236, &t1237)? { true } else { false } }) }; if !t1243.truthy() { t1243 } else { { let t1242 = { let t1238 = v_mu.clone(); Py::Bool({ let t1239 = Py::Int(0i64); if py::gt(&t1238, &t1239)? { true } else { false } }) }; if !t1242.truthy() { t1242 } else { { let t1240 = v_rho.clone(); Py::Bool({ let t1241 = Py::Int(0i64); if py::gt(&t1240, &t1241)? { true } else { false } }) } } } } }, Py::str("bulk modulus, shear modulus and density must be positive"))?;
    let t1244 = (Py::tuple(vec![py::math1("sqrt", &py::truediv(&py::add(&v_K.clone(), &py::truediv(&py::mul(&Py::Int(4i64), &v_mu.clone())?, &Py::Int(3i64))?)?, &v_rho.clone())?)?, py::math1("sqrt", &py::truediv(&v_mu.clone(), &v_rho.clone())?)?])).items()?;
    if t1244.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vp = t1244[0].clone();
    v_vs = t1244[1].clone();
    return Ok(Py::dict(vec![(Py::str("Vp"), f__r(v_vp.clone())?), (Py::str("Vs"), f__r(v_vs.clone())?), (Py::str("Vp_Vs"), f__r(py::truediv(&v_vp.clone(), &v_vs.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_velocities_from_young(mut v_E: Py, mut v_nu: Py, mut v_rho: Py) -> R {
    let mut v_lam: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_vp: Py = Py::None;
    let mut v_vs: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("E"), v_E.clone()), (Py::str("nu"), v_nu.clone()), (Py::str("rho"), v_rho.clone())]))?;
    let _ = f__need({ let t1258 = { let t1254 = v_E.clone(); Py::Bool({ let t1255 = Py::Int(0i64); if py::gt(&t1254, &t1255)? { true } else { false } }) }; if !t1258.truthy() { t1258 } else { { let t1256 = v_rho.clone(); Py::Bool({ let t1257 = Py::Int(0i64); if py::gt(&t1256, &t1257)? { true } else { false } }) } } }, Py::str("Young's modulus and density must be positive"))?;
    let _ = f__need({ let t1259 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t1260 = v_nu.clone(); if py::lt(&t1259, &t1260)? { { let t1261 = Py::Float(0.5); if py::lt(&t1260, &t1261)? { true } else { false } } } else { false } }) }, Py::str("Poisson's ratio must be between -1 and 0.5"))?;
    let t1262 = (Py::tuple(vec![py::truediv(&py::mul(&v_E.clone(), &v_nu.clone())?, &py::mul(&py::add(&Py::Int(1i64), &v_nu.clone())?, &py::sub(&Py::Int(1i64), &py::mul(&Py::Int(2i64), &v_nu.clone())?)?)?)?, py::truediv(&v_E.clone(), &py::mul(&Py::Int(2i64), &py::add(&Py::Int(1i64), &v_nu.clone())?)?)?])).items()?;
    if t1262.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t1262[0].clone();
    v_mu = t1262[1].clone();
    let t1263 = (Py::tuple(vec![py::math1("sqrt", &py::truediv(&py::add(&v_lam.clone(), &py::mul(&Py::Int(2i64), &v_mu.clone())?)?, &v_rho.clone())?)?, py::math1("sqrt", &py::truediv(&v_mu.clone(), &v_rho.clone())?)?])).items()?;
    if t1263.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vp = t1263[0].clone();
    v_vs = t1263[1].clone();
    return Ok(Py::dict(vec![(Py::str("Vp"), f__r(v_vp.clone())?), (Py::str("Vs"), f__r(v_vs.clone())?), (Py::str("Vp_Vs"), f__r(py::truediv(&v_vp.clone(), &v_vs.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vincenty_inverse(mut v_lat1: Py, mut v_lon1: Py, mut v_lat2: Py, mut v_lon2: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_U1: Py = Py::None;
    let mut v_U2: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_a1: Py = Py::None;
    let mut v_a2: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c2a: Py = Py::None;
    let mut v_c2sm: Py = Py::None;
    let mut v_cU1: Py = Py::None;
    let mut v_cU2: Py = Py::None;
    let mut v_cl: Py = Py::None;
    let mut v_cs: Py = Py::None;
    let mut v_ds: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_new: Py = Py::None;
    let mut v_norm: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sU1: Py = Py::None;
    let mut v_sU2: Py = Py::None;
    let mut v_sa: Py = Py::None;
    let mut v_sig: Py = Py::None;
    let mut v_sl: Py = Py::None;
    let mut v_sph: Py = Py::None;
    let mut v_ss: Py = Py::None;
    let mut v_u2: Py = Py::None;
    let _ = f__coord_ok(Py::dict(vec![(Py::str("lat1"), v_lat1.clone()), (Py::str("lon1"), v_lon1.clone()), (Py::str("lat2"), v_lat2.clone()), (Py::str("lon2"), v_lon2.clone())]))?;
    v_sph = py::getitem(&f_epicentral_distance(v_lat1.clone(), v_lon1.clone(), v_lat2.clone(), v_lon2.clone())?, &Py::str("delta"))?;
    let _ = f__need({ let t344 = Py::Float(0.01); Py::Bool({ let t345 = v_sph.clone(); if py::le(&t344, &t345)? { { let t346 = Py::Int(120i64); if py::le(&t345, &t346)? { true } else { false } } } else { false } }) }, Py::str("the points must be 0.01 to 120 degrees apart (Vincenty's iteration converges slowly towards antipodes)"))?;
    let _ = f__need(py::not(&{ let t351 = { let t347 = py::abs(&v_lat1.clone())?; Py::Bool({ let t348 = Py::Float(1e-09); if py::lt(&t347, &t348)? { true } else { false } }) }; if !t351.truthy() { t351 } else { { let t349 = py::abs(&v_lat2.clone())?; Py::Bool({ let t350 = Py::Float(1e-09); if py::lt(&t349, &t350)? { true } else { false } }) } } }), Py::str("two points on the equator are not supported"))?;
    let t352 = (Py::tuple(vec![Py::Float(6378137.0), py::truediv(&Py::Int(1i64), &Py::Float(298.257223563))?])).items()?;
    if t352.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t352[0].clone();
    v_f = t352[1].clone();
    v_b = py::mul(&v_a.clone(), &py::sub(&Py::Int(1i64), &v_f.clone())?)?;
    v_norm = { let v_a = v_a.clone(); let v_b = v_b.clone(); let v_f = v_f.clone(); let v_lat1 = v_lat1.clone(); let v_lat2 = v_lat2.clone(); let v_lon1 = v_lon1.clone(); let v_lon2 = v_lon2.clone(); let v_sph = v_sph.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::modulo(&py::add(&v_x.clone(), &Py::Float(180.0))?, &Py::Float(360.0))?, &Py::Float(180.0))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_L = py::math1("radians", &py::sub(&py::call(&v_norm.clone(), &[v_lon2.clone()])?, &py::call(&v_norm.clone(), &[v_lon1.clone()])?)?)?;
    let t353 = (Py::tuple(vec![py::math1("atan", &py::mul(&py::sub(&Py::Int(1i64), &v_f.clone())?, &py::math1("tan", &py::math1("radians", &v_lat1.clone())?)?)?)?, py::math1("atan", &py::mul(&py::sub(&Py::Int(1i64), &v_f.clone())?, &py::math1("tan", &py::math1("radians", &v_lat2.clone())?)?)?)?])).items()?;
    if t353.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_U1 = t353[0].clone();
    v_U2 = t353[1].clone();
    let t354 = (Py::tuple(vec![py::math1("sin", &v_U1.clone())?, py::math1("cos", &v_U1.clone())?, py::math1("sin", &v_U2.clone())?, py::math1("cos", &v_U2.clone())?])).items()?;
    if t354.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_sU1 = t354[0].clone();
    v_cU1 = t354[1].clone();
    v_sU2 = t354[2].clone();
    v_cU2 = t354[3].clone();
    v_lam = v_L.clone();
    let mut t355 = false;
    for t356 in py::range_iter(&[(Py::Int(500i64)).i()?]) {
        v__ = t356;
        let t357 = (Py::tuple(vec![py::math1("sin", &v_lam.clone())?, py::math1("cos", &v_lam.clone())?])).items()?;
        if t357.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_sl = t357[0].clone();
        v_cl = t357[1].clone();
        v_ss = py::hypot(&[py::mul(&v_cU2.clone(), &v_sl.clone())?, py::sub(&py::mul(&v_cU1.clone(), &v_sU2.clone())?, &py::mul(&py::mul(&v_sU1.clone(), &v_cU2.clone())?, &v_cl.clone())?)?])?;
        v_cs = py::add(&py::mul(&v_sU1.clone(), &v_sU2.clone())?, &py::mul(&py::mul(&v_cU1.clone(), &v_cU2.clone())?, &v_cl.clone())?)?;
        v_sig = py::atan2(&v_ss.clone(), &v_cs.clone())?;
        v_sa = py::truediv(&py::mul(&py::mul(&v_cU1.clone(), &v_cU2.clone())?, &v_sl.clone())?, &v_ss.clone())?;
        v_c2a = py::sub(&Py::Int(1i64), &py::mul(&v_sa.clone(), &v_sa.clone())?)?;
        v_c2sm = py::sub(&v_cs.clone(), &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_sU1.clone())?, &v_sU2.clone())?, &v_c2a.clone())?)?;
        v_C = py::mul(&py::mul(&py::truediv(&v_f.clone(), &Py::Int(16i64))?, &v_c2a.clone())?, &py::add(&Py::Int(4i64), &py::mul(&v_f.clone(), &py::sub(&Py::Int(4i64), &py::mul(&Py::Int(3i64), &v_c2a.clone())?)?)?)?)?;
        v_new = py::add(&v_L.clone(), &py::mul(&py::mul(&py::mul(&py::sub(&Py::Int(1i64), &v_C.clone())?, &v_f.clone())?, &v_sa.clone())?, &py::add(&v_sig.clone(), &py::mul(&py::mul(&v_C.clone(), &v_ss.clone())?, &py::add(&v_c2sm.clone(), &py::mul(&py::mul(&v_C.clone(), &v_cs.clone())?, &py::add(&py::neg(&Py::Int(1i64))?, &py::mul(&py::mul(&Py::Int(2i64), &v_c2sm.clone())?, &v_c2sm.clone())?)?)?)?)?)?)?)?;
        if ({ let t358 = py::abs(&py::sub(&v_new.clone(), &v_lam.clone())?)?; Py::Bool({ let t359 = Py::Float(1e-15); if py::lt(&t358, &t359)? { true } else { false } }) }).truthy() {
            v_lam = v_new.clone();
            { t355 = true; break; }
        }
        v_lam = v_new.clone();
    }
    if !t355 {
        return Err(py::value_error(py::to_str(&Py::str("Vincenty's iteration did not converge"))));
    }
    let t360 = (Py::tuple(vec![py::math1("sin", &v_lam.clone())?, py::math1("cos", &v_lam.clone())?])).items()?;
    if t360.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sl = t360[0].clone();
    v_cl = t360[1].clone();
    v_ss = py::hypot(&[py::mul(&v_cU2.clone(), &v_sl.clone())?, py::sub(&py::mul(&v_cU1.clone(), &v_sU2.clone())?, &py::mul(&py::mul(&v_sU1.clone(), &v_cU2.clone())?, &v_cl.clone())?)?])?;
    v_cs = py::add(&py::mul(&v_sU1.clone(), &v_sU2.clone())?, &py::mul(&py::mul(&v_cU1.clone(), &v_cU2.clone())?, &v_cl.clone())?)?;
    v_sig = py::atan2(&v_ss.clone(), &v_cs.clone())?;
    v_sa = py::truediv(&py::mul(&py::mul(&v_cU1.clone(), &v_cU2.clone())?, &v_sl.clone())?, &v_ss.clone())?;
    v_c2a = py::sub(&Py::Int(1i64), &py::mul(&v_sa.clone(), &v_sa.clone())?)?;
    v_c2sm = py::sub(&v_cs.clone(), &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_sU1.clone())?, &v_sU2.clone())?, &v_c2a.clone())?)?;
    v_u2 = py::truediv(&py::mul(&v_c2a.clone(), &py::sub(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?)?, &py::mul(&v_b.clone(), &v_b.clone())?)?;
    v_A = py::add(&Py::Int(1i64), &py::mul(&py::truediv(&v_u2.clone(), &Py::Int(16384i64))?, &py::add(&Py::Int(4096i64), &py::mul(&v_u2.clone(), &py::add(&py::neg(&Py::Int(768i64))?, &py::mul(&v_u2.clone(), &py::sub(&Py::Int(320i64), &py::mul(&Py::Int(175i64), &v_u2.clone())?)?)?)?)?)?)?)?;
    v_B = py::mul(&py::truediv(&v_u2.clone(), &Py::Int(1024i64))?, &py::add(&Py::Int(256i64), &py::mul(&v_u2.clone(), &py::add(&py::neg(&Py::Int(128i64))?, &py::mul(&v_u2.clone(), &py::sub(&Py::Int(74i64), &py::mul(&Py::Int(47i64), &v_u2.clone())?)?)?)?)?)?)?;
    v_ds = py::mul(&py::mul(&v_B.clone(), &v_ss.clone())?, &py::add(&v_c2sm.clone(), &py::mul(&py::truediv(&v_B.clone(), &Py::Int(4i64))?, &py::sub(&py::mul(&v_cs.clone(), &py::add(&py::neg(&Py::Int(1i64))?, &py::mul(&py::mul(&Py::Int(2i64), &v_c2sm.clone())?, &v_c2sm.clone())?)?)?, &py::mul(&py::mul(&py::mul(&py::truediv(&v_B.clone(), &Py::Int(6i64))?, &v_c2sm.clone())?, &py::add(&py::neg(&Py::Int(3i64))?, &py::mul(&py::mul(&Py::Int(4i64), &v_ss.clone())?, &v_ss.clone())?)?)?, &py::add(&py::neg(&Py::Int(3i64))?, &py::mul(&py::mul(&Py::Int(4i64), &v_c2sm.clone())?, &v_c2sm.clone())?)?)?)?)?)?)?;
    v_s = py::mul(&py::mul(&v_b.clone(), &v_A.clone())?, &py::sub(&v_sig.clone(), &v_ds.clone())?)?;
    v_a1 = py::modulo(&py::math1("degrees", &py::atan2(&py::mul(&v_cU2.clone(), &v_sl.clone())?, &py::sub(&py::mul(&v_cU1.clone(), &v_sU2.clone())?, &py::mul(&py::mul(&v_sU1.clone(), &v_cU2.clone())?, &v_cl.clone())?)?)?)?, &Py::Float(360.0))?;
    v_a2 = py::modulo(&py::add(&py::math1("degrees", &py::atan2(&py::mul(&v_cU1.clone(), &v_sl.clone())?, &py::add(&py::mul(&py::neg(&v_sU1.clone())?, &v_cU2.clone())?, &py::mul(&py::mul(&v_cU1.clone(), &v_sU2.clone())?, &v_cl.clone())?)?)?)?, &Py::Float(180.0))?, &Py::Float(360.0))?;
    return Ok(Py::dict(vec![(Py::str("distance"), f__r(v_s.clone())?), (Py::str("azimuth"), f__r(v_a1.clone())?), (Py::str("back_azimuth"), f__r(v_a2.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_virtual_temperature(mut v_T: Py, mut v_w: Py) -> R {
    let _ = f__T_ok(v_T.clone(), Py::Int(150i64), Py::Int(350i64), Py::str("temperature"))?;
    let _ = f__finite(Py::dict(vec![(Py::str("w"), v_w.clone())]))?;
    let _ = f__need({ let t1448 = Py::Int(0i64); Py::Bool({ let t1449 = v_w.clone(); if py::le(&t1448, &t1449)? { { let t1450 = Py::Float(0.06); if py::le(&t1449, &t1450)? { true } else { false } } } else { false } }) }, Py::str("the mixing ratio must be a fraction between 0 and 0.06 kg/kg"))?;
    return Ok(Py::dict(vec![(Py::str("Tv"), f__r(py::truediv(&py::mul(&v_T.clone(), &py::add(&v_w.clone(), &c_EPS()?)?)?, &py::mul(&c_EPS()?, &py::add(&Py::Int(1i64), &v_w.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wave_dispersion(mut v_T: Py, mut v_h: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_dx: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("T"), v_T.clone()), (Py::str("h"), v_h.clone())]))?;
    let _ = f__need({ let t1460 = Py::Float(0.5); Py::Bool({ let t1461 = v_T.clone(); if py::le(&t1460, &t1461)? { { let t1462 = Py::Int(60i64); if py::le(&t1461, &t1462)? { true } else { false } } } else { false } }) }, Py::str("the wave period must be between 0.5 and 60 s"))?;
    let _ = f__need({ let t1463 = Py::Int(0i64); Py::Bool({ let t1464 = v_h.clone(); if py::lt(&t1463, &t1464)? { { let t1465 = Py::Int(11000i64); if py::le(&t1464, &t1465)? { true } else { false } } } else { false } }) }, Py::str("the depth must be between 0 and 11,000 m"))?;
    v_w = py::truediv(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_T.clone())?;
    v_y = py::truediv(&py::mul(&py::mul(&v_w.clone(), &v_w.clone())?, &v_h.clone())?, &c_G0()?)?;
    v_x = (if ({ let t1466 = v_y.clone(); Py::Bool({ let t1467 = Py::Int(1i64); if py::lt(&t1466, &t1467)? { true } else { false } }) }).truthy() { py::math1("sqrt", &v_y.clone())? } else { v_y.clone() });
    v_x = py::minmax(&[v_x.clone(), py::math1("sqrt", &v_y.clone())?], true)?;
    for t1468 in py::range_iter(&[(Py::Int(100i64)).i()?]) {
        v__ = t1468;
        v_t = py::math1("tanh", &v_x.clone())?;
        v_f = py::sub(&py::mul(&v_x.clone(), &v_t.clone())?, &v_y.clone())?;
        v_dx = py::truediv(&v_f.clone(), &py::add(&v_t.clone(), &py::mul(&v_x.clone(), &py::sub(&Py::Int(1i64), &py::mul(&v_t.clone(), &v_t.clone())?)?)?)?)?;
        v_x = py::sub(&v_x, &v_dx.clone())?;
        if ({ let t1469 = py::abs(&v_dx.clone())?; Py::Bool({ let t1470 = py::mul(&Py::Float(1e-15), &v_x.clone())?; if py::le(&t1469, &t1470)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    v_k = py::truediv(&v_x.clone(), &v_h.clone())?;
    v_c = py::truediv(&v_w.clone(), &v_k.clone())?;
    v_n = py::mul(&Py::Float(0.5), &py::add(&Py::Int(1i64), &(if ({ let t1471 = v_x.clone(); Py::Bool({ let t1472 = Py::Int(350i64); if py::lt(&t1471, &t1472)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&Py::Int(2i64), &v_x.clone())?, &py::math1("sinh", &py::mul(&Py::Int(2i64), &v_x.clone())?)?)? } else { Py::Float(0.0) }))?)?;
    return Ok(Py::dict(vec![(Py::str("L"), f__r(py::truediv(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_k.clone())?)?), (Py::str("c"), f__r(v_c.clone())?), (Py::str("cg"), f__r(py::mul(&v_n.clone(), &v_c.clone())?)?), (Py::str("kh"), f__r(v_x.clone())?), (Py::str("L0"), f__r(py::truediv(&py::mul(&py::mul(&c_G0()?, &v_T.clone())?, &v_T.clone())?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wien_peak(mut v_T: Py) -> R {
    let mut v_lam: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("T"), v_T.clone())]))?;
    let _ = f__need({ let t1482 = Py::Int(1i64); Py::Bool({ let t1483 = v_T.clone(); if py::le(&t1482, &t1483)? { { let t1484 = Py::Float(100000.0); if py::le(&t1483, &t1484)? { true } else { false } } } else { false } }) }, Py::str("temperature must be between 1 K and 100,000 K"))?;
    v_lam = py::truediv(&c_WIEN_B()?, &v_T.clone())?;
    return Ok(Py::dict(vec![(Py::str("lambda_max"), f__r(v_lam.clone())?), (Py::str("lambda_um"), f__r(py::mul(&v_lam.clone(), &Py::Float(1000000.0))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wind_chill(mut v_T: Py, mut v_V: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("T"), v_T.clone()), (Py::str("V"), v_V.clone())]))?;
    let _ = f__need({ let t1494 = v_T.clone(); Py::Bool({ let t1495 = Py::Float(283.15); if py::le(&t1494, &t1495)? { true } else { false } }) }, Py::str("the wind chill index is defined for temperatures of 10 °C (50 °F) or below"))?;
    let _ = f__need({ let t1496 = v_V.clone(); Py::Bool({ let t1497 = Py::Float(1.34112); if py::gt(&t1496, &t1497)? { true } else { false } }) }, Py::str("the wind chill index is defined for wind speeds above 3 mph (4.8 km/h)"))?;
    let _ = f__need({ let t1502 = { let t1498 = v_T.clone(); Py::Bool({ let t1499 = Py::Int(200i64); if py::ge(&t1498, &t1499)? { true } else { false } }) }; if !t1502.truthy() { t1502 } else { { let t1500 = v_V.clone(); Py::Bool({ let t1501 = Py::Int(60i64); if py::lt(&t1500, &t1501)? { true } else { false } }) } } }, Py::str("temperature or wind speed out of range"))?;
    v_s = py::pow(&py::mul(&v_V.clone(), &Py::Float(3.6))?, &Py::Float(0.16))?;
    v_c = py::sub(&v_T.clone(), &Py::Float(273.15))?;
    v_w = py::add(&py::sub(&py::mul(&py::add(&Py::Float(0.6215), &py::mul(&Py::Float(0.3965), &v_s.clone())?)?, &v_c.clone())?, &py::mul(&Py::Float(11.37), &v_s.clone())?)?, &Py::Float(13.12))?;
    return Ok(Py::dict(vec![(Py::str("WCT"), f__r(py::add(&v_w.clone(), &Py::Float(273.15))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wind_components(mut v_speed: Py, mut v_direction: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_snap: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("speed"), v_speed.clone()), (Py::str("direction"), v_direction.clone())]))?;
    let _ = f__need({ let t1512 = v_speed.clone(); Py::Bool({ let t1513 = Py::Int(0i64); if py::ge(&t1512, &t1513)? { true } else { false } }) }, Py::str("the wind speed cannot be negative"))?;
    let _ = f__need({ let t1514 = Py::Int(0i64); Py::Bool({ let t1515 = v_direction.clone(); if py::le(&t1514, &t1515)? { { let t1516 = Py::Int(360i64); if py::le(&t1515, &t1516)? { true } else { false } } } else { false } }) }, Py::str("the direction must be between 0 and 360 degrees"))?;
    v_d = py::math1("radians", &v_direction.clone())?;
    let t1517 = (Py::tuple(vec![py::mul(&py::neg(&v_speed.clone())?, &py::math1("sin", &v_d.clone())?)?, py::mul(&py::neg(&v_speed.clone())?, &py::math1("cos", &v_d.clone())?)?])).items()?;
    if t1517.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t1517[0].clone();
    v_v = t1517[1].clone();
    v_snap = { let v_d = v_d.clone(); let v_direction = v_direction.clone(); let v_speed = v_speed.clone(); let v_u = v_u.clone(); let v_v = v_v.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t1518 = py::abs(&v_x.clone())?; Py::Bool({ let t1519 = py::mul(&Py::Float(1e-12), &py::minmax(&[v_speed.clone(), Py::Float(1e-300)], true)?)?; if py::le(&t1518, &t1519)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::dict(vec![(Py::str("u"), f__r(py::call(&v_snap.clone(), &[v_u.clone()])?)?), (Py::str("v"), f__r(py::call(&v_snap.clone(), &[v_v.clone()])?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wind_speed_direction(mut v_u: Py, mut v_v: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("u"), v_u.clone()), (Py::str("v"), v_v.clone())]))?;
    v_s = py::hypot(&[v_u.clone(), v_v.clone()])?;
    if ({ let t1533 = { let t1529 = v_u.clone(); Py::Bool({ let t1530 = Py::Int(0i64); if py::eq(&t1529, &t1530) { true } else { false } }) }; if !t1533.truthy() { t1533 } else { { let t1531 = v_v.clone(); Py::Bool({ let t1532 = Py::Int(0i64); if py::eq(&t1531, &t1532) { true } else { false } }) } } }).truthy() {
        v_d = Py::Float(0.0);
    } else {
        v_d = py::sub(&Py::Float(90.0), &py::math1("degrees", &py::atan2(&py::neg(&v_v.clone())?, &py::neg(&v_u.clone())?)?)?)?;
        if ({ let t1534 = v_d.clone(); Py::Bool({ let t1535 = Py::Int(0i64); if py::le(&t1534, &t1535)? { true } else { false } }) }).truthy() {
            v_d = py::iadd(&v_d, &Py::Float(360.0))?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("speed"), f__r(v_s.clone())?), (Py::str("direction"), f__r(v_d.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}
