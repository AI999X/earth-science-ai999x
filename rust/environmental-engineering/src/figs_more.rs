//! Figure helpers for the engineering and earth models (Environmental, Civil, Aerospace Engineering, Earth Science),
//! beside the physics family's figs_common: a value over two inputs on stated ranges (logarithmic axes and shading
//! when asked), one output against one input, given data with what the model fitted, a verdict along one input, the
//! worked request as a table, and a plot frame of its own for drawings over measured axes. Everything drawn comes from
//! solves of the model itself; a figure that cannot be drawn is None.
//!
//! Included by path next to figs_common (`#[path = "../../environmental-engineering/src/figs_more.rs"] mod figs_more;`).

#![allow(dead_code)]

use crate::figs_common::{self as c, axis, eqs, esc, get, inp, meaning, name, qv, unit};
use ax_engine::figkit::{self as k, num, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

pub const X0: f64 = 96.0;
pub const X1: f64 = 504.0;
pub const Y0: f64 = 44.0;
pub const Y1: f64 = 278.0;

/// A range of an input: from, to, logarithmic.
pub type Rg = (f64, f64, bool);

/// From a·x to b·x, linear.
pub fn fx(x: f64, a: f64, b: f64) -> Rg {
    let (l, h) = ((x * a).min(x * b), (x * a).max(x * b));
    (l, h, false)
}

/// A tenth to ten times x, logarithmic (x > 0).
pub fn dec(x: f64) -> Rg {
    (x / 10.0, x * 10.0, true)
}

fn px_of(r: Rg, a: f64, b: f64) -> impl Fn(f64) -> f64 {
    move |v: f64| {
        let f = if r.2 { (v.log10() - r.0.log10()) / (r.1.log10() - r.0.log10()) } else { (v - r.0) / (r.1 - r.0) };
        a + f * (b - a)
    }
}

/// Nice tick values over a range (powers of ten on a logarithmic one).
pub fn ticks(r: Rg, n: usize) -> Vec<f64> {
    if r.2 && r.0 > 0.0 {
        let (e0, e1) = (r.0.log10().ceil() as i32, r.1.log10().floor() as i32);
        let step = (((e1 - e0 + 1).max(1) as f64) / 6.0).ceil().max(1.0) as usize;
        let v: Vec<f64> = (e0..=e1).step_by(step).map(|e| 10f64.powi(e)).collect();
        if !v.is_empty() {
            return v;
        }
    }
    let mut v = vec![r.0];
    v.extend(c::nice_levels(r.0, r.1, n));
    v.push(r.1);
    let lv = c::nice_levels(r.0, r.1, n);
    if lv.len() >= 2 { lv } else { v }
}

// ------------------------------------------------------------------------------------------------ a value over two inputs
/// `z` over the inputs `x` on `rx` and `y` on `ry`, every cell one solve of the model, shaded by its value
/// (logarithmically when the values span more than two decades), iso-lines at round values, the request marked.
pub fn field_xy(s: &Solved, kind: &str, x: &str, rx: Rg, y: &str, ry: Rg, z: &str) -> Option<Figure> {
    let (x0, y0, z0) = (inp(s, x)?, inp(s, y)?, get(s, z)?);
    if !(rx.1 > rx.0) || !(ry.1 > ry.0) || (rx.2 && rx.0 <= 0.0) || (ry.2 && ry.0 <= 0.0) {
        return None;
    }
    let (nx, ny) = if s.left() >= 21 * 15 { (21, 15) } else if s.left() >= 11 * 9 { (11, 9) } else { return None };
    let xs = k::span(rx.0, rx.1, nx, rx.2);
    let ys = k::span(ry.0, ry.1, ny, ry.2);
    let mut vals: Vec<Vec<Option<f64>>> = vec![vec![None; ny]; nx];
    let mut n_ok = 0;
    for (i, xv) in xs.iter().enumerate() {
        for (j, yv) in ys.iter().enumerate() {
            if let Some(v) = c::at(s, &[(x, *xv), (y, *yv)], z) {
                vals[i][j] = Some(v);
                n_ok += 1;
            }
        }
    }
    if n_ok < nx * ny / 3 {
        return None;
    }
    let all: Vec<f64> = vals.iter().flatten().flatten().copied().collect();
    let lo = all.iter().cloned().fold(f64::INFINITY, f64::min);
    let hi = all.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let logz = lo > 0.0 && hi / lo > 100.0;
    let shade: Vec<Vec<Option<f64>>> = if logz {
        vals.iter().map(|col| col.iter().map(|v| v.map(|w| w.log10())).collect()).collect()
    } else {
        vals.clone()
    };
    let zu = unit(s, z);
    let mut body = k::field(&xs, &ys, &shade, (x0, y0), &axis(s, x), &axis(s, y), &eqs(s, z, z0), (rx.2, ry.2));
    if body.is_empty() {
        return None;
    }
    if logz {
        // the scale states the values, not their logarithms
        let (slo, shi) = (lo.log10(), hi.log10());
        body = body.replace(&format!(">light {} to dark {}<", esc(&num(k::snap(slo, shi))), esc(&num(shi))),
                            &format!(">light {} to dark {} (log)<", esc(&num(lo)), esc(&num(hi))));
    }
    let px = px_of(rx, X0, X1);
    let py = px_of(ry, Y1, Y0);
    // iso-lines (marching squares on the solved grid), at powers of ten when shaded logarithmically
    let levels: Vec<f64> = if logz {
        let (a, b) = (lo.log10().ceil() as i32, hi.log10().floor() as i32);
        (a..=b).map(|e| e as f64).filter(|e| *e > lo.log10() && *e < hi.log10()).take(8).collect()
    } else {
        c::nice_levels(lo, hi, 5)
    };
    let (slo, shi) = if logz { (lo.log10(), hi.log10()) } else { (lo, hi) };
    let mut iso = String::new();
    for lv0 in &levels {
        let lv = lv0 + (shi - slo) * 1e-9;
        let mut segs: Vec<((f64, f64), (f64, f64))> = vec![];
        for i in 0..nx - 1 {
            for j in 0..ny - 1 {
                let cs = [(i, j), (i + 1, j), (i + 1, j + 1), (i, j + 1)];
                let v: Vec<Option<f64>> = cs.iter().map(|(a, b)| shade[*a][*b]).collect();
                if v.iter().any(|w| w.is_none()) {
                    continue;
                }
                let v: Vec<f64> = v.into_iter().flatten().collect();
                let mut pts = vec![];
                for e in 0..4 {
                    let (a, b) = (e, (e + 1) % 4);
                    if (v[a] - lv) * (v[b] - lv) < 0.0 {
                        let t = (lv - v[a]) / (v[b] - v[a]);
                        let (pa, pb) = ((px(xs[cs[a].0]), py(ys[cs[a].1])), (px(xs[cs[b].0]), py(ys[cs[b].1])));
                        pts.push((pa.0 + t * (pb.0 - pa.0), pa.1 + t * (pb.1 - pa.1)));
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
        // the label on a segment away from the request's mark and its value (beside it, to the right)
        let (mkx, mky) = (px(x0), py(y0));
        let mid = |g: &((f64, f64), (f64, f64))| ((g.0 .0 + g.1 .0) / 2.0, (g.0 .1 + g.1 .1) / 2.0);
        let clear = |q: (f64, f64)| !((q.0 - mkx).abs() < 30.0 && (q.1 - mky).abs() < 22.0)
            && !(q.0 > mkx && q.0 < mkx + 130.0 && (q.1 - (mky - 10.0)).abs() < 16.0);
        let order = [segs.len() / 2, segs.len() / 4, segs.len() * 3 / 4, segs.len() / 8, segs.len() * 7 / 8];
        let pick = order.iter().map(|i| mid(&segs[*i])).find(|q| clear(*q)).unwrap_or_else(|| mid(&segs[segs.len() / 2]));
        let (lx, ly) = pick;
        let lab = if logz { num(10f64.powf(*lv0)) } else { num(*lv0) };
        iso.push_str(&format!("<text class=\"axp-t\" font-size=\"11\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                              lx.clamp(X0 + 18.0, X1 - 18.0), (ly - 3.0).clamp(Y0 + 12.0, Y1 - 3.0), esc(&lab)));
    }
    body.push_str(&iso);
    let (mx, my) = (px(x0), py(y0));
    let lab = qv(z0, &zu);
    let w = k::text_w(&lab, true) * 1.15;
    let tx = if mx + 12.0 + w < X1 { mx + 12.0 } else { (mx - 12.0 - w).max(X0 + 2.0) };
    body.push_str(&format!("<text class=\"axp-mt\" x=\"{tx:.1}\" y=\"{:.1}\">{}</text>", (my - 8.0).max(Y0 + 14.0), esc(&lab)));
    let body = c::fit_right(&body);
    let (zm, xm, ym) = (meaning(s, z), meaning(s, x), meaning(s, y));
    let title = format!("{}: {zm} over {xm} and {ym}", name(s));
    let logs = match (rx.2, ry.2) {
        (true, true) => " (logarithmic axes)",
        (true, false) => " (logarithmic horizontal axis)",
        (false, true) => " (logarithmic vertical axis)",
        _ => "",
    };
    let cap = format!("{z} ({zm}) over {x} ({xm}) from {} to {} and {y} ({ym}) from {} to {}{logs}: each of the {nx} × {ny} \
                       cells is one solve of the model, shaded by its value (light {} to dark {}{}){}; iso-lines at {z} = {}. \
                       Marked: the given point, {}, {}, {}.",
                      qv(xs[0], &unit(s, x)), qv(xs[nx - 1], &unit(s, x)), qv(ys[0], &unit(s, y)), qv(ys[ny - 1], &unit(s, y)),
                      qv(lo, &zu), qv(hi, &zu), if logz { ", logarithmic shading" } else { "" },
                      if n_ok < nx * ny { "; blank cells: inputs the model refuses" } else { "" },
                      levels.iter().map(|l| if logz { num(10f64.powf(*l)) } else { num(*l) }).collect::<Vec<_>>().join(", "),
                      eqs(s, x, x0), eqs(s, y, y0), eqs(s, z, z0));
    Some(s.figure(kind, &title, &cap, &body, 340.0))
}

/// `field_xy` over half to one and a half times each input.
pub fn field_half(s: &Solved, kind: &str, x: &str, y: &str, z: &str) -> Option<Figure> {
    let (x0, y0) = (inp(s, x)?, inp(s, y)?);
    field_xy(s, kind, x, fx(x0, 0.5, 1.5), y, fx(y0, 0.5, 1.5), z)
}

// ------------------------------------------------------------------------------------------------ one curve
/// `y` against the input `x` over `r` (31 points, each a solve of the model with the other inputs as given), the
/// request marked. `title`: the published heading's subject.
pub fn curve1(s: &Solved, kind: &str, x: &str, r: Rg, y: &str) -> Option<Figure> {
    let (x0, y0) = (inp(s, x)?, get(s, y)?);
    let xs = k::span(r.0, r.1, 31, r.2);
    let pts = s.curve(x, &xs, y, &[]);
    if pts.len() < 3 {
        return None;
    }
    let uy = unit(s, y);
    let body = c::fit_right(&k::lines(&[(String::new(), pts.clone())], &axis(s, x), &axis(s, y), &eqs(s, y, y0),
                                      Some((x0, y0)), &qv(y0, &uy), r.2));
    let title = format!("{y} against {x}: {}", name(s));
    let cap = format!("{y} ({}) against {x} ({}) from {} to {}{}, the other inputs as given: {} points, each a solve of the \
                       model{}. Marked: the given point, {}, {}.",
                      meaning(s, y), meaning(s, x), qv(xs[0], &unit(s, x)), qv(xs[30], &unit(s, x)),
                      if r.2 { " on a logarithmic axis" } else { "" }, pts.len(),
                      if pts.len() < 31 { " (points the model refuses are left out)" } else { "" }, eqs(s, x, x0), eqs(s, y, y0));
    Some(s.chart(kind, &title, &cap, &body))
}

// ------------------------------------------------------------------------------------------------ data and fits
/// Points drawn as dots only (no joining line): each point followed by a break.
pub fn dots(p: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut v = vec![];
    for q in p {
        v.push(*q);
        v.push((f64::NAN, f64::NAN));
    }
    v
}

/// A list input as numbers (as read), or a list output.
pub fn list(s: &Solved, sym: &str) -> Option<Vec<f64>> {
    s.vec(sym).or_else(|| s.out_list(sym)).filter(|v| !v.is_empty())
}

/// Text of numbers: "1, 2.5, 3".
pub fn nums(v: &[f64]) -> String {
    v.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", ")
}

/// A chart of series in the generic frame (dots for series of at most 12 points); None when no series has two
/// points.
#[allow(clippy::too_many_arguments)]
pub fn chart(s: &Solved, kind: &str, title: &str, cap: &str, series: Vec<(String, Vec<(f64, f64)>)>, xlab: &str,
             ylab: &str, header: &str, marker: Option<(f64, f64)>, mlabel: &str, log_x: bool) -> Option<Figure> {
    let series: Vec<(String, Vec<(f64, f64)>)> =
        series.into_iter().filter(|x| x.1.iter().filter(|p| p.0.is_finite() && p.1.is_finite()).count() >= 1).collect();
    let n: usize = series.iter().map(|x| x.1.iter().filter(|p| p.0.is_finite()).count()).sum();
    if series.is_empty() || n < 2 {
        return None;
    }
    let body = c::fit_right(&k::lines(&series, xlab, ylab, header, marker, mlabel, log_x));
    if body.is_empty() {
        return None;
    }
    Some(s.chart(kind, title, cap, &body))
}

// ------------------------------------------------------------------------------------------------ a verdict along one input
/// The verdict `v` (a text output) along the input `x` over `r` (81 solves), each stretch shaded by the verdict
/// returned there, the changes marked, the request's point drawn. Kind usually "decision".
pub fn bands_1d(s: &Solved, kind: &str, x: &str, r: Rg, v: &str) -> Option<Figure> {
    let x0 = inp(s, x)?;
    let mine = s.out_text(v)?;
    let xs = k::span(r.0, r.1, 81, r.2);
    let mut got: Vec<(f64, Option<String>)> = vec![];
    for xv in &xs {
        let t = s.with(&[(x, *xv)]).and_then(|a| match a.values.iter().find(|e| e.0 == v).map(|e| e.1.clone()) {
            Some(serde_json::Value::String(t)) => Some(t),
            Some(serde_json::Value::Bool(b)) => Some(if b { "yes".into() } else { "no".into() }),
            _ => None,
        });
        got.push((*xv, t));
    }
    let mut names: Vec<String> = vec![];
    for (_, t) in &got {
        if let Some(t) = t {
            if !names.contains(t) {
                names.push(t.clone());
            }
        }
    }
    if names.is_empty() {
        return None;
    }
    let px = px_of(r, X0, X1);
    let classes = ["axp-b", "axp-b2", "axp-b3", "axp-b4", "axp-s", "axp-s2", "axp-s3", "axp-s4"];
    let mut out = String::new();
    let (by0, by1) = (Y0 + 40.0, Y1 - 40.0);
    let mut changes = vec![];
    let mut i = 0;
    while i < got.len() {
        let Some(t) = got[i].1.clone() else { i += 1; continue };
        let mut j = i;
        while j + 1 < got.len() && got[j + 1].1.as_ref() == Some(&t) {
            j += 1;
        }
        let ki = names.iter().position(|n| *n == t).unwrap_or(0);
        let left = if i == 0 { got[i].0 } else if r.2 { (got[i - 1].0 * got[i].0).sqrt() } else { (got[i - 1].0 + got[i].0) / 2.0 };
        let right = if j + 1 >= got.len() { got[j].0 } else if r.2 { (got[j].0 * got[j + 1].0).sqrt() } else { (got[j].0 + got[j + 1].0) / 2.0 };
        if j + 1 < got.len() && got[j + 1].1.is_some() {
            changes.push(right);
        }
        let (l, rr) = (px(left).max(X0), px(right).min(X1));
        out.push_str(&format!("<rect class=\"{}\" x=\"{l:.1}\" y=\"{by0:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                              classes[ki % 8], (rr - l).max(0.5), by1 - by0));
        if rr - l > k::text_w(&t, false) * 1.2 + 14.0 {
            out.push_str(&format!("<text class=\"axp-bt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                                  (l + rr) / 2.0, (by0 + by1) / 2.0 + 5.0, esc(&t)));
        }
        i = j + 1;
    }
    out.push_str(&format!("<rect class=\"axp-f\" x=\"{X0}\" y=\"{by0:.1}\" width=\"{}\" height=\"{:.1}\"/>", X1 - X0, by1 - by0));
    for t in ticks(r, 6) {
        let x = px(t);
        if (X0 - 0.5..=X1 + 0.5).contains(&x) {
            out.push_str(&format!("<line class=\"axp-g\" x1=\"{x:.1}\" y1=\"{by1:.1}\" x2=\"{x:.1}\" y2=\"{:.1}\"/>\
                                   <text class=\"axp-t\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                                  by1 + 6.0, by1 + 22.0, esc(&num(t))));
        }
    }
    for ch in &changes {
        let x = px(*ch);
        out.push_str(&format!("<path class=\"axp-c4\" stroke-dasharray=\"4 3\" d=\"M{x:.1},{:.1} L{x:.1},{:.1}\"/>", by0 - 6.0, by1));
    }
    let mx = px(x0);
    out.push_str(&format!("<path class=\"axp-c\" d=\"M{mx:.1},{:.1} L{mx:.1},{:.1}\"/><circle class=\"axp-m\" cx=\"{mx:.1}\" \
                           cy=\"{:.1}\" r=\"6\"/>", by0 - 8.0, by1 + 4.0, by0 - 8.0));
    let lab = format!("{} → {mine}", eqs(s, x, x0));
    let w = k::text_w(&lab, true) * 1.15;
    let tx = if mx + 12.0 + w < X1 { mx + 12.0 } else { (mx - 12.0 - w).max(X0) };
    out.push_str(&format!("<text class=\"axp-mt\" x=\"{tx:.1}\" y=\"{:.1}\">{}</text>", by0 - 12.0, esc(&lab)));
    // legend
    let (mut lx, mut ly) = (X0, Y1 - 4.0);
    for (ki, n) in names.iter().enumerate().take(8) {
        let t = if names.len() > 4 { n.clone() } else { format!("{v}: {n}") };
        let w = k::text_w(&t, false) + 24.0;
        if lx + w > X1 + 10.0 {
            if ly > Y1 + 4.0 {
                break;
            }
            lx = X0;
            ly += 17.0;
        }
        out.push_str(&format!("<rect class=\"{}\" x=\"{lx:.1}\" y=\"{ly:.1}\" width=\"12\" height=\"10\"/>\
                               <text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\">{}</text>", classes[ki % 8], lx + 16.0,
                              ly + 9.0, esc(&t)));
        lx += w;
    }
    out.push_str(&format!("<text class=\"axp-l\" x=\"{:.1}\" y=\"326\" text-anchor=\"middle\">{}</text>", (X0 + X1) / 2.0,
                          esc(&axis(s, x))));
    out.push_str(&format!("<text class=\"axp-mt\" x=\"{X0}\" y=\"20\">{}</text>", esc(&format!("{v} = {mine}"))));
    let title = format!("{}: where the verdict changes ({v})", name(s));
    let cap = format!("{} ({v}) along {x} ({}) from {} to {}{}: {} solves of the model, each stretch shaded by the verdict \
                       returned ({}){}. Marked: the given point, {} gives {mine}.",
                      meaning(s, v), meaning(s, x), qv(xs[0], &unit(s, x)), qv(xs[80], &unit(s, x)),
                      if r.2 { " (logarithmic axis)" } else { "" }, got.iter().filter(|g| g.1.is_some()).count(),
                      names.join(", "),
                      if changes.is_empty() { String::new() } else {
                          format!("; it changes near {x} = {}", changes.iter().map(|c_| qv(*c_, &unit(s, x))).collect::<Vec<_>>().join(", "))
                      }, eqs(s, x, x0));
    Some(s.chart(kind, &title, &cap, &out))
}

// ------------------------------------------------------------------------------------------------ the request as a table
/// The inputs as given and every output returned, one row each. Kind usually "io".
pub fn io_table(s: &Solved, kind: &str) -> Option<Figure> {
    let mut rows: Vec<Vec<String>> = vec![];
    for (sym, v) in s.given {
        let t = match v {
            Input::Text(t) => t.clone(),
            Input::Float(x) => num(*x),
            other => format!("{other:?}"),
        };
        rows.push(vec![sym.clone(), t, "given".to_string()]);
    }
    let n_in = rows.len();
    for (sym, _, _) in &s.a.values {
        if let Some(t) = s.out_text(sym) {
            let u = unit(s, sym);
            let t = match s.out(sym) { Some(x) => qv(x, &u), None => t };
            rows.push(vec![sym.clone(), t, "returned".to_string()]);
        }
    }
    if rows.len() <= n_in {
        return None;
    }
    let (body, h) = k::table(&["quantity".to_string(), "value".to_string(), "".to_string()], &rows, None, &name(s));
    let cap = format!("The request as given and what the model returned: {}.",
                      rows.iter().map(|r| format!("{} = {} ({})", r[0], r[1], r[2])).collect::<Vec<_>>().join(", "));
    Some(s.figure(kind, &format!("{}: inputs and results", name(s)), &cap, &body, h))
}

// ------------------------------------------------------------------------------------------------ a plot frame of its own
/// Measured axes for a drawing that is not a plain curve (a section to scale, a Mohr circle): pixel maps, ticks,
/// labels, in the generic frame's place.
pub struct Plot {
    pub xr: Rg,
    pub yr: Rg,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
}

impl Plot {
    pub fn new(xr: Rg, yr: Rg) -> Plot {
        Plot { xr, yr, x0: X0, x1: X1, y0: Y0, y1: Y1 }
    }
    /// Equal scales on both axes over the given extents (the box is wider than tall: one range grows).
    pub fn equal(xl: f64, xh: f64, yl: f64, yh: f64) -> Plot {
        let (w, h) = (X1 - X0, Y1 - Y0);
        let (dx, dy) = ((xh - xl).max(1e-300), (yh - yl).max(1e-300));
        let scale = (w / dx).min(h / dy);
        let (cx, cy) = ((xl + xh) / 2.0, (yl + yh) / 2.0);
        let (hx, hy) = (w / scale / 2.0, h / scale / 2.0);
        Plot::new((cx - hx, cx + hx, false), (cy - hy, cy + hy, false))
    }
    pub fn px(&self, v: f64) -> f64 {
        px_of(self.xr, self.x0, self.x1)(v)
    }
    pub fn py(&self, v: f64) -> f64 {
        px_of(self.yr, self.y1, self.y0)(v)
    }
    pub fn p(&self, x: f64, y: f64) -> (f64, f64) {
        (self.px(x), self.py(y))
    }
    /// Grid, ticks, frame, axis labels and the heading.
    pub fn axes(&self, out: &mut String, xlab: &str, ylab: &str, header: &str) {
        for t in ticks(self.xr, 6) {
            let x = self.px(t);
            if (self.x0 - 0.5..=self.x1 + 0.5).contains(&x) {
                out.push_str(&format!("<line class=\"axp-g\" x1=\"{x:.1}\" y1=\"{:.1}\" x2=\"{x:.1}\" y2=\"{:.1}\"/>\
                                       <text class=\"axp-t\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                                      self.y0, self.y1, self.y1 + 20.0, esc(&num(k::snap(t, self.xr.1 - self.xr.0)))));
            }
        }
        for t in ticks(self.yr, 5) {
            let y = self.py(t);
            if (self.y0 - 0.5..=self.y1 + 0.5).contains(&y) {
                out.push_str(&format!("<line class=\"axp-g\" x1=\"{:.1}\" y1=\"{y:.1}\" x2=\"{:.1}\" y2=\"{y:.1}\"/>\
                                       <text class=\"axp-t\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>",
                                      self.x0, self.x1, self.x0 - 6.0, y + 5.0, esc(&num(k::snap(t, self.yr.1 - self.yr.0)))));
            }
        }
        out.push_str(&format!("<rect class=\"axp-f\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>", self.x0,
                              self.y0, self.x1 - self.x0, self.y1 - self.y0));
        if !xlab.is_empty() {
            out.push_str(&format!("<text class=\"axp-l\" x=\"{:.1}\" y=\"326\" text-anchor=\"middle\">{}</text>",
                                  (self.x0 + self.x1) / 2.0, esc(xlab)));
        }
        if !ylab.is_empty() {
            out.push_str(&format!("<text class=\"axp-l\" transform=\"translate(16,{:.1}) rotate(-90)\" text-anchor=\"middle\">{}</text>",
                                  (self.y0 + self.y1) / 2.0, esc(ylab)));
        }
        if !header.is_empty() {
            out.push_str(&format!("<text class=\"axp-mt\" x=\"{:.1}\" y=\"20\">{}</text>", self.x0, esc(header)));
        }
    }
    /// The clip to the plot box for what follows (close with `</g>`).
    pub fn clip(&self, out: &mut String) {
        out.push_str(&format!("<clipPath id=\"CLIPID\"><rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/></clipPath>\
                               <g clip-path=\"url(#CLIPID)\">", self.x0, self.y0, self.x1 - self.x0, self.y1 - self.y0));
    }
}
