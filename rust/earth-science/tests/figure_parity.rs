//! Every capability's own live figures (src/figures*.rs): its worked example gives each published figure kind with
//! the slug "<capability>-<kind>", a changed input changes each drawing, no drawing holds a non-finite number or text
//! outside its canvas, and a refused input draws nothing. The table is tools/figure_test_table.py's.

use ax_engine::figures::figures;
use ax_engine::Input;
use earth_science_ai999x::pack;

mod figure_table;
use figure_table::TABLE;

fn given(ex: &[(&str, &str)]) -> Vec<(String, Input)> {
    ex.iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect()
}

fn check(f: &ax_engine::figures::Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    // FIG_DUMP=<dir>: every own figure of the worked examples written there (for looking at them)
    let dump = std::env::var("FIG_DUMP").ok();
    for (key, kinds, ex) in TABLE {
        let g = given(ex);
        let figs = figures(p, key, &g, None, None);
        if let Some(d) = &dump {
            for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                let _ = std::fs::create_dir_all(d);
                let _ = std::fs::write(format!("{d}/{}.svg", f.slug), &f.svg);
            }
        }
        for kind in *kinds {
            match figs.iter().find(|f| f.kind == *kind) {
                Some(f) => {
                    // the model's own figure, or a generic one of the same kind (a verdict map, bars over a choice)
                    if !f.slug.contains("-live-") {
                        assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                        check(f);
                    }
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "{} missing figures: {missing:?}", missing.len());
}

/// Numbers in a text multiplied by f ("12 V" -> "16.44 V"); None when it holds no non-zero number.
fn scale_text(t: &str, f: f64) -> Option<String> {
    let mut out = String::new();
    let mut cur = String::new();
    let mut changed = false;
    let flush = |cur: &mut String, out: &mut String, changed: &mut bool| {
        if !cur.is_empty() {
            match cur.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    *changed = true;
                }
                _ => out.push_str(cur),
            }
            cur.clear();
        }
    };
    let cs: Vec<char> = t.chars().collect();
    for (i, c) in cs.iter().enumerate() {
        let exp_sign = (*c == '-' || *c == '+') && i > 0 && (cs[i - 1] == 'e' || cs[i - 1] == 'E') && !cur.is_empty();
        if c.is_ascii_digit() || *c == '.' || (*c == '-' && cur.is_empty()) || exp_sign
            || ((*c == 'e' || *c == 'E') && !cur.is_empty() && !cur.contains('e')) {
            cur.push(*c);
        } else {
            flush(&mut cur, &mut out, &mut changed);
            out.push(*c);
        }
    }
    flush(&mut cur, &mut out, &mut changed);
    changed.then_some(out)
}

/// Inputs to change for a capability whose inputs are not plain quantities (text, a choice): (key, input, value).
const OTHER: &[(&str, &str, &str)] = &[];

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds, ex) in TABLE {
        let base_g = given(ex);
        let base = figures(p, key, &base_g, None, None);
        // the model's own kinds (a kind the generic figures give is theirs to follow)
        let kinds: Vec<&str> = kinds.iter().copied()
            .filter(|kd| base.iter().any(|f| f.kind == *kd && !f.slug.contains("-live-"))).collect();
        if kinds.is_empty() {
            continue;
        }
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        for (_, sym, val) in OTHER.iter().filter(|o| o.0 == *key) {
            let mut g = base_g.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            tries.push(g);
        }
        // a whole number one more (a quantum number, a count)
        for i in 0..base_g.len() {
            if let Input::Text(t) = &base_g[i].1 {
                if let Ok(n) = t.trim().parse::<i64>() {
                    let mut g = base_g.clone();
                    g[i].1 = Input::Text((n + 1).to_string());
                    tries.push(g);
                }
            }
        }
        for f in [1.37, 0.73, 1.1, 0.9] {
            for i in 0..base_g.len() {
                if let Input::Text(t) = &base_g[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = base_g.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        let mut why = String::new();
        for g in tries {
            if !ax_engine::solve(p, key, &g, None).solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = kinds.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if !differs && why.is_empty() {
                why = kinds.iter().filter(|kind| {
                    match (base.iter().find(|f| f.kind == **kind), figs.iter().find(|f| f.kind == **kind)) {
                        (Some(a), Some(b)) => a.svg == b.svg,
                        _ => true,
                    }
                }).map(|x| x.to_string()).collect::<Vec<_>>().join("+");
            }
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    check(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(format!("{key} ({why})"));
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn a_refused_input_draws_nothing() {
    // every number of the worked example negated in turn: whenever the model refuses, no figure at all
    let p = pack();
    let mut refused = 0;
    for (key, _, ex) in TABLE {
        let base_g = given(ex);
        for i in 0..base_g.len() {
            let Input::Text(t) = &base_g[i].1 else { continue };
            let Some(t2) = scale_text(t, -1.0) else { continue };
            let mut g = base_g.clone();
            g[i].1 = Input::Text(t2);
            if ax_engine::solve(p, key, &g, None).solved() {
                for f in figures(p, key, &g, None, None).iter().filter(|f| !f.slug.contains("-live-")) {
                    check(f);
                }
                continue;
            }
            refused += 1;
            assert!(figures(p, key, &g, None, None).is_empty(), "{key}: a refused input drew a figure");
        }
    }
    assert!(refused > 0, "no refused input was tried");
}
