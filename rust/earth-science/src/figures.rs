//! Earth Science's own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model computes:
//! values over two inputs, vertical profiles of the atmosphere, the crust and the ocean, decay and travel-time charts,
//! fitted data here; the drawings (isostasy, dipole field, greenhouse layer, insolation table, wind compass) in
//! figures_draw.rs (docs/FIGURE_PARITY.md). Helpers: figs_common.rs, figs_more.rs.

use crate::figs_common::{self as c, esc, qv};
use crate::figs_more::{self as m, dec, fx};
use ax_engine::figkit::{self as k, num, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        // a value over two inputs
        "bouguer_slab" => vec![m::field_half(s, "field", "h", "drho", "dg_mGal")],
        "brunt_vaisala_frequency" => vec![m::field_half(s, "field", "dtheta_dz", "theta", "N")],
        "co2_radiative_forcing" => vec![m::field_half(s, "field", "C", "C0", "dF")],
        "ekman_depth" => vec![lat_field(s, "Az", "D_E")],
        "local_magnitude" => vec![dec_field(s, "r", "A", "ML")],
        "mixing_ratio" => vec![m::field_half(s, "field", "e", "p", "w")],
        "ocean_depth_from_pressure" => vec![lat_field(s, "p", "depth")],
        "ocean_pressure_from_depth" => vec![lat_field(s, "depth", "p")],
        "planetary_equilibrium_temperature" => vec![frac_field(s, "S", "albedo", "Te")],
        "radioactive_decay_remaining" => vec![decay(s), m::field_half(s, "field", "half_life", "t", "fraction")],
        "radiometric_age" => vec![m::field_half(s, "field", "half_life", "ratio", "age")],
        "stream_power" => vec![m::field_half(s, "field", "Q", "S", "Omega")],
        "temperature_from_potential_temperature" => vec![m::field_half(s, "field", "theta", "p", "T")],
        "virtual_temperature" => vec![m::field_half(s, "field", "T", "w", "Tv")],
        "wave_dispersion" => vec![dec_field(s, "T", "h", "L")],
        // profiles
        "dry_adiabatic_ascent" => vec![parcel(s)],
        "geotherm_radiogenic" => vec![geotherm(s)],
        "isothermal_pressure" => vec![p_height(s, "isothermal_pressure", 20e3, 1.5)],
        "standard_atmosphere_pressure" => vec![p_height(s, "standard_atmosphere_pressure", 11e3, 1.5)],
        "pressure_with_lapse_rate" => vec![lapse_profile(s), p_height(s, "pressure_with_lapse_rate", 11e3, 1.3)],
        "seawater_density" => vec![ocean_profile(s, "rho", "ρ, in-situ density (kg/m³)", 1.0)],
        "sound_speed_seawater" => vec![ocean_profile(s, "c", "c, speed of sound (m/s)", 1.0)],
        // curves and data
        "saturation_vapor_pressure" => vec![svp(s)],
        "sp_epicentral_distance" => vec![travel_times(s)],
        "isochron_age" => vec![isochron(s)],
        "nmo_velocity" => vec![nmo(s)],
        _ => vec![],
    };
    let mut out: Vec<Figure> = v.into_iter().flatten().collect();
    out.extend(crate::figures_draw::live(s));
    out
}

// ------------------------------------------------------------------------------------------------ value maps
/// `z` over `x` (half to one and a half times) and the latitude (its own hemisphere, equator to pole).
fn lat_field(s: &Solved, x: &str, z: &str) -> Option<Figure> {
    let (x0, lat) = (c::inp(s, x)?, c::inp(s, "lat")?);
    let ry = if lat >= 0.0 { (0.5, 89.5, false) } else { (-89.5, -0.5, false) };
    m::field_xy(s, "field", x, fx(x0, 0.5, 1.5), "lat", ry, z)
}

/// `z` over `x` and `y`, each a tenth to ten times the given value (logarithmic axes).
fn dec_field(s: &Solved, x: &str, y: &str, z: &str) -> Option<Figure> {
    let (x0, y0) = (c::inp(s, x)?, c::inp(s, y)?);
    if !(x0 > 0.0 && y0 > 0.0) {
        return None;
    }
    m::field_xy(s, "field", x, dec(x0), y, dec(y0), z)
}

/// `z` over `x` (half to one and a half times) and a fraction `f` (2 % to 98 %, or wider round a larger value).
fn frac_field(s: &Solved, x: &str, f: &str, z: &str) -> Option<Figure> {
    let (x0, f0) = (c::inp(s, x)?, c::inp(s, f)?);
    let hi = if f0 < 0.9 { 0.98 } else { f0 * 1.5 };
    m::field_xy(s, "field", x, fx(x0, 0.5, 1.5), f, (0.02f64.min(f0 * 0.5), hi, false), z)
}

// ------------------------------------------------------------------------------------------------ an x-y plot of its own
/// One drawn series: name (legend), points, line class, dashed, dots instead of a line.
pub struct Ser {
    pub name: String,
    pub pts: Vec<(f64, f64)>,
    pub class: &'static str,
    pub dashed: bool,
    pub dots: bool,
}

pub fn ser(name: &str, pts: Vec<(f64, f64)>, class: &'static str) -> Ser {
    Ser { name: name.to_string(), pts, class, dashed: false, dots: false }
}

/// Round tick values over [lo, hi] (either order), including round ends.
fn nticks(a: f64, b: f64, n: usize) -> Vec<f64> {
    let (lo, hi) = (a.min(b), a.max(b));
    let span = hi - lo;
    if !(span > 0.0) || !span.is_finite() {
        return vec![lo];
    }
    let raw = span / n as f64;
    let mag = 10f64.powf(raw.log10().floor());
    let norm = raw / mag;
    let step = mag * if norm < 1.5 { 1.0 } else if norm < 3.0 { 2.0 } else if norm < 7.0 { 5.0 } else { 10.0 };
    let mut v = vec![];
    let mut t = (lo / step - 1e-9).ceil() * step;
    while t <= hi + step * 1e-9 && v.len() < 14 {
        v.push(if t.abs() < step * 1e-9 { 0.0 } else { t });
        t += step;
    }
    v
}

fn dot_class(class: &str) -> &'static str {
    match class {
        "axp-c" => "axp-d",
        "axp-c2" => "axp-d2",
        "axp-c3" => "axp-d3",
        _ => "axp-d4",
    }
}

/// A plot of series over measured axes: `xr` left to right, `yr` bottom to top (a depth axis: `yr` = (deepest, 0));
/// a legend above the plot, the marked point labelled. None when nothing finite remains.
#[allow(clippy::too_many_arguments)]
pub fn xy(s: &Solved, kind: &str, title: &str, cap: &str, series: &[Ser], xr: (f64, f64), yr: (f64, f64), xlab: &str,
          ylab: &str, marker: Option<((f64, f64), String)>) -> Option<Figure> {
    let finite = |p: &(f64, f64)| p.0.is_finite() && p.1.is_finite();
    if series.iter().map(|x| x.pts.iter().filter(|p| finite(p)).count()).sum::<usize>() < 2 {
        return None;
    }
    if !(xr.1 > xr.0) || yr.0 == yr.1 || !xr.0.is_finite() || !xr.1.is_finite() || !yr.0.is_finite() || !yr.1.is_finite() {
        return None;
    }
    let named: Vec<&Ser> = series.iter().filter(|x| !x.name.is_empty()).collect();
    // two legend entries a row when short, else one a row
    let per = if named.iter().all(|x| x.name.chars().count() <= 26) { 2 } else { 1 };
    let rows = named.len().min(4).div_ceil(per);
    let (x0, x1) = (96.0, 504.0);
    let (y0, y1) = (34.0 + 18.0 * rows as f64, 278.0);
    let px = |v: f64| x0 + (v - xr.0) / (xr.1 - xr.0) * (x1 - x0);
    let py = |v: f64| y1 - (v - yr.0) / (yr.1 - yr.0) * (y1 - y0);
    let mut out = String::new();
    for t in nticks(xr.0, xr.1, 5) {
        let x = px(t);
        out.push_str(&format!("<line class=\"axp-g\" x1=\"{x:.1}\" y1=\"{y0:.1}\" x2=\"{x:.1}\" y2=\"{y1:.1}\"/>\
                               <text class=\"axp-t\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>", y1 + 20.0, esc(&num(t))));
    }
    for t in nticks(yr.0, yr.1, 5) {
        let y = py(t);
        out.push_str(&format!("<line class=\"axp-g\" x1=\"{x0:.1}\" y1=\"{y:.1}\" x2=\"{x1:.1}\" y2=\"{y:.1}\"/>\
                               <text class=\"axp-t\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>", x0 - 6.0, y + 5.0, esc(&num(t))));
    }
    out.push_str(&format!("<rect class=\"axp-f\" x=\"{x0}\" y=\"{y0:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>", x1 - x0, y1 - y0));
    out.push_str(&format!("<text class=\"axp-l\" x=\"{:.1}\" y=\"326\" text-anchor=\"middle\">{}</text>", (x0 + x1) / 2.0, esc(xlab)));
    out.push_str(&format!("<text class=\"axp-l\" transform=\"translate(16,{:.1}) rotate(-90)\" text-anchor=\"middle\">{}</text>",
                          (y0 + y1) / 2.0, esc(ylab)));
    out.push_str(&format!("<clipPath id=\"CLIPID\"><rect x=\"{x0}\" y=\"{y0:.1}\" width=\"{:.1}\" height=\"{:.1}\"/></clipPath>\
                           <g clip-path=\"url(#CLIPID)\">", x1 - x0, y1 - y0));
    for sr in series {
        if sr.dots {
            for p in sr.pts.iter().filter(|p| finite(p)) {
                out.push_str(&format!("<circle class=\"{}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"3.5\"/>", dot_class(sr.class), px(p.0), py(p.1)));
            }
            continue;
        }
        let mut d = String::new();
        let mut pen = false;
        for p in &sr.pts {
            if finite(p) {
                d.push_str(&format!("{}{:.1},{:.1} ", if pen { "L" } else { "M" }, px(p.0), py(p.1)));
                pen = true;
            } else {
                pen = false;
            }
        }
        if !d.is_empty() {
            out.push_str(&format!("<path class=\"{}\"{} d=\"{}\"/>", sr.class,
                                  if sr.dashed { " stroke-dasharray=\"6 4\"" } else { "" }, d.trim_end()));
        }
    }
    out.push_str("</g>");
    // legend: up to two per row
    for (i, sr) in named.iter().enumerate().take(4) {
        let (lx, ly) = (x0 - 10.0 + 210.0 * (i % per) as f64, 22.0 + 18.0 * (i / per) as f64);
        if sr.dots {
            out.push_str(&format!("<circle class=\"{}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\"/>", dot_class(sr.class), lx + 8.0, ly - 4.0));
        } else {
            out.push_str(&format!("<path class=\"{}\"{} d=\"M{lx:.1},{:.1} L{:.1},{:.1}\"/>", sr.class,
                                  if sr.dashed { " stroke-dasharray=\"6 4\"" } else { "" }, ly - 4.0, lx + 18.0, ly - 4.0));
        }
        out.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{ly:.1}\">{}</text>", lx + 24.0, esc(&m_clip(&sr.name, if per == 2 { 26 } else { 52 }))));
    }
    if let Some(((mx, my), lab)) = marker {
        if mx.is_finite() && my.is_finite() {
            let (qx, qy) = (px(mx), py(my));
            let w = 9.0 * lab.chars().count() as f64;
            let (tx, anchor) = if qx + 10.0 + w <= x1 { (qx + 10.0, "start") } else { ((qx - 10.0).max(x0 + w), "end") };
            let ty = if qy < y0 + 26.0 { qy + 22.0 } else { qy - 10.0 };
            out.push_str(&format!("<circle class=\"axp-m\" cx=\"{qx:.1}\" cy=\"{qy:.1}\" r=\"6\"/>\
                                   <text class=\"axp-mt\" x=\"{tx:.1}\" y=\"{ty:.1}\" text-anchor=\"{anchor}\">{}</text>", esc(&lab)));
        }
    }
    Some(s.chart(kind, title, cap, &out))
}

fn m_clip(t: &str, n: usize) -> String {
    let cs: Vec<char> = t.chars().collect();
    if cs.len() <= n { t.to_string() } else { format!("{}...", cs[..n - 3].iter().collect::<String>().trim_end()) }
}

/// The extent of finite values, padded by a fraction.
fn ext(v: impl Iterator<Item = f64>, pad: f64) -> Option<(f64, f64)> {
    let (lo, hi) = v.filter(|x| x.is_finite()).fold((f64::INFINITY, f64::NEG_INFINITY), |a, x| (a.0.min(x), a.1.max(x)));
    if !lo.is_finite() {
        return None;
    }
    let d = if hi > lo { (hi - lo) * pad } else { lo.abs().max(1.0) * 0.1 };
    Some((lo - d, hi + d))
}

// ------------------------------------------------------------------------------------------------ profiles
/// A dry parcel's temperature with height: T2 against the rise from 0 to the larger of 6 km and 1.3 times the rise.
fn parcel(s: &Solved) -> Option<Figure> {
    let (t, dz, t2) = (c::inp(s, "T")?, c::inp(s, "dz")?, s.out("T2")?);
    let top = (6000f64).max(dz * 1.3);
    let zs = k::span(0.0, top, 41, false);
    let pts: Vec<(f64, f64)> = s.curve("dz", &zs, "T2", &[]).into_iter().map(|(z, tt)| (tt, z / 1000.0)).collect();
    if pts.len() < 3 {
        return None;
    }
    let xr = ext(pts.iter().map(|p| p.0).chain([t2]), 0.06)?;
    let cap = format!("Temperature of a dry parcel rising from T = {} through 0 to {} km, drawn with height upward: {} solves \
                       of the model. Marked: a rise of {} gives T2 = {}.", qv(t, "K"), num(top / 1000.0), pts.len(),
                      qv(dz, "m"), qv(t2, "K"));
    xy(s, "profile", "Dry-adiabatic ascent: parcel temperature with height", &cap,
       &[ser("parcel temperature", pts, "axp-c")], xr, (0.0, top / 1000.0), "T2, parcel temperature (K)",
       "rise of the parcel (km)", Some(((t2, dz / 1000.0), qv(t2, "K"))))
}

/// The geotherm: temperature with depth from 0 to the larger of 40 km and 1.2 times the given depth, depth downward.
fn geotherm(s: &Solved) -> Option<Figure> {
    let (z, t) = (c::inp(s, "z")?, s.out("T")?);
    let bottom = 40e3f64.max(z * 1.2);
    let zs = k::span(0.0, bottom, 41, false);
    let pts: Vec<(f64, f64)> = s.curve("z", &zs, "T", &[]).into_iter().map(|(zz, tt)| (tt, zz / 1000.0)).collect();
    if pts.len() < 3 {
        return None;
    }
    let xr = ext(pts.iter().map(|p| p.0).chain([t]), 0.06)?;
    let q = s.out("q");
    let cap = format!("Steady temperature with depth in a crust with radiogenic heating, from the surface to {} km, depth \
                       increasing downward: {} solves of the model with the inputs as given. Marked: at z = {} the model \
                       returns T = {}{}.", num(bottom / 1000.0), pts.len(), qv(z, "m"), qv(t, "K"),
                      q.map(|v| format!(" and a heat flow of {} W/m²", num(v))).unwrap_or_default());
    xy(s, "profile", "Geotherm with radiogenic heating", &cap, &[ser("temperature", pts, "axp-c")], xr,
       (bottom / 1000.0, 0.0), "T, temperature (K)", "depth (km)", Some(((t, z / 1000.0), qv(t, "K"))))
}

/// Pressure with height (kPa against km) from this capability solved at many heights: up to the larger of `top` and
/// `f` times the given height.
fn p_height(s: &Solved, key: &str, top: f64, f: f64) -> Option<Figure> {
    let (z, p) = (c::inp(s, "z")?, s.out("p")?);
    let top = top.max(z * f);
    let zs = k::span(0.0, top, 41, false);
    let pts: Vec<(f64, f64)> = s.curve("z", &zs, "p", &[]).into_iter().map(|(zz, pp)| (pp / 1000.0, zz / 1000.0)).collect();
    if pts.len() < 3 {
        return None;
    }
    let xr = ext(pts.iter().map(|q| q.0).chain([p / 1000.0]), 0.05)?;
    let kind = if key == "pressure_with_lapse_rate" { "profile-2" } else { "profile" };
    let what = match key {
        "isothermal_pressure" => "Isothermal atmosphere: pressure with height",
        "standard_atmosphere_pressure" => "Standard atmosphere: pressure with height",
        _ => "Pressure with height (constant lapse rate)",
    };
    let cap = format!("{what}: p against z from 0 to {} km, drawn with height upward; {} solves of the model with the other \
                       inputs as given. Marked: at z = {} the model returns p = {}.", num(top / 1000.0), pts.len(), qv(z, "m"),
                      q_kpa(p));
    xy(s, kind, what, &cap, &[ser("pressure", pts, "axp-c")], xr, (0.0, top / 1000.0), "p, pressure (kPa)",
       "z, height (km)", Some(((p / 1000.0, z / 1000.0), q_kpa(p))))
}

fn q_kpa(p: f64) -> String {
    format!("{} kPa", num(p / 1000.0))
}

/// Temperature with height under the constant lapse rate, beside a dry parcel lifted from the same surface
/// temperature (the model's dry-adiabatic capability), to 11 km.
fn lapse_profile(s: &Solved) -> Option<Figure> {
    let (t0, gam, z, t, p) = (c::inp(s, "T0")?, c::inp(s, "gamma")?, c::inp(s, "z")?, s.out("T")?, s.out("p")?);
    let top = 11e3f64.max(z * 1.2);
    let zs = k::span(0.0, top, 56, false);
    let atm: Vec<(f64, f64)> = s.curve("z", &zs, "T", &[]).into_iter().map(|(zz, tt)| (tt, zz / 1000.0)).collect();
    let mut parcel = vec![];
    for zz in zs.iter().step_by(5) {
        let a = s.solve_other("dry_adiabatic_ascent", &[("T", Input::Float(t0)), ("dz", Input::Float(*zz))]);
        if let Some(v) = a.and_then(|a| k::num_of(&a, "T2")) {
            parcel.push((v, zz / 1000.0));
        }
    }
    if atm.len() < 3 {
        return None;
    }
    let xr = ext(atm.iter().chain(parcel.iter()).map(|q| q.0), 0.05)?;
    let mut series = vec![ser(&format!("atmosphere, lapse rate {} K/km", num(gam * 1000.0)), atm, "axp-c")];
    let np = parcel.len();
    if np >= 2 {
        let mut d = ser("dry-adiabatic parcel from the surface", parcel, "axp-c3");
        d.dashed = true;
        series.insert(0, d);
    }
    let cap = format!("Temperature against height in an atmosphere with a constant lapse rate of {} K/km from {} at the \
                       surface{}, solved by the model at {} heights to {} km. Marked: at z = {}, T = {} and p = {}.",
                      num(gam * 1000.0), qv(t0, "K"),
                      if np >= 2 { format!(", and a dry parcel lifted adiabatically from the same surface temperature ({np} solves of the model's dry-adiabatic capability)") } else { String::new() },
                      zs.len(), num(top / 1000.0), qv(z, "m"), qv(t, "K"), q_kpa(p));
    xy(s, "profile", "Temperature profile of the lower atmosphere", &cap, &series, xr, (0.0, top / 1000.0),
       "T, temperature (K)", "z, height (km)", Some(((t, z / 1000.0), format!("{}, {}", qv(t, "K"), q_kpa(p)))))
}

/// A property of seawater with pressure, 0 to the larger of 6000 dbar and 1.2 times the given pressure, depth (as
/// pressure) downward.
fn ocean_profile(s: &Solved, y: &str, xlab: &str, f: f64) -> Option<Figure> {
    let (p, v) = (c::inp(s, "p")?, s.out(y)?);
    let bottom = 6.0e7f64.max(p * 1.2);
    let ps = k::span(0.0, bottom, 41, false);
    let pts: Vec<(f64, f64)> = s.curve("p", &ps, y, &[]).into_iter().map(|(pp, vv)| (vv * f, pp / 1e4)).collect();
    if pts.len() < 3 {
        return None;
    }
    let xr = ext(pts.iter().map(|q| q.0).chain([v * f]), 0.06)?;
    let u = c::unit(s, y);
    let what = if y == "rho" { "Seawater density with pressure" } else { "Sound speed with pressure" };
    let cap = format!("{what}: {y} against sea pressure from 0 to {} dbar, drawn with depth increasing downward; {} solves of \
                       the model with S and T as given. Marked: at {} dbar the model returns {y} = {}.", num(bottom / 1e4),
                      pts.len(), num(p / 1e4), qv(v, &u));
    xy(s, "profile", what, &cap, &[ser(y, pts, "axp-c")], xr, (bottom / 1e4, 0.0), xlab, "depth as sea pressure (dbar)",
       Some(((v * f, p / 1e4), qv(v, &u))))
}

// ------------------------------------------------------------------------------------------------ curves and data
fn svp(s: &Solved) -> Option<Figure> {
    let t = c::inp(s, "T")?;
    m::curve1(s, "curve-2", "T", fx(t, 0.9, 1.1), "es")
}

/// The fraction remaining with time (41 solves), with a dot at each of the first five half-lives.
fn decay(s: &Solved) -> Option<Figure> {
    let (hl, t, fr) = (c::inp(s, "half_life")?, c::inp(s, "t")?, s.out("fraction")?);
    if !(hl > 0.0) {
        return None;
    }
    let end = (5.2 * hl).max(t * 1.2);
    let ts = k::span(0.0, end, 41, false);
    let curve = s.curve("t", &ts, "fraction", &[]);
    let halves: Vec<(f64, f64)> = (1..=5).filter_map(|i| {
        let tt = hl * i as f64;
        c::at(s, &[("t", tt)], "fraction").map(|v| (tt, v))
    }).collect();
    if curve.len() < 3 {
        return None;
    }
    let mut hs = ser(&format!("each half-life ({} years)", num(hl)), halves.clone(), "axp-c2");
    hs.dots = true;
    let cap = format!("Fraction of the parent remaining (half-life {} years) from 0 to {} years: {} solves of the model, and \
                       the fraction at each of the first five half-lives ({}). Marked: t = {} years, fraction = {}.",
                      num(hl), num(end), curve.len(),
                      halves.iter().map(|h| num(h.1)).collect::<Vec<_>>().join(", "), num(t), num(fr));
    xy(s, "decay", "Radioactive decay: fraction remaining with time", &cap,
       &[ser("fraction remaining", curve, "axp-c"), hs], (0.0, end), (0.0, 1.05), "t, time (years)",
       "fraction remaining", Some(((t, fr), num(fr))))
}

/// P and S travel times against distance (S-P intervals from a tenth to three times the given one, each a solve),
/// the given interval drawn as the gap between the curves at its distance.
fn travel_times(s: &Solved) -> Option<Figure> {
    let (dt, vp, vs) = (c::inp(s, "dt")?, c::inp(s, "vp")?, c::inp(s, "vs")?);
    let (d, tp, ts) = (s.out("distance")?, s.out("tp")?, s.out("ts")?);
    let mut p = vec![];
    let mut sw = vec![];
    for x in k::span(dt * 0.1, dt * 3.0, 30, false) {
        if let Some(a) = s.with(&[("dt", x)]) {
            if let (Some(dd), Some(a1), Some(a2)) = (k::num_of(&a, "distance"), k::num_of(&a, "tp"), k::num_of(&a, "ts")) {
                p.push((dd / 1000.0, a1));
                sw.push((dd / 1000.0, a2));
            }
        }
    }
    if p.len() < 3 {
        return None;
    }
    let xmax = p.iter().map(|q| q.0).fold(0.0, f64::max).max(d / 1000.0) * 1.05;
    let ymax = sw.iter().map(|q| q.1).fold(0.0, f64::max).max(ts) * 1.08;
    let gap = vec![(d / 1000.0, tp), (d / 1000.0, ts)];
    let cap = format!("P and S arrival times against distance (P {} m/s, S {} m/s as given): {} solves of the model for S-P \
                       intervals from {} to {} s. The given interval {} s fixes the distance the model returns: {} km (P \
                       after {} s, S after {} s).", num(vp), num(vs), p.len(), num(dt * 0.1), num(dt * 3.0), num(dt),
                      num(d / 1000.0), num(tp), num(ts));
    xy(s, "diagram", "Travel-time diagram: distance from the S-P interval", &cap,
       &[ser(&format!("P wave ({} m/s)", num(vp)), p, "axp-c"), ser(&format!("S wave ({} m/s)", num(vs)), sw, "axp-c2"),
         ser(&format!("S - P = {} s", num(dt)), gap, "axp-c3")],
       (0.0, xmax), (0.0, ymax), "epicentral distance (km)", "travel time (s)", Some(((d / 1000.0, ts), format!("{} km", num(d / 1000.0)))))
}

/// The isochron: the sample ratios as given and the straight line of the returned slope and initial ratio.
fn isochron(s: &Solved) -> Option<Figure> {
    let (x, y) = (m::list(s, "x")?, m::list(s, "y")?);
    let (age, slope, y0) = (s.out("age")?, s.out("slope")?, s.out("initial_ratio")?);
    if x.len() != y.len() || x.len() < 2 {
        return None;
    }
    let (xl, xh) = x.iter().fold((0.0f64, f64::NEG_INFINITY), |a, b| (a.0.min(*b), a.1.max(*b)));
    let xr = (xl, xh * 1.08);
    let line = vec![(xr.0, y0 + slope * xr.0), (xr.1, y0 + slope * xr.1)];
    let pts: Vec<(f64, f64)> = x.iter().copied().zip(y.iter().copied()).collect();
    let yr = ext(pts.iter().map(|p| p.1).chain(line.iter().map(|p| p.1)), 0.08)?;
    let mut data = ser("samples (as given)", pts.clone(), "axp-c2");
    data.dots = true;
    let cap = format!("The sample ratios y against x as given ({} samples) and the isochron through them with the returned \
                       slope {} and initial ratio {}: age {} years.", pts.len(), num(slope), num(y0), num(age));
    xy(s, "xy", "Isochron age: the samples and the returned isochron", &cap,
       &[ser(&format!("isochron, age {} years", num(age)), line, "axp-c"), data], xr, yr,
       "x, parent/stable isotope ratio", "y, daughter/stable ratio", Some(((0.0, y0), format!("initial {}", num(y0)))))
}

/// Reflection picks as given; the returned velocity, zero-offset time and depth stated.
fn nmo(s: &Solved) -> Option<Figure> {
    let (x, t) = (m::list(s, "x")?, m::list(s, "t")?);
    let (v, t0, dep) = (s.out("V_nmo")?, s.out("t0")?, s.out("depth")?);
    if x.len() != t.len() || x.len() < 2 {
        return None;
    }
    let pts: Vec<(f64, f64)> = x.iter().copied().zip(t.iter().copied()).collect();
    let xr = ext(pts.iter().map(|p| p.0).chain([0.0]), 0.05)?;
    let yr = ext(pts.iter().map(|p| p.1).chain([t0]), 0.1)?;
    let mut data = ser(&format!("picks as given; V = {} m/s, depth {} m", num(v), num(dep)),
                       pts.clone(), "axp-c2");
    data.dots = true;
    let cap = format!("Two-way reflection times t against source-receiver offset x as given ({} picks). Returned by the model: \
                       NMO velocity {} m/s, zero-offset time t0 = {} s (marked at zero offset), reflector depth {} m.",
                      pts.len(), num(v), num(t0), num(dep));
    xy(s, "xy", "Normal-moveout velocity: the picks and what was returned", &cap, &[data], xr, yr,
       "x, source-receiver offset (m)", "t, two-way time (s)",
       Some(((0.0, t0), format!("t0 = {} s", num(t0)))))
}
