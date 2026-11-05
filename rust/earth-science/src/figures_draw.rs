//! Earth Science's drawings for the visitor's inputs: mountains and their roots (Airy isostasy), the dipole field
//! round a meridian section, the one-layer greenhouse, the daily insolation table and the wind compass. Every value
//! drawn is one the model returned (this capability solved again where several are shown).

use crate::figs_common::{self as c, area, poly, rect, varrow};
use ax_engine::figkit::{self as k, num, Canvas, Solved};
use ax_engine::figures::Figure;

pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        "airy_isostasy" => vec![isostasy(s)],
        "dipole_inclination" => vec![dipole(s)],
        "greenhouse_single_layer" => vec![greenhouse(s)],
        "daily_mean_insolation" => vec![insolation(s)],
        "wind_components" => vec![compass(s, false)],
        "wind_speed_direction" => vec![compass(s, true)],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

fn fig(s: &Solved, kind: &str, title: &str, cap: &str, cv: &Canvas) -> Option<Figure> {
    Some(s.figure(kind, title, cap, cv.body(), cv.h))
}

fn km(x: f64) -> String {
    format!("{} km", num(x / 1000.0))
}

// ------------------------------------------------------------------------------------------------ isostasy
/// Two mountains (half the given height and the given height) floating on the mantle, each with the root the model
/// returns; vertical sizes to scale, the normal crust as given (else a reference that only sets the scale).
fn isostasy(s: &Solved) -> Option<Figure> {
    let (h, root) = (c::inp(s, "h")?, s.out("root")?);
    let (rc, rm) = (c::inp(s, "rho_c")?, c::inp(s, "rho_m")?);
    // the normal crust: as given (optional input), else a 35 km reference that only sets the drawing's scale
    let given_tc = c::inp(s, "tc").filter(|x| *x > 0.0);
    let tc = given_tc.unwrap_or(35e3);
    if !(h > 0.0 && root > 0.0) {
        return None;
    }
    let a = s.with(&[("h", h / 2.0)])?;
    let root2 = k::num_of(&a, "root")?;
    let mut cv = Canvas::new(380.0);
    // vertical scale: the given mountain's column (h + tc + root) fills 270 px
    let sc = 270.0 / (h + tc + root);
    let sea = 60.0 + h * sc;                       // sea level (top of the normal crust)
    let base = sea + tc * sc;                      // base of the normal crust
    // mantle below, normal crust between sea level and its base
    cv.raw(&format!("<rect class=\"axp-s3\" x=\"30\" y=\"{base:.1}\" width=\"460\" height=\"{:.1}\"/>", 360.0 - base));
    rect(&mut cv, 30.0, sea, 460.0, tc * sc);
    // the mountains: column from the top of the topography to the bottom of its root
    for (x, w, hh, rr) in [(220.0, 80.0, h / 2.0, root2), (350.0, 110.0, h, root)] {
        let top = sea - hh * sc;
        let bot = base + rr * sc;
        area(&mut cv, &[(x, top), (x + w, top), (x + w, bot), (x, bot)], "axp-s");
        poly(&mut cv, &[(x, sea), (x, top), (x + w, top), (x + w, sea)], "axp-c", false);
        poly(&mut cv, &[(x, base), (x, bot), (x + w, bot), (x + w, base)], "axp-c", false);
        cv.value(x + w / 2.0, top - 8.0, "middle", &format!("h = {}", km(hh)));
        cv.value(x + w / 2.0, bot + 16.0, "middle", &format!("root {}", km(rr)));
    }
    cv.dashed((30.0, sea), (490.0, sea));
    cv.name(210.0, sea - 6.0, "end", "sea level");
    cv.name(36.0, sea + 18.0, "start", &format!("crust ρc = {} kg/m³", num(rc)));
    if given_tc.is_some() {
        cv.name(36.0, (sea + base) / 2.0 + 5.0, "start", &format!("normal crust {}", km(tc)));
    } else {
        cv.name(36.0, (sea + base) / 2.0, "start", &format!("reference crust {}", km(tc)));
        cv.name(36.0, (sea + base) / 2.0 + 17.0, "start", "(drawing scale)");
    }
    cv.name(36.0, 352.0, "start", &format!("mantle ρm = {} kg/m³", num(rm)));
    cv.bold(16.0, 24.0, "start", "Airy isostasy: each mountain floats on a root");
    let cap = format!("Airy isostasy: mountains of h = {} and h = {} on crust of density {} kg/m³ floating on mantle of {} \
                       kg/m³ are balanced by roots of {} and {}, as the model returns (the given one solved again at half \
                       the height). Vertical sizes to scale; {}.", km(h), km(h / 2.0), num(rc), num(rm), km(root), km(root2),
                      if given_tc.is_some() { format!("the normal crust is {} as given", km(tc)) }
                      else { "the 35 km reference crust only sets the drawing".to_string() });
    fig(s, "column", "Airy isostasy: mountains and their roots", &cap, &cv)
}

// ------------------------------------------------------------------------------------------------ dipole field
/// A meridian section of the Earth with the field direction (dip) and relative strength the model returns at
/// magnetic latitudes from 75° S to 75° N on both sides, the given latitude marked.
fn dipole(s: &Solved) -> Option<Figure> {
    let (lat, inc) = (c::inp(s, "lat")?, s.out("inclination")?);
    let f0 = s.out("F_ratio")?;
    let mut cv = Canvas::new(420.0);
    let (cx, cy, r) = (260.0, 225.0, 120.0);
    cv.raw(&format!("<circle class=\"axp-ka\" cx=\"{cx}\" cy=\"{cy}\" r=\"{r}\"/><circle class=\"axp-k\" cx=\"{cx}\" cy=\"{cy}\" r=\"{r}\"/>"));
    cv.dashed((cx, cy - r - 30.0), (cx, cy + r + 20.0));
    cv.dashed((cx - r, cy), (cx + r, cy));
    cv.name(cx, cy - r - 36.0, "middle", "dipole axis, magnetic north");
    let mut got = vec![];
    for i in -5..=5 {
        let l = 15.0 * i as f64;
        let Some(a) = s.with(&[("lat", l)]) else { continue };
        let (Some(ii), Some(fr)) = (k::num_of(&a, "inclination"), k::num_of(&a, "F_ratio")) else { continue };
        got.push((l, ii, fr));
    }
    if got.len() < 5 {
        return None;
    }
    let len = |fr: f64| 26.0 * fr;
    let draw = |cv: &mut Canvas, l: f64, ii: f64, fr: f64, class: &str| {
        for side in [1.0, -1.0] {
            let ph = l.to_radians();
            let (ir, ii_) = ((ph.cos() * side, ph.sin()), ii.to_radians());
            // north along the surface, and radially outward (math axes, y up)
            let nt = (-ph.sin() * side, ph.cos());
            let dir = (ii_.cos() * nt.0 - ii_.sin() * ir.0, ii_.cos() * nt.1 - ii_.sin() * ir.1);
            let p = (cx + r * ir.0, cy - r * ir.1);
            // the arrow centred on the surface point
            let l2 = len(fr) / 2.0;
            let a = (p.0 - dir.0 * l2, p.1 + dir.1 * l2);
            let b = (p.0 + dir.0 * l2, p.1 - dir.1 * l2);
            varrow(cv, a, b, class);
        }
    };
    for (l, ii, fr) in &got {
        draw(&mut cv, *l, *ii, *fr, if *l > 0.0 { "axp-c" } else if *l < 0.0 { "axp-c3" } else { "axp-c4" });
    }
    draw(&mut cv, lat, inc, f0, "axp-c2");
    let ph = lat.to_radians();
    let p = (cx + (r + 28.0) * ph.cos(), cy - (r + 28.0) * ph.sin());
    cv.raw(&format!("<circle class=\"axp-m\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\"/>", cx + r * ph.cos(), cy - r * ph.sin()));
    cv.value(p.0 + 8.0, p.1, "start", &format!("{}°: I = {}°", num(lat), num(inc)));
    cv.bold(16.0, 24.0, "start", &format!("At {}° the field dips {}°, ×{} the equatorial strength", num(lat), num(inc), num(f0)));
    cv.name(16.0, 404.0, "start", "arrow angle: inclination; length: strength");
    let ex: Vec<String> = got.iter().filter(|g| (g.0 as i64) % 30 == 0)
        .map(|g| format!("{}°: I = {}°, ×{}", num(g.0), num(g.1), num(g.2))).collect();
    let cap = format!("The direction and relative strength of a geocentric dipole field at the surface on a meridian section \
                       of the Earth: at each magnetic latitude from 75° S to 75° N ({} solves of the model) the arrow's angle \
                       is the returned inclination (dip below the horizontal) and its length the field strength relative \
                       to the equator. Examples: {}. Marked (green): at {}° the field dips {}°.",
                      got.len(), ex.join("; "), num(lat), num(inc));
    fig(s, "field", "Dipole field: inclination round the globe", &cap, &cv)
}

// ------------------------------------------------------------------------------------------------ greenhouse
fn greenhouse(s: &Solved) -> Option<Figure> {
    let (sol, alb, eps) = (c::inp(s, "S")?, c::inp(s, "albedo")?, c::inp(s, "eps")?);
    let (ts, ta) = (s.out("Ts")?, s.out("Ta")?);
    // the same planet without the absorbing layer, solved by the model (emissivity 0), when it accepts it
    let bare = c::at(s, &[("eps", 0.0)], "Ts");
    let mut cv = Canvas::new(330.0);
    let (layer_y, ground_y) = (110.0, 270.0);
    cv.raw(&format!("<rect class=\"axp-s3\" x=\"60\" y=\"{}\" width=\"400\" height=\"16\"/><rect class=\"axp-k\" x=\"60\" y=\"{}\"                      width=\"400\" height=\"16\"/>", layer_y - 8.0, layer_y - 8.0));
    rect(&mut cv, 60.0, ground_y, 400.0, 14.0);
    varrow(&mut cv, (110.0, 34.0), (110.0, ground_y - 2.0), "axp-c");
    varrow(&mut cv, (300.0, ground_y), (300.0, layer_y + 10.0), "axp-c2");
    varrow(&mut cv, (380.0, layer_y + 10.0), (380.0, ground_y - 2.0), "axp-c2");
    varrow(&mut cv, (440.0, layer_y - 10.0), (440.0, 30.0), "axp-c2");
    cv.name(120.0, 46.0, "start", "sunlight");
    cv.name(120.0, 63.0, "start", &format!("S = {} W/m², albedo {}", num(sol), num(alb)));
    cv.value(260.0, layer_y - 16.0, "middle", &format!("layer Ta = {}, emissivity {}", k::q(ta, "K"), num(eps)));
    cv.name(292.0, 200.0, "end", "surface emission");
    cv.name(388.0, 232.0, "start", "back-radiation");
    cv.name(448.0, 50.0, "start", "to space");
    cv.value(260.0, ground_y + 32.0, "middle", &format!("surface Ts = {}", k::q(ts, "K")));
    if let Some(b) = bare {
        cv.name(16.0, 320.0, "start", &format!("without the layer (emissivity 0): Ts = {}", k::q(b, "K")));
    }
    let cap = format!("A one-layer greenhouse: sunlight S = {} W/m² with albedo {} reaches the surface, the layer of emissivity \
                       {} absorbs the surface's infrared and re-emits it up and down. The temperatures the model returns: \
                       surface Ts = {}, layer Ta = {}{}.", num(sol), num(alb), num(eps), k::q(ts, "K"), k::q(ta, "K"),
                      bare.map(|b| format!("; the same planet without the layer (emissivity 0, solved again) has Ts = {}", k::q(b, "K")))
                          .unwrap_or_default());
    fig(s, "diagram", "One-layer greenhouse with its temperatures", &cap, &cv)
}

// ------------------------------------------------------------------------------------------------ insolation table
/// Daily mean insolation for latitudes 80° N to 80° S and five declinations (minus to plus the given one, or the
/// solstice values ±23.44° when the given one is near zero), each cell a solve with the given solar constant.
fn insolation(s: &Solved) -> Option<Figure> {
    let (lat, decl, q0) = (c::inp(s, "lat")?, c::inp(s, "decl")?, s.out("Q")?);
    let d = if decl.abs() > 1.0 { decl.abs() } else { 23.44 };
    let decls = [-d, -d / 2.0, 0.0, d / 2.0, d];
    let lats: Vec<f64> = (0..9).map(|i| 80.0 - 20.0 * i as f64).collect();
    let mut grid = vec![vec![None; 5]; 9];
    let mut mx = 0.0f64;
    for (i, la) in lats.iter().enumerate() {
        for (j, de) in decls.iter().enumerate() {
            if let Some(v) = c::at(s, &[("lat", *la), ("decl", *de)], "Q") {
                grid[i][j] = Some(v);
                mx = mx.max(v);
            }
        }
    }
    if grid.iter().flatten().filter(|v| v.is_some()).count() < 30 || !(mx > 0.0) {
        return None;
    }
    let mut cv = Canvas::new(420.0);
    let (x0, y0, cw, ch) = (130.0, 66.0, 66.0, 34.0);
    for (j, de) in decls.iter().enumerate() {
        cv.name(x0 + cw * (j as f64 + 0.5), y0 - 8.0, "middle", &format!("{}°", num(*de)));
    }
    for (i, la) in lats.iter().enumerate() {
        let lab = if *la > 0.0 { format!("{}° N", num(*la)) } else if *la < 0.0 { format!("{}° S", num(-la)) } else { "0°".into() };
        cv.name(x0 - 8.0, y0 + ch * (i as f64 + 0.62), "end", &lab);
        for j in 0..5 {
            let (x, y) = (x0 + cw * j as f64, y0 + ch * i as f64);
            match grid[i][j] {
                Some(v) => {
                    cv.raw(&format!("<rect class=\"axp-hp\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill-opacity=\"{:.2}\"/>",
                                    cw - 1.0, ch - 1.0, 0.06 + 0.7 * v / mx));
                    cv.raw(&format!("<text class=\"axp-ht\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"13\">{}</text>",
                                    x + cw / 2.0, y + ch * 0.62, v.round()));
                }
                None => cv.raw(&format!("<rect class=\"axp-f\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>", cw - 1.0, ch - 1.0)),
            }
        }
    }
    cv.bold(16.0, 24.0, "start", &format!("At {}°, δ = {}°: Q = {} W/m²", num(lat), num(decl), num(q0)));
    cv.name(x0 + 2.5 * cw, y0 - 26.0, "middle", "solar declination δ");
    cv.name(x0 + 2.5 * cw, y0 + 9.0 * ch + 22.0, "middle", "daily mean insolation (W/m²), rows 80° N to 80° S");
    let dl = s.out("day_length_h");
    let cap = format!("Daily mean solar radiation at the top of the atmosphere (solar constant {} W/m², rounded to whole W/m²) \
                       for latitudes 80° N to 80° S (rows) and solar declinations {} (columns), each of the 45 cells a solve \
                       of the model. At latitude {}° and δ = {}° the model returns {} W/m²{}.",
                      num(c::inp(s, "S0")?), decls.iter().map(|x| format!("{}°", num(*x))).collect::<Vec<_>>().join(", "),
                      num(lat), num(decl), num(q0), dl.map(|v| format!(", day length {} h", num(v))).unwrap_or_default());
    fig(s, "heatmap", "Daily insolation by latitude and declination", &cap, &cv)
}

// ------------------------------------------------------------------------------------------------ wind compass
/// The wind as a vector on a compass (north up): the arrow is where the air goes, its components u (east) and v
/// (north) drawn, the direction it comes from marked on the rim. `from_parts`: the components were given and the
/// speed and direction returned; otherwise the reverse.
fn compass(s: &Solved, from_parts: bool) -> Option<Figure> {
    let (u, v, sp, dir) = if from_parts {
        (c::inp(s, "u")?, c::inp(s, "v")?, s.out("speed")?, s.out("direction")?)
    } else {
        (s.out("u")?, s.out("v")?, c::inp(s, "speed")?, c::inp(s, "direction")?)
    };
    if !(sp > 0.0) {
        return None;
    }
    let mut cv = Canvas::new(400.0);
    let (cx, cy, r) = (260.0, 220.0, 140.0);
    // rings at round speeds
    let step = {
        let raw = sp / 2.0;
        let mag = 10f64.powf(raw.log10().floor());
        let n = raw / mag;
        mag * if n < 1.5 { 1.0 } else if n < 3.5 { 2.0 } else if n < 7.5 { 5.0 } else { 10.0 }
    };
    let rmax = (sp / step).ceil().max(1.0) * step;
    let sc = r / rmax;
    let mut ring = step;
    while ring <= rmax + 1e-9 {
        cv.raw(&format!("<circle class=\"axp-g\" fill=\"none\" cx=\"{cx}\" cy=\"{cy}\" r=\"{:.1}\"/>", ring * sc));
        cv.raw(&format!("<text class=\"axp-t\" font-size=\"11\" x=\"{:.1}\" y=\"{:.1}\">{}</text>", cx + 4.0, cy - ring * sc + 12.0,
                        num(ring)));
        ring += step;
    }
    for (a, lab) in [(0.0, "N"), (45.0, "45°"), (90.0, "E"), (135.0, "135°"), (180.0, "S"), (225.0, "225°"), (270.0, "W"), (315.0, "315°")] {
        let t = f64::to_radians(a);
        cv.raw(&format!("<line class=\"axp-g\" x1=\"{cx}\" y1=\"{cy}\" x2=\"{:.1}\" y2=\"{:.1}\"/>", cx + r * t.sin(), cy - r * t.cos()));
        cv.name(cx + (r + 16.0) * t.sin(), cy - (r + 16.0) * t.cos() + 5.0, "middle", lab);
    }
    let tip = (cx + u * sc, cy - v * sc);
    if u.abs() * sc > 2.0 {
        varrow(&mut cv, (cx, cy), (cx + u * sc, cy), "axp-c2");
    }
    if v.abs() * sc > 2.0 {
        varrow(&mut cv, (cx + u * sc, cy), tip, "axp-c3");
    }
    varrow(&mut cv, (cx, cy), tip, "axp-c");
    let t = dir.to_radians();
    let fp = (cx + r * t.sin(), cy - r * t.cos());
    cv.raw(&format!("<circle class=\"axp-m\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"6\"/>", fp.0, fp.1));
    let inward = (cx + (r - 30.0) * t.sin(), cy - (r - 30.0) * t.cos());
    cv.value(inward.0, inward.1 + 5.0, "middle", &format!("from {}°", num(dir)));
    cv.bold(16.0, 24.0, "start", &format!("speed {} from {}°", k::q(sp, "m/s"), num(dir)));
    cv.value(16.0, 44.0, "start", &format!("u = {} (east), v = {} (north)", k::q(k::snap(u, sp), "m/s"), k::q(k::snap(v, sp), "m/s")));
    cv.name(16.0, 392.0, "start", "arrow: where the air goes; dot: where it comes from");
    let (title, cap) = if from_parts {
        ("Wind speed and direction from its components",
         format!("The given wind components u = {} m/s and v = {} m/s as a vector on a compass (north up); the model returns \
                  speed {} m/s from {}° (where the air comes from, opposite to the arrow). Marked: that direction.",
                 num(u), num(v), num(sp), num(dir)))
    } else {
        ("Wind vector and its components",
         format!("A wind of {} m/s blowing from {}° (as given, meteorological convention) on a compass (north up): the air \
                  moves towards the opposite side. Its components returned by the model: u = {} m/s (eastward, green), v = {} \
                  m/s (northward, violet). Marked: the direction it comes from.", num(sp), num(dir), num(k::snap(u, sp)),
                 num(k::snap(v, sp))))
    };
    fig(s, "vector", title, &cap, &cv)
}
