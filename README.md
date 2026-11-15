# Earth Science (earth-science-ai999x)

[![tests](https://github.com/AI999X/earth-science-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/earth-science-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/earth-science-ai999x)](https://pypi.org/project/earth-science-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/earth-science-ai999x)

Earth Science covers atmospheric thermodynamics and meteorology, climate and radiation balance, oceanography and seawater properties, seismology and geophysics, and geology: 87 calculations in 21 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Earth Science desktop app: Atmospheric scale height, with its worked example and live scene](https://www.ai999x.com/?ax_shot=earth-science-ai999x/hero.webp)](https://www.ai999x.com/models/earth-science-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://downloads.ai999x.com/desktop/latest/ai999x-windows-x86_64.zip) · [Linux](https://downloads.ai999x.com/desktop/latest/ai999x-linux-x86_64.tar.gz) |
| **Command line** `earth-science` (Windows, Linux) | [Windows](https://downloads.ai999x.com/desktop/latest/ai999x-cli-windows-x86_64.zip) · [Linux](https://downloads.ai999x.com/desktop/latest/ai999x-cli-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/earth-science-ai999x/#try) |
| **Python** 3.10+ | `pip install earth-science-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Example

```python
import earth_science_ai999x as earth_science
a = earth_science.ask("Lifting condensation level with pressure = 1000 hPa, temperature = 30 \u00b0C and dewpoint = 20 \u00b0C.")
print(a.outcome, a.value, a.unit)
```

Output:

```text
SOLVED 86421.3026232 Pa
```

Every calculation, with its formula, a worked example and its figures: **[Earth Science on ai999x.com](https://www.ai999x.com/models/earth-science-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.3 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/earth-science-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.3`; the [GitHub release](https://github.com/AI999X/earth-science-ai999x/releases/tag/v1.6.3) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `earth-science` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/earth-science`) and its data (`rust/models/earth_science/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p earth-science-ai999x
./target/release/earth-science --help
cargo test -p earth-science-ai999x
```
