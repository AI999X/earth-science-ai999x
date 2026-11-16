# Earth Science (earth-science-ai999x)

[![tests](https://github.com/AI999X/earth-science-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/earth-science-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/earth-science-ai999x)](https://pypi.org/project/earth-science-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/earth-science-ai999x)

Earth Science covers atmospheric thermodynamics and meteorology, climate and radiation balance, oceanography and seawater properties, seismology and geophysics, and geology: 87 calculations in 21 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Earth Science desktop app: Atmospheric scale height, with its worked example and live scene](https://www.ai999x.com/?ax_shot=earth-science-ai999x/hero.webp)](https://www.ai999x.com/models/earth-science-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `earth-science` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/earth-science-ai999x/#try) |
| **Python** 3.10+ | `pip install earth-science-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import earth_science_ai999x as earth_science

a = earth_science.ask(
    "Heat index with temperature = 32 °C and relative humidity = 70 %."
)
print(a.outcome, a.values)
# SOLVED {'HI': 313.55927368}

a = earth_science.ask(
    "Brunt vaisala frequency with potential temperature = 300 K and "
    "potential temperature gradient = 3 K/km."
)
print(a.outcome, a.values)
# SOLVED {'N': 0.00990285312423, 'period': 634.482328311}

a = earth_science.ask(
    "Dewpoint from relative humidity with temperature = 30 °C and "
    "relative humidity = 60 %."
)
print(a.outcome, a.values)
# SOLVED {'Td': 294.506499951}

a = earth_science.ask("Scale height with temperature = 15 °C.")
print(a.outcome, a.values)
# SOLVED {'H': 8434.3516415}

a = earth_science.ask(
    "Energy balance model with forcing = 3.7 W/m^2, sensitivity = 0.8 "
    "K/(W/m^2), heat capacity = 4.2e+08 J/(m^2*K) and time = 10 yr."
)
print(a.outcome, a.values)
# SOLVED {'dT': 1.8028327616, 'dT_eq': 2.96, 'tau_years': 10.6471975055}

a = earth_science.ask(
    "Carbon dioxide radiative forcing with co2 = 560 ppm and reference "
    "concentration = 280 ppm."
)
print(a.outcome, a.values)
# SOLVED {'dF': 3.708337416, 'doublings': 1.0}

a = earth_science.ask("Radiocarbon age with fraction modern = 0.5.")
print(a.outcome, a.values)
# SOLVED {'age': 5568.05130144}

a = earth_science.ask(
    "Pratt isostasy with elevation = 3 km, reference density = 2800 "
    "kg/m^3 and compensation depth = 100 km."
)
print(a.outcome, a.values)
# SOLVED {'rho': 2718.44660194, 'drho': 81.5533980583}

a = earth_science.ask(
    "Dipole inclination with magnetic latitude = 45 °."
)
print(a.outcome, a.values)
# SOLVED {'inclination': 63.4349488229, 'F_ratio': 1.58113883008}

a = earth_science.ask(
    "Stream power with discharge = 100 m^3/s and slope = 0.002."
)
print(a.outcome, a.values)
# SOLVED {'Omega': 1961.33}

a = earth_science.ask(
    "Gravity at latitude with latitude = 45 ° and height = 0 m."
)
print(a.outcome, a.values)
# SOLVED {'g': 9.80618987521}

a = earth_science.ask(
    "Geotherm with surface temperature = 10 °C, surface heat flow = 65 "
    "mW/m^2, thermal conductivity = 2.5 W/(m*K), heat production = 1 "
    "µW/m^3 and depth = 10 km."
)
print(a.outcome, a.values)
# SOLVED {'T': 523.15, 'q': 0.055}

a = earth_science.ask(
    "Darcy flow with hydraulic conductivity = 0.0001 m/s, area = 50 "
    "m^2, head difference = 2 m and length = 100 m."
)
print(a.outcome, a.values)
# SOLVED {'Q': 0.0001, 'q': 2e-06, 'gradient': 0.02}

a = earth_science.ask(
    "Ekman depth with eddy viscosity = 0.05 m^2/s and latitude = 45 °."
)
print(a.outcome, a.values)
# SOLVED {'D_E': 97.8285506131, 'delta': 31.1397948112}

a = earth_science.ask(
    "Adiabatic temperature gradient of seawater with salinity = 35, "
    "temperature = 10 °C and pressure = 1000 dbar."
)
print(a.outcome, a.values)
# SOLVED {'gamma': 0.000127387056532, 'gamma_per_km_dbar': 0.127387056532}
```

## Screenshots

[![The Earth Science desktop app: Atmospheric scale height, with its worked example and live scene](https://www.ai999x.com/?ax_shot=earth-science-ai999x/hero-dark.webp)](https://www.ai999x.com/models/earth-science-ai999x/)

[![Density of pure water (SMOW, Bigg 1967): the live scene in the Earth Science desktop app](https://www.ai999x.com/?ax_shot=earth-science-ai999x/see-1.webp)](https://www.ai999x.com/models/earth-science-ai999x/)

*Density of pure water (SMOW, Bigg 1967)*

[![Geostrophic wind from a height gradient: the live scene in the Earth Science desktop app](https://www.ai999x.com/?ax_shot=earth-science-ai999x/see-2.webp)](https://www.ai999x.com/models/earth-science-ai999x/)

*Geostrophic wind from a height gradient*

[![Relative humidity from temperature and dewpoint: the live scene in the Earth Science desktop app](https://www.ai999x.com/?ax_shot=earth-science-ai999x/see-3.webp)](https://www.ai999x.com/models/earth-science-ai999x/)

*Relative humidity from temperature and dewpoint*

Every calculation, with its formula, a worked example and its figures: **[Earth Science on ai999x.com](https://www.ai999x.com/models/earth-science-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/earth-science-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/earth-science-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `earth-science` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/earth-science`) and its data (`rust/models/earth_science/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p earth-science-ai999x
./target/release/earth-science --help
cargo test -p earth-science-ai999x
```
