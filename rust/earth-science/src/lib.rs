//! Earth Science by AI999X, Rust build: the model's data is compiled into the binary (no files to install).
//! The same code serves the command line (src/main.rs) and the browser (WebAssembly: the `ax_*` exports).

use ax_engine::Pack;
use std::sync::OnceLock;

/// The model's data, written by tools/export_rust_pack.py from the Python package (the reference).
pub const PACK_JSON: &str = include_str!("../../models/earth_science/pack.json");

pub mod procedures;
#[path = "../../classical-physics/src/figs_common.rs"]
mod figs_common;
#[path = "../../environmental-engineering/src/figs_more.rs"]
mod figs_more;
pub mod figures;
mod figures_draw;

pub fn pack() -> &'static Pack {
    static P: OnceLock<Pack> = OnceLock::new();
    P.get_or_init(|| {
        let mut p = Pack::from_json(PACK_JSON).expect("the bundled model data is valid");
        p.native = Some(procedures::native);       // the procedures, translated from the package (tools/py2rust.py)
        p.figures = Some(figures::live);           // the model's own live figures (ax_engine::figkit)
        p
    })
}

/// One request as JSON text -> the answer as JSON text (ax_engine::solve_json).
pub fn solve_json(request: &str) -> String {
    ax_engine::solve_json(pack(), request)
}

/// Every capability for a form (ax_engine::catalog_json).
pub fn catalog_json() -> String {
    ax_engine::catalog_json(pack())
}

ax_engine::wasm_exports!(crate::pack);
