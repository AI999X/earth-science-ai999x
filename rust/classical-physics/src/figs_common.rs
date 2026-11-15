//! Figure helpers shared by the physics-family models (Classical Physics, Quantum Physics, Astronomy, Nuclear
//! Engineering): a quantity's meaning and unit as the model states them, a value over two inputs (a shaded map with
//! iso-lines), curve families, bars over named cases, and drawing helpers (vectors, springs, bodies). Everything drawn
//! comes from solves of the model itself (ax_engine::figkit::Solved); a figure that cannot be drawn is None.
//!
//! Included by path from the other crates (`#[path = "../../classical-physics/src/figs_common.rs"] mod common;`).

#![allow(dead_code)]

use ax_engine::figkit::{self as k, num, q, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::{Answer, Input};

// ------------------------------------------------------------------------------------------------ names and units
/// The unit of a quantity of this capability (a formula's symbol, a procedure's input or output), as the pack has it;
/// "" for a pure number.
pub fn unit(s: &Solved, sym: &str) -> String {
    let kind = if let Some(r) = s.p.relation(s.key) {
        r.symbols.iter().find(|x| x.name == sym).map(|x| x.kind.clone())
    } else if let Some(p) = s.p.procedure(s.key) {
        p.inputs.iter().find(|x| x.symbol == sym).map(|x| x.kind.clone())
            .or_else(|| p.outputs.iter().find(|x| x.symbol == sym).map(|x| x.kind.clone()))
    } else {
        None
    };
    let u = kind.and_then(|kd| s.p.kinds.get(&kd).map(|x| x.unit.clone())).unwrap_or_default();
    if u == "1" { String::new() } else { u }
}

/// What a quantity means ("mass"), as the pack words it; the symbol when it has no words.
pub fn meaning(s: &Solved, sym: &str) -> String {
    let m = if let Some(r) = s.p.relation(s.key) {
        r.symbols.iter().find(|x| x.name == sym).map(|x| if x.desc.is_empty() { x.kind.replace('_', " ") } else { x.desc.clone() })
    } else if let Some(p) = s.p.procedure(s.key) {
        p.inputs.iter().find(|x| x.symbol == sym).map(|x| x.meaning.clone())
            .or_else(|| p.outputs.iter().find(|x| x.symbol == sym).map(|x| x.meaning.clone()))
    } else {
        None
    };
    m.filter(|x| !x.is_empty()).unwrap_or_else(|| sym.to_string())
}

/// The capability's name ("Linear momentum").
pub fn name(s: &Solved) -> String {
    s.p.relation(s.key).map(|r| r.name.clone())
        .or_else(|| s.p.procedure(s.key).map(|p| p.name.clone())).unwrap_or_else(|| s.key.replace('_', " "))
}

/// A unit written for reading: m/s^2 -> m/s², kg*m/s -> kg·m/s.
pub fn pu(u: &str) -> String {
    u.replace("^-1", "⁻¹").replace("^-2", "⁻²").replace("^-3", "⁻³").replace("^2", "²").replace("^3", "³")
        .replace("^4", "⁴").replace('*', "·")
}

/// A value with its unit (SI prefixes where they apply).
pub fn qv(x: f64, u: &str) -> String {
    let u2 = if u == "1" { "" } else { u };
    if u2.contains('^') || u2.contains('*') || u2.contains('/') {
        let n = num(x);
        if u2.is_empty() { n } else { format!("{n} {}", pu(u2)) }
    } else {
        q(x, u2)
    }
}

/// "sym = value unit" with the unit of this capability's quantity.
pub fn eqs(s: &Solved, sym: &str, x: f64) -> String {
    format!("{sym} = {}", qv(x, &unit(s, sym)))
}

/// An axis label "m, mass (kg)".
pub fn axis(s: &Solved, sym: &str) -> String {
    let u = unit(s, sym);
    let m = meaning(s, sym);
    let full = if u.is_empty() { format!("{sym}, {m}") } else { format!("{sym}, {m} ({})", pu(&u)) };
    if full.chars().count() <= 34 {
        return full;
    }
    // too long for a vertical axis: the meaning shortened at a word, the unit kept
    let tail = if u.is_empty() { String::new() } else { format!(" ({})", pu(&u)) };
    let room = 34usize.saturating_sub(sym.chars().count() + 2 + tail.chars().count());
    let mut short = String::new();
    for w in m.split_whitespace() {
        if short.chars().count() + w.chars().count() + usize::from(!short.is_empty()) > room {
            break;
        }
        if !short.is_empty() {
            short.push(' ');
        }
        short.push_str(w);
    }
    while let Some(i) = short.rfind(' ') {
        if ["of", "the", "a", "an", "at", "in", "to", "and", "for", "on", "by", "from", "with"].contains(&&short[i + 1..]) {
            short.truncate(i);
        } else {
            break;
        }
    }
    if short.is_empty() { format!("{sym}{tail}") } else { format!("{sym}, {short}{tail}") }
}

/// A number input as read (SI), for a formula or a procedure (a procedure's inputs are not in the answer's SI list).
pub fn inp(s: &Solved, sym: &str) -> Option<f64> {
    s.inp(sym).or_else(|| match s.val(sym)? {
        ax_engine::Val::Num(x) => Some(x),
        ax_engine::Val::Int(i) => Some(i as f64),
        _ => None,
    }).filter(|x| x.is_finite())
}

/// A quantity of the request, input or output.
pub fn get(s: &Solved, sym: &str) -> Option<f64> {
    inp(s, sym).or_else(|| s.out(sym))
}

/// A number output of another answer (a formula's value, or a procedure's named output).
pub fn val(a: &Answer, sym: &str) -> Option<f64> {
    k::num_of(a, sym)
}

/// The value of `y` when the inputs `set` (SI) replace the request's own; None when refused.
pub fn at(s: &Solved, set: &[(&str, f64)], y: &str) -> Option<f64> {
    s.with(set).and_then(|a| k::num_of(&a, y))
}

/// Centred labels that would cross the canvas's right edge (an axis's last tick at the frame's corner) set flush
/// right instead.
pub fn fit_right(body: &str) -> String {
    let mut out = String::with_capacity(body.len());
    let mut rest = body;
    while let Some(i) = rest.find("<text ") {
        out.push_str(&rest[..i]);
        let tail = &rest[i..];
        let Some(j) = tail.find("</text>") else { out.push_str(tail); return out };
        let el = &tail[..j + 7];
        let x: Option<f64> = el.split(" x=\"").nth(1).and_then(|v| v.split('"').next()).and_then(|v| v.parse().ok());
        let content = el.split('>').nth(1).unwrap_or("").split('<').next().unwrap_or("");
        let w = content.chars().count() as f64 * 7.6;
        match x {
            Some(x) if el.contains("text-anchor=\"middle\"") && x + w / 2.0 > 516.0 && !el.contains("transform=") => {
                out.push_str(&el.replace("text-anchor=\"middle\"", "text-anchor=\"end\"")
                                .replacen(&format!(" x=\"{}", el.split(" x=\"").nth(1).unwrap().split('"').next().unwrap()),
                                          " x=\"516", 1));
            }
            _ => out.push_str(el),
        }
        rest = &tail[j + 7..];
    }
    out.push_str(rest);
    out
}

pub fn esc(t: &str) -> String {
    t.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// Nice round levels between lo and hi (about n of them, strictly inside).
pub fn nice_levels(lo: f64, hi: f64, n: usize) -> Vec<f64> {
    let span = hi - lo;
    if !(span > 0.0) || !span.is_finite() {
        return vec![];
    }
    let raw = span / n as f64;
    let mag = 10f64.powf(raw.log10().floor());
    let norm = raw / mag;
    let step = mag * if norm < 1.5 { 1.0 } else if norm < 3.0 { 2.0 } else if norm < 7.0 { 5.0 } else { 10.0 };
    let mut v = vec![];
    let mut t = (lo / step).floor() * step + step;
    while t < hi - step * 1e-9 && v.len() < 12 {
        if t > lo + step * 1e-9 {
            v.push(if t.abs() < step * 1e-9 { 0.0 } else { t });
        }
        t += step;
    }
    v
}

// ------------------------------------------------------------------------------------------------ a value over two inputs
const X0: f64 = 96.0;
const X1: f64 = 504.0;
const Y0: f64 = 44.0;
const Y1: f64 = 278.0;

/// `z` over the inputs `x` and `y` (each from half to one and a half times the request's value), every cell one solve
/// of the model, shaded by its value, iso-lines at round values, the request marked. Kind "field".
pub fn field_map(s: &Solved, x: &str, y: &str, z: &str) -> Option<Figure> {
    field_map_k(s, "field", x, y, z, (0.5, 1.5), (0.5, 1.5))
}

/// `field_map` with its own kind and ranges (factors of the request's values).
pub fn field_map_k(s: &Solved, kind: &str, x: &str, y: &str, z: &str, fx: (f64, f64), fy: (f64, f64)) -> Option<Figure> {
    field_map_n(s, kind, x, y, z, fx, fy, 21)
}

/// `field_map_k` with at most `nmax` cells across (a slow calculation takes a coarser grid: 11 gives 11 x 9).
#[allow(clippy::too_many_arguments)]
pub fn field_map_n(s: &Solved, kind: &str, x: &str, y: &str, z: &str, fx: (f64, f64), fy: (f64, f64), nmax: usize)
                   -> Option<Figure> {
    let (x0, y0, z0) = (inp(s, x)?, inp(s, y)?, get(s, z)?);
    if x0 == 0.0 || y0 == 0.0 {
        return None;
    }
    let (nx, ny) = if nmax >= 21 && s.left() >= 21 * 15 { (21, 15) }
                   else if nmax >= 15 && s.left() >= 15 * 11 { (15, 11) }
                   else if s.left() >= 11 * 9 { (11, 9) } else { return None };
    let (xa, xb) = ((x0 * fx.0).min(x0 * fx.1), (x0 * fx.0).max(x0 * fx.1));
    let (ya, yb) = ((y0 * fy.0).min(y0 * fy.1), (y0 * fy.0).max(y0 * fy.1));
    let xs = k::span(xa, xb, nx, false);
    let ys = k::span(ya, yb, ny, false);
    let mut vals: Vec<Vec<Option<f64>>> = vec![vec![None; ny]; nx];
    let mut n_ok = 0;
    for (i, xv) in xs.iter().enumerate() {
        for (j, yv) in ys.iter().enumerate() {
            if let Some(v) = at(s, &[(x, *xv), (y, *yv)], z) {
                vals[i][j] = Some(v);
                n_ok += 1;
            }
        }
    }
    if n_ok < nx * ny / 2 {
        return None;
    }
    let all: Vec<f64> = vals.iter().flatten().flatten().copied().collect();
    let lo = all.iter().cloned().fold(f64::INFINITY, f64::min);
    let hi = all.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let zu = unit(s, z);
    let mut body = k::field(&xs, &ys, &vals, (x0, y0), &axis(s, x), &axis(s, y), &eqs(s, z, z0), (false, false));
    if body.is_empty() {
        return None;
    }
    // iso-lines (marching squares on the solved grid)
    let px = |v: f64| X0 + (v - xs[0]) / (xs[nx - 1] - xs[0]) * (X1 - X0);
    let py = |v: f64| Y1 - (v - ys[0]) / (ys[ny - 1] - ys[0]) * (Y1 - Y0);
    let levels = nice_levels(lo, hi, 5);
    let mut iso = String::new();
    for lv0 in &levels {
        // a level a grid value meets exactly would leave gaps: crossed a hair above it
        let lv = &(lv0 + (hi - lo) * 1e-9);
        let mut segs: Vec<((f64, f64), (f64, f64))> = vec![];
        for i in 0..nx - 1 {
            for j in 0..ny - 1 {
                let c = [(i, j), (i + 1, j), (i + 1, j + 1), (i, j + 1)];
                let v: Vec<Option<f64>> = c.iter().map(|(a, b)| vals[*a][*b]).collect();
                if v.iter().any(|w| w.is_none()) {
                    continue;
                }
                let v: Vec<f64> = v.into_iter().flatten().collect();
                let mut pts = vec![];
                for e in 0..4 {
                    let (a, b) = (e, (e + 1) % 4);
                    let (va, vb) = (v[a], v[b]);
                    if (va - lv) * (vb - lv) < 0.0 {
                        let t = (lv - va) / (vb - va);
                        let (pa, pb) = ((xs[c[a].0], ys[c[a].1]), (xs[c[b].0], ys[c[b].1]));
                        pts.push((px(pa.0 + t * (pb.0 - pa.0)), py(pa.1 + t * (pb.1 - pa.1))));
                    }
                }
                if pts.len() == 2 {
                    segs.push((pts[0], pts[1]));
                } else if pts.len() == 4 {
                    segs.push((pts[0], pts[1]));
                    segs.push((pts[2], pts[3]));
                }
            }
        }
        if segs.is_empty() {
            continue;
        }
        let mut d = String::new();
        for (a, b) in &segs {
            d.push_str(&format!("M{:.1},{:.1} L{:.1},{:.1} ", a.0, a.1, b.0, b.1));
        }
        iso.push_str(&format!("<path class=\"axp-c4\" stroke-width=\"1.2\" d=\"{}\"/>", d.trim_end()));
        let m = segs[segs.len() / 2];
        let (lx, ly) = ((m.0 .0 + m.1 .0) / 2.0, (m.0 .1 + m.1 .1) / 2.0);
        iso.push_str(&format!("<text class=\"axp-t\" font-size=\"11\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                              lx.clamp(X0 + 14.0, X1 - 14.0), (ly - 3.0).clamp(Y0 + 12.0, Y1 - 3.0), esc(&num(*lv0))));
    }
    body.push_str(&iso);
    // the request's value beside its mark
    let (mx, my) = (px(x0), py(y0));
    let lab = qv(z0, &zu);
    let w = k::text_w(&lab, true);
    let tx = if mx + 12.0 + w < X1 { mx + 12.0 } else { mx - 12.0 - w };
    body.push_str(&format!("<text class=\"axp-mt\" x=\"{tx:.1}\" y=\"{:.1}\">{}</text>", (my - 8.0).max(Y0 + 14.0), esc(&lab)));
    let body = fit_right(&body);
    let (zm, xm, ym) = (meaning(s, z), meaning(s, x), meaning(s, y));
    let title = format!("{}: {zm} over {xm} and {ym}", name(s));
    let cap = format!("{z} ({zm}) over {x} ({xm}) from {} to {} and {y} ({ym}) from {} to {}: each of the {nx} × {ny} \
                       cells is one solve of the model, shaded by its value (light {} to dark {}); iso-lines at {} = {}. \
                       Marked: the given point, {}, {}, {}.",
                      qv(xs[0], &unit(s, x)), qv(xs[nx - 1], &unit(s, x)), qv(ys[0], &unit(s, y)), qv(ys[ny - 1], &unit(s, y)),
                      qv(lo, &zu), qv(hi, &zu), z, levels.iter().map(|l| num(*l)).collect::<Vec<_>>().join(", "),
                      eqs(s, x, x0), eqs(s, y, y0), eqs(s, z, z0));
    Some(s.figure(kind, &title, &cap, &body, 340.0))
}

// ------------------------------------------------------------------------------------------------ curve families
/// `y` against `x` over `xs`, one curve for each value of `param` (SI), the request's point marked. Curves with
/// fewer than two points are left out; None when no curve remains.
#[allow(clippy::too_many_arguments)]
pub fn family_fig(s: &Solved, kind: &str, x: &str, xs: &[f64], y: &str, param: &str, values: &[f64], log_x: bool,
                  title: &str, note: &str) -> Option<Figure> {
    let (x0, y0) = (inp(s, x)?, get(s, y)?);
    let pu_ = unit(s, param);
    let mut series = vec![];
    for v in values {
        let pts = s.curve(x, xs, y, &[(param, *v)]);
        if pts.len() >= 2 {
            series.push((format!("{param} = {}", qv(*v, &pu_)), pts));
        }
    }
    if series.is_empty() {
        return None;
    }
    let body = fit_right(&k::lines(&series, &axis(s, x), &axis(s, y), &eqs(s, y, y0), Some((x0, y0)), &qv(y0, &unit(s, y)), log_x));
    let cap = format!("{y} ({}) against {x} ({}) from {} to {}{}, one curve for each value of {param} ({}): {}; {} points \
                       per curve, each a solve of the model. {note}Marked: the given point, {}, {}.",
                      meaning(s, y), meaning(s, x), qv(xs[0], &unit(s, x)), qv(xs[xs.len() - 1], &unit(s, x)),
                      if log_x { " on a logarithmic axis" } else { "" }, meaning(s, param),
                      values.iter().map(|v| qv(*v, &pu_)).collect::<Vec<_>>().join(", "), xs.len(), eqs(s, x, x0), eqs(s, y, y0));
    Some(s.chart(kind, title, &cap, &body))
}

/// The standard family: `y` against `x` from lo·x to hi·x (41 points, logarithmic when `log`), one curve each for
/// `param` at its value, half and twice it.
pub fn family_std(s: &Solved, kind: &str, x: &str, y: &str, param: &str, lo: f64, hi: f64, log: bool) -> Option<Figure> {
    let (x0, p0) = (inp(s, x)?, inp(s, param)?);
    let xs = k::span(x0 * lo, x0 * hi, if log { 41 } else { 31 }, log);
    let title = format!("{}: {} against {}", name(s), meaning(s, y), meaning(s, x));
    family_fig(s, kind, x, &xs, y, param, &[p0, p0 / 2.0, p0 * 2.0], log, &title, "")
}

// ------------------------------------------------------------------------------------------------ bars over cases
/// One bar per case: `y` solved with the case's inputs set (SI), the request's own value as the last bar
/// (highlighted). `log`: a logarithmic value axis is chosen by the chart itself when the values span decades.
pub fn case_bars(s: &Solved, kind: &str, y: &str, cases: &[(&str, Vec<(&str, f64)>)], with_mine: bool, ylab: &str,
                 title: &str, intro: &str) -> Option<Figure> {
    let mut labels = vec![];
    let mut vals = vec![];
    let uy = unit(s, y);
    for (lab, set) in cases {
        if let Some(v) = at(s, set, y) {
            labels.push(lab.to_string());
            vals.push(v);
        }
    }
    let mut hl = vec![];
    if with_mine {
        labels.push("given".to_string());
        vals.push(get(s, y)?);
        hl.push(vals.len() - 1);
    }
    if vals.len() < 2 {
        return None;
    }
    let body = k::bars(&labels, &vals, &hl, "", ylab, &eqs(s, y, get(s, y)?));
    let marks: Vec<String> = labels.iter().zip(&vals).map(|(l, v)| format!("{l} {}", qv(*v, &uy))).collect();
    let cap = format!("{intro} Each bar is a solve of the model{}. Marked: {}.",
                      if with_mine { "; the last bar is the given inputs" } else { "" }, marks.join(", "));
    Some(s.chart(kind, title, &cap, &body))
}

// ------------------------------------------------------------------------------------------------ drawing helpers
/// An arrow from a to b in a colour class (axp-c: accent, axp-c2: green, axp-c3: violet, axp-k: grey), with a head.
pub fn varrow(c: &mut Canvas, a: (f64, f64), b: (f64, f64), class: &str) {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let d = (dx * dx + dy * dy).sqrt();
    if !(d > 1.0) {
        return;
    }
    let (ux, uy) = (dx / d, dy / d);
    let (nx, ny) = (-uy, ux);
    let e = (b.0 - ux * 9.0, b.1 - uy * 9.0);
    let fill = match class {
        "axp-c" => "axp-d",
        "axp-c2" => "axp-d2",
        "axp-c3" => "axp-d3",
        "axp-c4" => "axp-d4",
        _ => "axp-kd",
    };
    c.raw(&format!("<path class=\"{class}\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", a.0, a.1, e.0, e.1));
    c.raw(&format!("<path class=\"{fill}\" d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>", b.0, b.1,
                   e.0 + nx * 5.0, e.1 + ny * 5.0, e.0 - nx * 5.0, e.1 - ny * 5.0));
}

/// A filled rectangle (a body, a block), grey.
pub fn rect(c: &mut Canvas, x: f64, y: f64, w: f64, h: f64) {
    c.raw(&format!("<rect class=\"axp-ka\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{w:.1}\" height=\"{h:.1}\"/>\
                    <rect class=\"axp-k\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{w:.1}\" height=\"{h:.1}\"/>"));
}

/// A filled circle in a class (axp-ka grey fill with outline; axp-d accent dot).
pub fn disc(c: &mut Canvas, p: (f64, f64), r: f64, filled: bool) {
    if filled {
        c.raw(&format!("<circle class=\"axp-ka\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"{r:.1}\"/>", p.0, p.1));
    }
    c.raw(&format!("<circle class=\"axp-k\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"{r:.1}\"/>", p.0, p.1));
}

/// A circle or ellipse outline in a line class.
pub fn ellipse(c: &mut Canvas, p: (f64, f64), rx: f64, ry: f64, class: &str) {
    c.raw(&format!("<ellipse class=\"{class}\" cx=\"{:.1}\" cy=\"{:.1}\" rx=\"{rx:.1}\" ry=\"{ry:.1}\"/>", p.0, p.1));
}

/// A hatched ground line from x0 to x1 at y.
pub fn ground_line(c: &mut Canvas, x0: f64, x1: f64, y: f64) {
    c.line((x0, y), (x1, y));
    let mut x = x0 + 4.0;
    let mut d = String::new();
    while x < x1 {
        d.push_str(&format!("M{:.1},{:.1} L{:.1},{:.1} ", x, y, x - 7.0, y + 8.0));
        x += 9.0;
    }
    c.raw(&format!("<path class=\"axp-c4\" stroke-width=\"1\" d=\"{}\"/>", d.trim_end()));
}

/// A coil spring (zig-zag) from a to b with `turns` teeth, half-width w.
pub fn spring(c: &mut Canvas, a: (f64, f64), b: (f64, f64), turns: usize, w: f64) {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let d = (dx * dx + dy * dy).sqrt().max(1e-9);
    let (ux, uy) = (dx / d, dy / d);
    let (nx, ny) = (-uy, ux);
    let lead = (d * 0.08).min(14.0);
    let body = d - 2.0 * lead;
    let mut pts = vec![a, (a.0 + ux * lead, a.1 + uy * lead)];
    let n = turns.max(2) * 2;
    for i in 0..n {
        let t = lead + body * (i as f64 + 0.5) / n as f64;
        let side = if i % 2 == 0 { w } else { -w };
        pts.push((a.0 + ux * t + nx * side, a.1 + uy * t + ny * side));
    }
    pts.push((b.0 - ux * lead, b.1 - uy * lead));
    pts.push(b);
    c.wire(&pts);
}

/// A polyline in a colour class (a path of points), optionally dashed.
pub fn poly(c: &mut Canvas, pts: &[(f64, f64)], class: &str, dashed: bool) {
    if pts.len() < 2 {
        return;
    }
    let mut d = String::new();
    for (i, p) in pts.iter().enumerate() {
        d.push_str(&format!("{}{:.1},{:.1} ", if i == 0 { "M" } else { "L" }, p.0, p.1));
    }
    c.raw(&format!("<path class=\"{class}\"{} d=\"{}\"/>", if dashed { " stroke-dasharray=\"5 4\"" } else { "" },
                   d.trim_end()));
}

/// A filled polygon in a band class (axp-s, axp-s2, ...: translucent accent fills).
pub fn area(c: &mut Canvas, pts: &[(f64, f64)], class: &str) {
    if pts.len() < 3 {
        return;
    }
    let mut d = String::new();
    for (i, p) in pts.iter().enumerate() {
        d.push_str(&format!("{}{:.1},{:.1} ", if i == 0 { "M" } else { "L" }, p.0, p.1));
    }
    c.raw(&format!("<path class=\"{class}\" d=\"{}Z\"/>", d));
}

/// An arc of a circle round c from angle a0 to a1 (radians, anticlockwise on the page), in a line class.
pub fn arc(c: &mut Canvas, ctr: (f64, f64), r: f64, a0: f64, a1: f64, class: &str) {
    let n = 24;
    let pts: Vec<(f64, f64)> = (0..=n).map(|i| {
        let t = a0 + (a1 - a0) * i as f64 / n as f64;
        (ctr.0 + r * t.cos(), ctr.1 - r * t.sin())
    }).collect();
    poly(c, &pts, class, false);
}

/// A linear map of [lo, hi] onto [a, b] (pixels).
pub fn lin(lo: f64, hi: f64, a: f64, b: f64) -> impl Fn(f64) -> f64 {
    move |v: f64| if hi > lo { a + (v - lo) / (hi - lo) * (b - a) } else { (a + b) / 2.0 }
}

/// Input as a float (for solve_other).
pub fn fl(x: f64) -> Input {
    Input::Float(x)
}
