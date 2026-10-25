# earth-science-ai999x — offline, verified earth science solver with step-by-step answers

[![tests](https://github.com/AI999X/earth-science-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/earth-science-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/earth-science-ai999x)](https://pypi.org/project/earth-science-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/earth-science-ai999x)

**[Install](https://pypi.org/project/earth-science-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/earth-science-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/earth-science-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

Field: **Physical Sciences** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

earth-science-ai999x is an offline, deterministic Python solver that answers earth science questions with verified results from 85 certified capabilities, gives 522 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**earth-science-ai999x** is a compact, deterministic earth science engine for Python. It reads a question in plain English (or a typed call), picks a certified earth science capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified earth science capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 522 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** atmospheric thermodynamics and meteorology, climate and radiation balance, oceanography and seawater properties, seismology and geophysics, and geology.

## Model overview

| Property | Value |
|---|---|
| Package | `earth-science-ai999x` 1.0.0 (import `earth_science_ai999x`) |
| Type | Deterministic neuro-symbolic earth science engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 85 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 522 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install earth-science-ai999x
```

```python
import earth_science_ai999x as earth_science

earth_science.ask("Lifting condensation level with pressure = 1000 hPa, temperature = 30 \u00b0C and dewpoint = 20 \u00b0C.").show()
# -> SOLVED — p_lcl = 86421.3, T_lcl = 290.812  (lifting_condensation_level)
earth_science.solve("seawater_density", S=35, T="2 degC", p="40 MPa").show()
# -> SOLVED — rho = 1046.02, sigma = 46.0168  (seawater_density)
earth_science.ask("Planetary equilibrium temperature with solar constant = 1361 W/m^2 and albedo = 0.3.").show()
# -> SOLVED — Te = 254.578, absorbed = 238.175  (planetary_equilibrium_temperature)
earth_science.ask("Moment magnitude with seismic moment = 1.1e21 N*m.").show()
# -> SOLVED — Mw = 7.99426, M0_dyne_cm = 1.1e+28  (moment_magnitude)
earth_science.solve("radiometric_age", ratio=0.5, half_life=1250000000.0).show()
# -> SOLVED — age = 7.31203e+08, lambda = 5.54518e-10  (radiometric_age)
earth_science.ask("Who created you?").message
# -> earth-science-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `earth-science-ai999x "<question>"`, `earth-science-ai999x --capabilities`. The outputs above were produced by running this exact code on earth-science-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

85 certified procedures. A few of them:

- **Procedures:** Adiabatic temperature gradient of seawater (Bryden 1973); Atmospheric scale height; Brunt-Väisälä (buoyancy) frequency; Column density by Pratt isostasy; Conventional radiocarbon age; Coriolis parameter; Daily mean insolation at the top of the atmosphere; Density of moist air; Density of pure water (SMOW, Bigg 1967); Density of seawater at the surface (UNESCO 1981); Depth from sea pressure (Fofonoff and Millard 1983); Depth of the ocean floor with age (half-space cooling); Dewpoint from temperature and relative humidity; Dewpoint from vapour pressure; Distance to an earthquake from the S-P interval; Ekman layer depth; Ekman transport; Elastic moduli and Poisson's ratio from Vp, Vs and density; Epicentral distance on a spherical Earth; Equivalent potential temperature (Bolton 1980); Fraction of a radioactive parent remaining; Freezing point of seawater (UNESCO 1983); Geodesic distance and azimuths on the WGS84 ellipsoid (Vincenty); Geopotential and geopotential height …

`earth_science_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified earth science capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (atmospheric thermodynamics and meteorology, climate and radiation balance, oceanography and seawater properties, seismology and geophysics, and geology); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{earth_science_ai999x_1_0_0,
  title  = {earth-science-ai999x: an offline, verified earth science solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/earth-science-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/earth-science-ai999x/discussions or visit https://ai999x.com.

## Support the project

earth-science-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/earth-science-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/earth-science-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/earth-science-ai999x/tree/main/onnx).
