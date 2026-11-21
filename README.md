# Earth Science (earth-science-ai999x)

[![tests](https://github.com/AI999X/earth-science-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/earth-science-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/earth-science-ai999x)](https://pypi.org/project/earth-science-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/earth-science-ai999x)

Earth Science covers atmospheric thermodynamics and meteorology, climate and radiation balance, oceanography and seawater properties, seismology and geophysics, and geology: 87 calculations in 21 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Earth Science desktop app: Lifting condensation level (Romps 2017), with its worked example and live scene](https://www.ai999x.com/?ax_shot=earth-science-ai999x/hero.webp)](https://www.ai999x.com/models/earth-science-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-earth-science-1.6.6-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-earth-science-1.6.6-linux-x86_64.tar.gz) |
| **Command line** `earth-science` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-cli-1.6.6-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-cli-1.6.6-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/earth-science-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install earth-science-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Earth Science desktop app: every scene and chart is drawn from the computed answer.

[![Earth Science desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=earth-science-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=earth-science-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=earth-science-ai999x/clip.mp4)*

| | |
|---|---|
| [![Lifting condensation level (Romps 2017): the live scene in the Earth Science desktop app](https://www.ai999x.com/?ax_shot=earth-science-ai999x/scene-1.webp)](https://www.ai999x.com/models/earth-science-ai999x/#cap-lifting_condensation_level)<br>Lifting condensation level (Romps 2017) | [![In-situ density of seawater (EOS-80): the live scene in the Earth Science desktop app](https://www.ai999x.com/?ax_shot=earth-science-ai999x/scene-2.webp)](https://www.ai999x.com/models/earth-science-ai999x/#cap-seawater_density)<br>In-situ density of seawater (EOS-80) |
| [![Planetary equilibrium (effective radiating) temperature: the live scene in the Earth Science desktop app](https://www.ai999x.com/?ax_shot=earth-science-ai999x/scene-3.webp)](https://www.ai999x.com/models/earth-science-ai999x/#cap-planetary_equilibrium_temperature)<br>Planetary equilibrium (effective radiating) temperature | [![Moment magnitude from seismic moment: the live scene in the Earth Science desktop app](https://www.ai999x.com/?ax_shot=earth-science-ai999x/scene-4.webp)](https://www.ai999x.com/models/earth-science-ai999x/#cap-moment_magnitude)<br>Moment magnitude from seismic moment |
| [![Radiometric age from a daughter/parent ratio: the live scene in the Earth Science desktop app](https://www.ai999x.com/?ax_shot=earth-science-ai999x/scene-5.webp)](https://www.ai999x.com/models/earth-science-ai999x/#cap-radiometric_age)<br>Radiometric age from a daughter/parent ratio | [![Heat index (NWS): the live scene in the Earth Science desktop app](https://www.ai999x.com/?ax_shot=earth-science-ai999x/scene-6.webp)](https://www.ai999x.com/models/earth-science-ai999x/#cap-heat_index)<br>Heat index (NWS) |
| [![Period of inertial oscillations: the live scene in the Earth Science desktop app](https://www.ai999x.com/?ax_shot=earth-science-ai999x/scene-7.webp)](https://www.ai999x.com/models/earth-science-ai999x/#cap-inertial_period)<br>Period of inertial oscillations | [![Column density by Pratt isostasy: the live scene in the Earth Science desktop app](https://www.ai999x.com/?ax_shot=earth-science-ai999x/scene-8.webp)](https://www.ai999x.com/models/earth-science-ai999x/#cap-pratt_isostasy)<br>Column density by Pratt isostasy |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Lifting condensation level (Romps 2017): a chart in the Earth Science desktop app](https://www.ai999x.com/?ax_shot=earth-science-ai999x/chart-1.webp)](https://www.ai999x.com/models/earth-science-ai999x/#cap-lifting_condensation_level)<br>Lifting condensation level (Romps 2017) | [![In-situ density of seawater (EOS-80): a chart in the Earth Science desktop app](https://www.ai999x.com/?ax_shot=earth-science-ai999x/chart-2.webp)](https://www.ai999x.com/models/earth-science-ai999x/#cap-seawater_density)<br>In-situ density of seawater (EOS-80) | [![Planetary equilibrium (effective radiating) temperature: a chart in the Earth Science desktop app](https://www.ai999x.com/?ax_shot=earth-science-ai999x/chart-3.webp)](https://www.ai999x.com/models/earth-science-ai999x/#cap-planetary_equilibrium_temperature)<br>Planetary equilibrium (effective radiating) temperature | [![Moment magnitude from seismic moment: a chart in the Earth Science desktop app](https://www.ai999x.com/?ax_shot=earth-science-ai999x/chart-4.webp)](https://www.ai999x.com/models/earth-science-ai999x/#cap-moment_magnitude)<br>Moment magnitude from seismic moment |

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

## Command line: 15 examples

Each command below was run with `earth-science` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ earth-science solve heat_index "T=305.15 K" RH=0.7
HI = 313.5592737 K

$ earth-science solve inertial_period lat=45
T_inertial = 60927.21986 s
T_hours = 16.92422774
f = 0.0001031260793 1/s

$ earth-science solve brunt_vaisala_frequency "theta=300 K" "dtheta_dz=0.003 K/m"
N = 0.009902853124 1/s
period = 634.4823283 s

$ earth-science solve dewpoint_from_relative_humidity "T=303.15 K" RH=0.6
Td = 294.5065 K

$ earth-science solve scale_height "T=288.15 K"
H = 8434.351641 m

$ earth-science solve energy_balance_response "F=3.7 W/m^2" "lam=0.8 K*m^2/W" "C=420000000 J/(m^2*K)" t=10
dT = 1.802832762 K
dT_eq = 2.96 K
tau_years = 10.64719751

$ earth-science solve co2_radiative_forcing C=560 C0=280
dF = 3.708337416 W/m^2
doublings = 1

$ earth-science solve radiocarbon_age F=0.5
age = 5568.051301

$ earth-science solve pratt_isostasy "h=3000 m" "rho0=2800 kg/m^3" "D=100000 m"
rho = 2718.446602 kg/m^3
drho = 81.55339806 kg/m^3

$ earth-science solve dipole_inclination lat=45
inclination = 63.43494882
F_ratio = 1.58113883

$ earth-science solve stream_power "Q=100 m^3/s" S=0.002
Omega = 1961.33 W/m

$ earth-science solve gravity_at_latitude lat=45 "z=0 m"
g = 9.806189875 m/s^2

$ earth-science solve geotherm_radiogenic "T0=283.15 K" "q0=0.065 W/m^2" "k=2.5 W/(m*K)" "A=1e-06 W/m^3" "z=10000 m"
T = 523.15 K
q = 0.055 W/m^2

$ earth-science solve darcy_flow "K=0.0001 m/s" "A=50 m^2" "dh=2 m" "L=100 m"
Q = 0.0001 m^3/s
q = 2e-06 m/s
gradient = 0.02

$ earth-science solve map_gradient_relief "z1=1200 m" "z2=1450 m" "d_map=0.05 m" scale=24000
relief = 250 m
distance = 1200 m
gradient_m_per_km = 208.3333333
slope = 0.2083333333
slope_percent = 20.83333333
slope_deg = 11.76828893
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/earth-science-ai999x/blob/main/docs/earth_science_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/earth-science-ai999x/blob/main/docs/earth_science_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/earth-science-ai999x/blob/main/docs/earth_science_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/earth-science-ai999x/blob/main/docs/earth_science_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/earth-science-ai999x/blob/main/docs/earth_science_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/earth-science-ai999x/blob/main/docs/earth_science_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/earth-science-ai999x/blob/main/docs/earth_science_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/earth-science-ai999x/blob/main/docs/earth_science_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/earth-science-ai999x/blob/main/docs/earth_science_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Earth Science on ai999x.com](https://www.ai999x.com/models/earth-science-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.6 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/earth-science-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.6`; the [GitHub release](https://github.com/AI999X/earth-science-ai999x/releases/tag/v1.6.6) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `earth-science` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/earth-science`) and its data (`rust/models/earth_science/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p earth-science-ai999x
./target/release/earth-science --help
cargo test -p earth-science-ai999x
```
