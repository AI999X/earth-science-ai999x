# earth-science: the Earth Science program, user guide

**earth-science-ai999x 1.6.0**, Rust build: command line, desktop app and browser. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Earth Science covers atmospheric thermodynamics and meteorology, climate and radiation balance, oceanography and seawater properties, seismology and geophysics, and geology. This guide is about `earth-science`, the model as a self-contained program for Windows and Linux: 87 capabilities in 21 topics, answered on your own computer with no network, no Python and nothing else to install. The same compiled model runs in the AI999X desktop app and in the browser on the model's page (https://www.ai999x.com/models/earth-science-ai999x/); the Python package (`pip install earth-science-ai999x`) has its own guide. Every command output printed here is what the program printed.

## Contents

- 1. Get it: desktop app, command line, browser, Python
- 2. Quick start
- 3. Writing inputs
- 4. Outcomes and exit codes
- 5. Scripts and agents: batch and JSON
- 6. Figures
- 7. In a web page or Node.js: WebAssembly
- 8. Every capability

## 1. Get it

Version 1.6.0 of the AI999X desktop app and of the command-line bundle. Checksums for every file: https://downloads.ai999x.com/desktop/1.6.0/SHA256SUMS; the newest version is always under https://downloads.ai999x.com/desktop/latest/. Full instructions: https://www.ai999x.com/desktop-cli/

| what | system | download |
|---|---|---|
| Desktop app: Earth Science | Windows x86-64 | https://downloads.ai999x.com/desktop/1.6.0/ai999x-earth-science-1.6.0-windows-x86_64.zip |
| Desktop app: Earth Science | Linux x86-64 | https://downloads.ai999x.com/desktop/1.6.0/ai999x-earth-science-1.6.0-linux-x86_64.tar.gz |
| Desktop app: all models | Windows x86-64 | https://downloads.ai999x.com/desktop/1.6.0/ai999x-1.6.0-windows-x86_64.zip |
| Desktop app: all models | Linux x86-64 | https://downloads.ai999x.com/desktop/1.6.0/ai999x-1.6.0-linux-x86_64.tar.gz |
| Command line | Windows x86-64 | https://downloads.ai999x.com/desktop/1.6.0/ai999x-cli-1.6.0-windows-x86_64.zip |
| Command line | Linux x86-64 | https://downloads.ai999x.com/desktop/1.6.0/ai999x-cli-1.6.0-linux-x86_64.tar.gz |

### Desktop app

Earth Science in its own app: search, questions in words, input forms, live figures and explanations. Windows: Download ai999x-earth-science-1.6.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-earth-science.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.6.0/ai999x-earth-science-1.6.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.6.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-earth-science-1.6.0-linux-x86_64.tar.gz
./ai999x-earth-science-1.6.0-linux-x86_64/ai999x-earth-science
```

It opens on this model: type a question or pick a capability, fill in the form and press Compute. The AI999X desktop app with every model works the same way (choose the model first). Windows: Download ai999x-1.6.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.6.0/ai999x-1.6.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.6.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-1.6.0-linux-x86_64.tar.gz
./ai999x-1.6.0-linux-x86_64/ai999x
```

### Command line: Linux

The bundle holds one program per model; `earth-science` is this one. The checksum line should print OK.

```shell
curl -LO https://downloads.ai999x.com/desktop/1.6.0/ai999x-cli-1.6.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.6.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.6.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.6.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
earth-science --version
```

### Command line: Windows (PowerShell)

The fourth line should print True.

```powershell
$F = "ai999x-cli-1.6.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.6.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.6.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.6.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
earth-science --version
```

### In your browser

Open https://www.ai999x.com/models/earth-science-ai999x/, choose a capability and press Compute in your browser: the same program runs on your device as WebAssembly; nothing is sent anywhere.

### Python

```shell
pip install earth-science-ai999x
```

## 2. Quick start

### 2.1 Check the install

Prints the model, its version and that this is the Rust build.

```shell
$ earth-science --version
earth-science-ai999x 1.6.0 (Rust build, ax-engine 1.6.0)
```

### 2.2 Ask in words

Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```shell
$ earth-science ask "Lifting condensation level with pressure = 1000 hPa, temperature = 30 °C and dewpoint = 20 °C."
Read as: Lifting condensation level (Romps 2017)
p_lcl = 86421.30262 Pa
T_lcl = 290.8124964 K
  by: Lifting condensation level (Romps 2017)  (Romps (2017) Eq. 22: T_LCL = c/W_{-1}(RH^{1/a} c e^c) T, p_LCL = p (T_LCL/T)^{c_pm/R_m})
  T = 303.15 K
  Td = 293.15 K
  p = 100000 Pa
  assumes: Td <= T; 100 hPa <= p <= 1100 hPa
```

### 2.3 Find a capability

Search the capabilities by what you want to know; the first column is the id to pass to info and solve.

```shell
$ earth-science search "lifting condensation level (romps 2017)"
lifting_condensation_level           Lifting condensation level (Romps 2017)  (atmospheric thermodynamics)
dry_adiabatic_ascent                 Temperature of a dry parcel after adiabatic ascent  (atmospheric thermodynamics)
standard_atmosphere_height           Height of a pressure level in the standard atmosphere  (atmospheric thermodynamics)
pratt_isostasy                       Column density by Pratt isostasy  (geodynamics)
airy_isostasy                        Mountain root by Airy isostasy  (geodynamics)
brunt_vaisala_frequency              Brunt-Väisälä (buoyancy) frequency  (atmospheric dynamics)
geopotential_height                  Geopotential and geopotential height  (atmospheric thermodynamics)
gravity_at_latitude                  Gravitational acceleration at a latitude and height  (geophysics)
… 4 more
```

### 2.4 List the capabilities

Every capability the program answers: its id (what you pass to solve) and its name.

```shell
$ earth-science list
airy_isostasy                            Mountain root by Airy isostasy
blackbody_flux                           Radiant flux emitted by a black or grey body (Stefan-Boltzmann)
bouguer_slab                             Gravity anomaly of an infinite horizontal slab (Bouguer)
brunt_vaisala_frequency                  Brunt-Väisälä (buoyancy) frequency
buried_sphere_anomaly                    Gravity anomaly above a buried sphere
co2_radiative_forcing                    Radiative forcing of a change in CO2 (Myhre et al. 1998)
coriolis_parameter                       Coriolis parameter
daily_mean_insolation                    Daily mean insolation at the top of the atmosphere
… 79 more
```

### 2.5 What a capability needs

The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```shell
$ earth-science info lifting_condensation_level
Lifting condensation level (Romps 2017)
  Romps (2017) Eq. 22: T_LCL = c/W_{-1}(RH^{1/a} c e^c) T, p_LCL = p (T_LCL/T)^{c_pm/R_m}
  in   p        starting pressure [Pa]
  in   T        starting temperature [K]
  in   Td       starting dewpoint [K]
  out  p_lcl    LCL pressure (Pa) [Pa]
  out  T_lcl    LCL temperature (K) [K]
```

### 2.6 Solve

Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```shell
$ earth-science solve lifting_condensation_level "p=100000 Pa" "T=303.15 K" "Td=293.15 K"
p_lcl = 86421.30262 Pa
T_lcl = 290.8124964 K
  by: Lifting condensation level (Romps 2017)  (Romps (2017) Eq. 22: T_LCL = c/W_{-1}(RH^{1/a} c e^c) T, p_LCL = p (T_LCL/T)^{c_pm/R_m})
  p = 100000 Pa
  T = 303.15 K
  Td = 293.15 K
  assumes: Td <= T; 100 hPa <= p <= 1100 hPa
```

### 2.7 Machine-readable answer

--json prints the same answer as one line of JSON (values in SI): for scripts and other programs.

```shell
$ earth-science solve lifting_condensation_level "p=100000 Pa" "T=303.15 K" "Td=293.15 K" --json
{"outcome":"SOLVED","message":"","value":86421.3026232,"unit":"Pa","solved_for":"p_lcl","si":[["p",100000.0],["T",303.15],["Td",293.15]],"values":[["p_lcl",86421.3026232,"Pa"],["T_lcl",290.812496352,"K"]],"target":"p_lcl","phrase":"LCL pressure (Pa)","assumptions":["Td <= T","100 hPa <= p <= 1100 hPa"],"name":"Lifting condensation level (Romps 2017)","equation":"Romps (2017) Eq. 22: T_LCL = c/W_{-1}(RH^{1/a} c e^c) T, p_LCL = p (T_LCL/T)^{c_pm/R_m}"}
```

### 2.8 When an input is missing

No number is guessed: the answer names what is missing, shows a complete call, and the program exits with status 1.

```shell
$ earth-science solve lifting_condensation_level "p=100000 Pa" "T=303.15 K"
NEEDS_INPUT: lifting_condensation_level needs Td (starting dewpoint). Example: solve('lifting_condensation_level', p='100000 Pa', T='303.15 K', Td='293.15 K').
```

### 2.9 Many questions at once

One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | earth-science batch

`cases.jsonl`:

```json
{"key": "lifting_condensation_level", "inputs": {"p": "100000 Pa", "T": "303.15 K", "Td": "293.15 K"}}
{"key": "seawater_density", "inputs": {"S": 35, "T": "2 degC", "p": "40 MPa"}}
```

```shell
$ earth-science batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":86421.3026232,"unit":"Pa","solved_for":"p_lcl","si":[["p",100000.0],["T",303.15],["Td",293.15]],"values":[["p_lcl",86421.3026232,"Pa"],["T_lcl",290.812496352,"K"]],"target":"p_lcl","phrase":"LCL pressure (Pa)","assumptions":["Td <= T","100 hPa <= p <= 1100 hPa"],"name":"Lifting condensation level (Romps 2017)","equation":"Romps (2017) Eq. 22: T_LCL = c/W_{-1}(RH^{1/a} c e^c) T, p_LCL = p (T_LCL/T)^{c_pm/R_m}"}
{"outcome":"SOLVED","message":"","value":1046.01684344,"unit":"kg/m^3","solved_for":"rho","si":[["S",35.0],["T",275.15],["p",40000000.0]],"values":[["rho",1046.01684344,"kg/m^3"],["sigma",46.0168434363,""]],"target":"rho","phrase":"in-situ density (kg/m^3)","assumptions":["0 <= S <= 42","-2 °C <= T <= 40 °C","0 <= p <= 10,000 dbar"],"name":"In-situ density of seawater (EOS-80)","equation":"rho = rho(S,T,0)/(1 - P/K(S,T,P)), P in bar (UNESCO 1983)"}
```

## 3. Writing inputs

Inputs are written `name=value`, one per argument; `earth-science info <capability>` lists the names. Quote an argument that holds a space or brackets (bash, zsh and PowerShell all accept double quotes). The answer always states each input as it was read, converted to SI, so a misread unit is visible at once.

| kind | example | how it is read |
|---|---|---|
| a number with a unit | `"R=0.128 nm"` | any unit of the same kind; converted to SI |
| a number in SI | `R=1.28e-10` | no unit means the SI unit shown by info |
| a choice | `structure=FCC` | one of the values info lists |
| a list or vector | `"v=[1, 2, 3]" or "v=1, 2, 3"` | brackets keep it a list |
| a matrix | `"A=[[1, 2], [3, 4]]"` | one list per row |
| true / false | `inclusive=true` | for yes/no options |

An equation can be solved for any of its quantities: give the others and add `--target <symbol>`. Without `--target`, the program solves for the quantity the capability is named after.

## 4. Outcomes and exit codes

The program never guesses a number: when it cannot answer, it says why and exits with status 1, so a script can tell an answer from a refusal without reading the text.

| outcome | meaning | exit status |
|---|---|---|
| SOLVED | the answer: every output with its unit, the method, the inputs in SI, the conditions assumed | 0 |
| NEEDS_INPUT | an input is missing; the message names it and shows a complete call | 1 |
| INVALID_INPUT | a value or unit of the wrong kind, an unknown input name, or a value outside the conditions | 1 |
| AMBIGUOUS | the inputs allow two different answers; say which one is meant | 1 |
| OUT_OF_SCOPE | the model has no capability for this, or the result would not be meaningful | 1 |
| (usage) | a command the program does not know: it prints its usage | 2 |

## 5. Scripts and agents: batch and JSON

`earth-science solve ... --json` prints one answer as one line of JSON. `earth-science batch` reads one request per line on standard input and writes one JSON answer per line, in order, starting the program once for any number of requests. A request is `{"key": "<capability>", "inputs": {"name": "value", ...}, "target": null}`. In PowerShell, pipe the file in: `Get-Content cases.jsonl | earth-science batch`. `earth-science catalog` prints every capability with its inputs as JSON, to build forms or tool definitions.

| answer field | meaning |
|---|---|
| outcome | one of the outcomes above |
| message | why there is no answer (empty when SOLVED) |
| value, unit | the main result in SI units |
| values | every output: [symbol, value, unit] (procedures) |
| target | the quantity solved for |
| si | each numeric input converted to SI: [symbol, value] |
| assumptions | the conditions the answer relies on |
| name, equation | the capability and its equation or method |

```python
import json, subprocess

reqs = [{"key": "<capability>", "inputs": {"name": "value"}}]
out = subprocess.run(["earth-science", "batch"], input="\n".join(map(json.dumps, reqs)),
                     capture_output=True, text=True, check=True).stdout
answers = [json.loads(line) for line in out.splitlines()]
```

## 6. Figures

`earth-science figures <capability> name=value ... --out DIR` draws the live figures for those inputs and writes each as an SVG file into DIR (an existing folder); without `--out` the SVG text comes back as JSON. The desktop app and the model page draw the same figures as soon as you press Compute.

## 7. In a web page or Node.js: WebAssembly

The same model is a WebAssembly module, `earth_science_ai999x.wasm` (https://www.ai999x.com/compute/rust/earth_science_ai999x.wasm), with a small loader, `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js). It runs in a browser, a Web Worker or Node.js, with no server.

```html
<script src="ax-wasm.js"></script>
<script>
  AXWasm.load("earth_science_ai999x.wasm").then(function (m) {
    var a = m.solve({key: "<capability>", inputs: {name: "value"}});
    console.log(a.outcome, a.values || a.value);
    console.log(m.catalog().capabilities.length + " capabilities");
  });
</script>
```

## 8. Every capability

By topic. For each: what it does, what it needs and gives, when it applies, and a command with what the program printed for it. `earth-science info <id>` shows the same inputs and outputs in the terminal.

### Applied meteorology (2)

<a id="heat_index"></a>

#### Heat index (NWS): `heat_index`

The apparent temperature of the US National Weather Service: Rothfusz's regression with the NWS adjustments for low and high humidity.

$$
HI = -42.379 + 2.049T + 10.143RH - 0.2248\,T\,RH - \dots\ (^\circ\mathrm{F},\ RH\ \mathrm{in}\ \%)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | air temperature | K |
| `RH` | in | relative humidity (fraction, or % in a question) | - |
| `HI` | out | heat index (K) | K |

**Applies when:** 80 °F <= T <= 130 °F; 0 <= RH <= 1.

```shell
$ earth-science solve heat_index "T=305.15 K" RH=0.7
HI = 313.5592737 K
  by: Heat index (NWS)  (Rothfusz (1990) regression with the NWS adjustments (as MetPy), computed in °F)
  T = 305.15 K
  RH = 0.7
  assumes: 80 °F <= T <= 130 °F; 0 <= RH <= 1
```

**What the worked example means:** This calculation determines the apparent temperature by combining the effects of air heat and humidity on the body's ability to cool itself through evaporation. Using the National Weather Service method, an input air temperature of 305.15 K and a relative humidity value of 0.7 produce a heat index result of 313.559 K. This metric helps assess potential health risks such as exhaustion or stroke during hot weather conditions.

<a id="wind_chill"></a>

#### Wind chill temperature index: `wind_chill`

The wind chill temperature index of the 2001 North American formula (wind measured at 10 m).

$$
WCT = 13.12 + 0.6215\,T - 11.37\,V^{0.16} + 0.3965\,T\,V^{0.16}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | air temperature | K |
| `V` | in | wind speed at 10 m | m/s |
| `WCT` | out | wind chill temperature (K) | K |

**Applies when:** T <= 10 °C; V > 3 mph (4.8 km/h).

```shell
$ earth-science solve wind_chill "T=263.15 K" "V=8.333333333 m/s"
WCT = 253.629502 K
  by: Wind chill temperature index  (WCT = 13.12 + 0.6215 T - 11.37 V^0.16 + 0.3965 T V^0.16 (T in °C, V in km/h))
  T = 263.15 K
  V = 8.333333333 m/s
  assumes: T <= 10 °C; V > 3 mph (4.8 km/h)
```

**What the worked example means:** This calculation determines the wind chill temperature index, which represents how cold the air feels to human skin due to accelerated heat loss. Using the 2001 North American formula with inputs of air temperature 263.15 K and wind speed at ten meters 8.33333 m/s, the resulting perceived temperature is 253.63 K. This value helps assess the risk of frostbite and hypothermia under these specific weather conditions.

### Atmospheric and ocean dynamics (1)

<a id="inertial_period"></a>

#### Period of inertial oscillations: `inertial_period`

The period 2 pi/|f| of free inertial oscillations of the wind or an ocean current at a latitude.

$$
T_i = \frac{2\pi}{|f|}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lat` | in | latitude (degrees, north positive) | - |
| `T_inertial` | out | inertial period (s) | s |
| `T_hours` | out | inertial period (hours) | - |
| `f` | out | Coriolis parameter (1/s) | 1/s |

**Applies when:** |latitude| >= 0.5 degrees.

```shell
$ earth-science solve inertial_period lat=45
T_inertial = 60927.21986 s
T_hours = 16.92422774
f = 0.0001031260793 1/s
  by: Period of inertial oscillations  (T = 2 pi/|f| with f = 2 Omega sin(phi))
  lat = 45
  assumes: |latitude| >= 0.5 degrees
```

**What the worked example means:** This calculation determines the time required for a free-moving object to complete one circular path due to the Coriolis effect alone. Using the latitude 45, it computes the Coriolis parameter as 0.000103126 1/s and derives the inertial period of 60927.2 s. This result represents the natural timescale for oscillations in ocean currents or atmospheric winds at that specific location.

### Atmospheric dynamics (8)

<a id="brunt_vaisala_frequency"></a>

#### Brunt-Väisälä (buoyancy) frequency, `brunt_vaisala_frequency`

The buoyancy frequency of a stably stratified atmosphere from the potential temperature and its vertical gradient.

$$
N = \sqrt{\frac{g}{\theta}\frac{\partial\theta}{\partial z}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `theta` | in | potential temperature at the level | K |
| `dtheta_dz` | in | vertical gradient of potential temperature | K/m |
| `N` | out | Brunt-Väisälä frequency (1/s) | 1/s |
| `period` | out | buoyancy period 2 pi/N (s) | s |

**Applies when:** d theta/dz > 0 (stable stratification).

```shell
$ earth-science solve brunt_vaisala_frequency "theta=300 K" "dtheta_dz=0.003 K/m"
N = 0.009902853124 1/s
period = 634.4823283 s
  by: Brunt-Väisälä (buoyancy) frequency  (N = sqrt((g/theta) d theta/dz))
  theta = 300 K
  dtheta_dz = 0.003 K/m
  assumes: d theta/dz > 0 (stable stratification)
```

**What the worked example means:** This calculation determines the stability of the atmosphere by computing the Brunt-Väisälä frequency from the potential temperature and its vertical gradient. The resulting frequency of 0.00990285 1/s indicates how strongly an air parcel will oscillate if displaced vertically within this stable layer, corresponding to a full oscillation cycle that takes 634.482 s to complete.

<a id="coriolis_parameter"></a>

#### Coriolis parameter: `coriolis_parameter`

The Coriolis parameter f = 2 Omega sin(latitude).

$$
f = 2\Omega\sin\varphi
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lat` | in | latitude (degrees, north positive) | - |
| `f` | out | Coriolis parameter (1/s) | 1/s |

**Applies when:** -90 <= latitude <= 90 degrees.

```shell
$ earth-science solve coriolis_parameter lat=45
f = 0.0001031260793 1/s
  by: Coriolis parameter  (f = 2 Omega sin(phi), Omega = 7.292115e-5 rad/s)
  lat = 45
  assumes: -90 <= latitude <= 90 degrees
```

**What the worked example means:** This calculation determines the Coriolis parameter, which quantifies the strength of the Coriolis effect due to Earth's rotation at a specific location. Using the formula f equals two times Omega times sine of phi, where Omega is the constant angular velocity of Earth and phi is the latitude in degrees, the result for an input latitude of 45 yields a value of 0.000103126 1/s. This parameter is essential for analyzing large-scale atmospheric and oceanic movements, indicating how strongly rotating reference frames influence fluid motion at that geographical position.

<a id="geostrophic_wind"></a>

#### Geostrophic wind from a height gradient: `geostrophic_wind`

Geostrophic wind speed from the change of geopotential height of a pressure surface over a horizontal distance.

$$
V_g = \frac{g}{|f|}\left|\frac{\Delta Z}{\Delta n}\right|
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lat` | in | latitude (degrees, north positive) | - |
| `dz` | in | change of geopotential height across the distance | m |
| `dn` | in | horizontal distance over which the height changes | m |
| `Vg` | out | geostrophic wind speed (m/s) | m/s |
| `f` | out | Coriolis parameter (1/s) | 1/s |

**Applies when:** 5 <= |latitude| <= 85 degrees; dn > 0.

```shell
$ earth-science solve geostrophic_wind lat=45 "dz=60 m" "dn=500000 m"
Vg = 11.41125511 m/s
f = 0.0001031260793 1/s
  by: Geostrophic wind from a height gradient  (V_g = (g/|f|) |dZ/dn|)
  lat = 45
  dz = 60 m
  dn = 500000 m
  assumes: 5 <= |latitude| <= 85 degrees; dn > 0
```

**What the worked example means:** This calculation determines the theoretical wind speed resulting from the balance between the pressure gradient force and the Coriolis force. Using a latitude of 45, the Coriolis parameter is computed as 0.000103126 1/s. The vertical change in geopotential height, given as 60 m, across the horizontal distance 500000 m yields a geostrophic wind speed of 11.4113 m/s. This value represents the speed at which air would flow parallel to constant pressure surfaces if friction were negligible.

<a id="gradient_wind"></a>

#### Gradient wind in curved flow: `gradient_wind`

The gradient wind speed of curved, frictionless flow from the geostrophic wind, the radius of curvature of the trajectory and the latitude, for cyclonic or anticyclonic flow (the regular solution).

$$
\frac{V^2}{R} \pm fV = fV_g
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lat` | in | latitude (degrees, north positive) | - |
| `R` | in | radius of curvature of the flow | m |
| `Vg` | in | geostrophic wind speed | m/s |
| `flow` | in | cyclonic or anticyclonic | - |
| `V` | out | gradient wind speed (m/s) | m/s |
| `ratio` | out | V/Vg | - |
| `f` | out | \|f\| (1/s) | 1/s |

**Applies when:** 5 <= |latitude| <= 85 degrees; R, Vg > 0; anticyclonic: Vg <= |f| R/4.

```shell
$ earth-science solve gradient_wind lat=45 "R=500000 m" "Vg=20 m/s" flow=cyclonic
V = 15.40036403 m/s
ratio = 0.7700182013
f = 0.0001031260793 1/s
  by: Gradient wind in curved flow  (cyclonic V = -fR/2 + sqrt(f^2R^2/4 + f R Vg); anticyclonic V = fR/2 - sqrt(f^2R^2/4 - f R Vg))
  lat = 45
  R = 500000 m
  Vg = 20 m/s
  flow = cyclonic
  assumes: 5 <= |latitude| <= 85 degrees; R, Vg > 0; anticyclonic: Vg <= |f| R/4
```

**What the worked example means:** This calculation determines the gradient wind speed for curved, frictionless flow using the given latitude, radius of curvature, and geostrophic wind speed. The result, 15.4004 m/s m/s, represents the actual wind velocity where pressure gradient, Coriolis, and centrifugal forces balance. This value is derived from a geostrophic wind of 20 m/s m/s along a path with a radius of 500000 m m. The ratio of the gradient wind to the geostrophic wind is 0.770018, indicating how much the curvature modifies the straight-flow estimate. The Coriolis parameter magnitude used was 0.000103126 1/s 1/s, determined by the latitude of 45 degrees.

<a id="rossby_wave_speed"></a>

#### Phase speed of a barotropic Rossby wave: `rossby_wave_speed`

Zonal phase speed of a barotropic Rossby wave in a uniform westerly flow on a beta-plane, the beta parameter, and the stationary wavelength 2 pi sqrt(U/beta).

$$
c = U - \frac{\beta}{k^2},\quad \beta = \frac{2\Omega\cos\varphi}{a},\quad L_s = 2\pi\sqrt{U/\beta}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `U` | in | zonal mean (westerly) wind speed | m/s |
| `lat` | in | latitude (degrees, north positive) | - |
| `Lx` | in | zonal wavelength | m |
| `c` | out | zonal phase speed (m/s; negative = westward) | m/s |
| `beta` | out | beta = df/dy (1/(m s)) | 1/(m*s) |
| `Ls` | out | stationary wavelength (m) | m |

**Applies when:** U > 0; 5 <= |latitude| <= 85; no meridional wavenumber (l = 0).

```shell
$ earth-science solve rossby_wave_speed "U=15 m/s" lat=45 "Lx=6000000 m"
c = 0.2394320651 m/s
beta = 1.618677403e-11 1/(m*s)
Ls = 6048467.424 m
  by: Phase speed of a barotropic Rossby wave  (c = U - beta/k^2, beta = 2 Omega cos(phi)/a, L_s = 2 pi sqrt(U/beta))
  U = 15 m/s
  lat = 45
  Lx = 6000000 m
  assumes: U > 0; 5 <= |latitude| <= 85; no meridional wavenumber (l = 0)
```

**What the worked example means:** This calculation determines the zonal phase speed of a barotropic Rossby wave, which is 0.239432 m/s. It also computes the beta parameter, representing the variation of the Coriolis force with latitude, found to be 1.61868e-11 1/(m*s), and the stationary wavelength where the wave does not move relative to the ground, calculated as 6.04847e+06 m. These values are derived from the background wind speed, latitude, and the specific zonal wavelength provided.

<a id="rossby_number"></a>

#### Rossby number: `rossby_number`

The Rossby number Ro = U/(|f| L) of a flow with speed scale U and length scale L at a latitude: the ratio of inertial to Coriolis accelerations.

$$
Ro = \frac{U}{|f|\,L}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `U` | in | velocity scale | m/s |
| `L` | in | horizontal length scale | m |
| `lat` | in | latitude (degrees, north positive) | - |
| `Ro` | out | Rossby number | - |
| `f` | out | \|f\| (1/s) | 1/s |

**Applies when:** |latitude| >= 1 degree; U, L > 0.

```shell
$ earth-science solve rossby_number "U=10 m/s" "L=1000000 m" lat=45
Ro = 0.09696868209
f = 0.0001031260793 1/s
  by: Rossby number  (Ro = U/(|f| L), f = 2 Omega sin(latitude))
  U = 10 m/s
  L = 1000000 m
  lat = 45
  assumes: |latitude| >= 1 degree; U, L > 0
```

**What the worked example means:** This calculation determines the Rossby number, which represents the ratio of inertial forces to Coriolis forces in a fluid flow. It uses the provided velocity scale 10 m/s, horizontal length scale 1e+06 m, and latitude 45 to compute the Coriolis parameter magnitude 0.000103126 1/s. The resulting dimensionless value 0.0969687 indicates whether rotation effects dominate the system's dynamics.

<a id="wind_components"></a>

#### Wind components from speed and direction: `wind_components`

The eastward (u) and northward (v) wind components from the speed and the direction the wind blows FROM.

$$
u = -V\sin\delta,\quad v = -V\cos\delta
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `speed` | in | wind speed | m/s |
| `direction` | in | direction the wind blows from (degrees clockwise from north) | - |
| `u` | out | eastward component (m/s) | m/s |
| `v` | out | northward component (m/s) | m/s |

**Applies when:** 0 <= direction <= 360; meteorological convention: the direction the wind comes from.

```shell
$ earth-science solve wind_components "speed=10 m/s" direction=225
u = 7.071067812 m/s
v = 7.071067812 m/s
  by: Wind components from speed and direction  (u = -V sin(dir), v = -V cos(dir))
  speed = 10 m/s
  direction = 225
  assumes: 0 <= direction <= 360; meteorological convention: the direction the wind comes from
```

**What the worked example means:** This calculation converts the meteorological wind speed of 10 m/s and direction of 225 degrees into Cartesian vector components. The resulting eastward component is 7.07107 m/s and the northward component is 7.07107 m/s. In this convention, the direction indicates where the wind originates, so a northerly wind produces a negative northward value while an easterly wind produces a positive eastward value. These components allow the wind vector to be used in models that require orthogonal axes rather than polar coordinates.

<a id="wind_speed_direction"></a>

#### Wind speed and direction from components: `wind_speed_direction`

Wind speed and the direction the wind blows from (degrees clockwise from north, (0, 360]) from u and v.

$$
V = \sqrt{u^2+v^2},\quad \delta = 90^\circ - \operatorname{atan2}(-v,-u)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `u` | in | eastward wind component | m/s |
| `v` | in | northward wind component | m/s |
| `speed` | out | wind speed (m/s) | m/s |
| `direction` | out | direction the wind comes from (degrees) | - |

**Applies when:** calm (u = v = 0) gives direction 0.

```shell
$ earth-science solve wind_speed_direction "u=5 m/s" "v=-3 m/s"
speed = 5.830951895 m/s
direction = 300.9637565
  by: Wind speed and direction from components  (V = hypot(u, v); direction = 90 - atan2(-v, -u) in degrees)
  u = 5 m/s
  v = -3 m/s
  assumes: calm (u = v = 0) gives direction 0
```

**What the worked example means:** This calculation determines the total wind speed and meteorological direction from the eastward and northward components. The speed, given as 5.83095 m/s, represents the magnitude of the wind vector derived from inputs 5 m/s and -3 m/s. The direction, reported as 300.964, indicates where the wind originates, measured in degrees clockwise from true north. This conversion allows raw vector data to be interpreted in standard weather reporting formats.

### Atmospheric moisture (7)

<a id="dewpoint_from_relative_humidity"></a>

#### Dewpoint from temperature and relative humidity, `dewpoint_from_relative_humidity`

The dewpoint of air at temperature T and relative humidity RH: Bolton's inversion applied to RH e_s(T).

$$
T_d = T_d^{\mathrm{Bolton}}\!\left(RH\,e_s(T)\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | air temperature | K |
| `RH` | in | relative humidity (fraction, or % in a question) | - |
| `Td` | out | dewpoint (K) | K |

**Applies when:** 0.01 <= RH <= 1.

```shell
$ earth-science solve dewpoint_from_relative_humidity "T=303.15 K" RH=0.6
Td = 294.5065 K
  by: Dewpoint from temperature and relative humidity  (Td = Bolton inversion of RH e_s(T), e_s from Ambaum (2020) (MetPy's convention))
  T = 303.15 K
  RH = 0.6
  assumes: 0.01 <= RH <= 1
```

**What the worked example means:** This calculation determines the dewpoint temperature using the air temperature and relative humidity. By applying Bolton's inversion to the saturation vapor pressure derived from the given temperature, the method finds the specific moisture content implied by the relative humidity. The resulting dewpoint of 294.506 K represents the temperature to which the air must be cooled at constant pressure to become saturated, providing a direct measure of atmospheric moisture availability.

<a id="dewpoint_from_vapor_pressure"></a>

#### Dewpoint from vapour pressure: `dewpoint_from_vapor_pressure`

The dewpoint at which the given vapour pressure would saturate (inversion of Bolton's 1980 formula).

$$
T_d = \frac{243.5\,\ln(e/6.112)}{17.67-\ln(e/6.112)}\ ^\circ\mathrm{C}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `e` | in | water vapour partial pressure | Pa |
| `Td` | out | dewpoint (K) | K |

**Applies when:** 0.1 hPa <= e <= 200 hPa.

```shell
$ earth-science solve dewpoint_from_vapor_pressure "e=1700 Pa"
Td = 288.1130551 K
  by: Dewpoint from vapour pressure  (Td = 243.5 v/(17.67 - v) °C with v = ln(e/6.112 hPa))
  e = 1700 Pa
  assumes: 0.1 hPa <= e <= 200 hPa
```

**What the worked example means:** This calculation determines the dewpoint temperature, which is the specific temperature at which air becomes saturated with water vapor and condensation begins. Given a measured water vapour partial pressure of 1700 Pa, the algorithm applies an inversion of Bolton's empirical saturation formula to compute the corresponding dewpoint value of 288.113 K. This result indicates the thermal condition required for the existing moisture content to reach saturation, providing critical insight into atmospheric humidity levels.

<a id="relative_humidity_from_dewpoint"></a>

#### Relative humidity from temperature and dewpoint, `relative_humidity_from_dewpoint`

Relative humidity as the ratio of the vapour pressure at the dewpoint to the saturation vapour pressure.

$$
RH = \frac{e_s(T_d)}{e_s(T)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | air temperature | K |
| `Td` | in | dewpoint temperature | K |
| `RH` | out | relative humidity (fraction) | - |
| `RH_percent` | out | relative humidity (%) | - |

**Applies when:** Td <= T; -45 °C <= T, Td <= 60 °C.

```shell
$ earth-science solve relative_humidity_from_dewpoint "T=298.15 K" "Td=288.15 K"
RH = 0.5385566217
RH_percent = 53.85566217
  by: Relative humidity from temperature and dewpoint  (e_s(Td)/e_s(T), Ambaum (2020) e_s)
  T = 298.15 K
  Td = 288.15 K
  assumes: Td <= T; -45 °C <= T, Td <= 60 °C
```

**What the worked example means:** This calculation determines the relative humidity by comparing the saturation vapor pressure at the dewpoint to that at the air temperature. Using the Ambaum method, the ratio yields a fraction of 0.538557, which corresponds to 53.8557 when expressed as a percentage. This value indicates how close the air is to saturation given the current temperatures.

<a id="saturation_mixing_ratio"></a>

#### Saturation mixing ratio: `saturation_mixing_ratio`

The mixing ratio of air saturated with respect to liquid water at temperature T and pressure p.

$$
w_s = \frac{\varepsilon\,e_s(T)}{p-e_s(T)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | total air pressure | Pa |
| `T` | in | air temperature | K |
| `ws` | out | saturation mixing ratio (kg/kg) | - |
| `ws_g_per_kg` | out | saturation mixing ratio (g/kg) | - |

**Applies when:** e_s(T) < p.

```shell
$ earth-science solve saturation_mixing_ratio "p=100000 Pa" "T=298.15 K"
ws = 0.0203107222
ws_g_per_kg = 20.3107222
  by: Saturation mixing ratio  (w_s = epsilon e_s/(p - e_s), Ambaum (2020) e_s)
  p = 100000 Pa
  T = 298.15 K
  assumes: e_s(T) < p
```

**What the worked example means:** This calculation determines the maximum mass of water vapor that air can hold relative to the mass of dry air at a specific temperature and pressure before condensation occurs. Using an input total air pressure of 100000 Pa and an air temperature of 298.15 K, the resulting saturation mixing ratio is 0.0203107. This value represents the humidity limit for the given atmospheric conditions, indicating how much moisture the air can sustain as vapor rather than liquid.

<a id="saturation_vapor_pressure"></a>

#### Saturation vapour pressure over water: `saturation_vapor_pressure`

The equilibrium vapour pressure over a plane surface of liquid water (Ambaum 2020, latent heat varying with temperature).

$$
e_s = e_{s0}\left(\frac{T_0}{T}\right)^{(c_{pl}-c_{pv})/R_v}\exp\!\left(\frac{L_0}{R_vT_0}-\frac{L(T)}{R_vT}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | temperature | K |
| `es` | out | saturation vapour pressure (Pa) | Pa |

**Applies when:** -45 °C <= T <= 60 °C (liquid water, possibly supercooled).

```shell
$ earth-science solve saturation_vapor_pressure "T=298.15 K"
es = 3162.345599 Pa
  by: Saturation vapour pressure over water  (Ambaum (2020) Eq. 13 with L(T) = L0 - (c_pl - c_pv)(T - T0))
  T = 298.15 K
  assumes: -45 °C <= T <= 60 °C (liquid water, possibly supercooled)
```

**What the worked example means:** This calculation determines the equilibrium vapour pressure over a plane surface of liquid water, denoted as 3162.35 Pa. It relies on the input temperature, specified here as 298.15 K. The result represents the maximum partial pressure that water vapour can exert in the air when it is in thermodynamic balance with the liquid phase. This value is critical for understanding humidity levels and cloud formation processes. As the temperature increases, the saturation vapour pressure also rises, indicating that warmer air can hold more moisture before condensation occurs.

<a id="specific_humidity_from_dewpoint"></a>

#### Specific humidity from dewpoint and pressure, `specific_humidity_from_dewpoint`

Specific humidity (mass of vapour per mass of moist air) from the dewpoint and the pressure.

$$
q = \frac{w_s(p,T_d)}{1+w_s(p,T_d)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | air pressure | Pa |
| `Td` | in | dewpoint temperature | K |
| `q` | out | specific humidity (kg/kg) | - |
| `w` | out | mixing ratio (kg/kg) | - |

**Applies when:** e_s(Td) < p.

```shell
$ earth-science solve specific_humidity_from_dewpoint "p=100000 Pa" "Td=288.15 K"
q = 0.01066120332
w = 0.0107760894
  by: Specific humidity from dewpoint and pressure  (q = w_s(p, Td)/(1 + w_s(p, Td)))
  p = 100000 Pa
  Td = 288.15 K
  assumes: e_s(Td) < p
```

**What the worked example means:** This calculation determines the specific humidity, which represents the mass of water vapor per unit mass of moist air, along with the mixing ratio. It uses the provided air pressure of 100000 Pa and the dewpoint temperature of 288.15 K to derive these moisture metrics. The resulting specific humidity of 0.0106612 indicates how much water vapor is present in the air parcel relative to its total mass, a key variable for understanding atmospheric moisture content.

<a id="mixing_ratio"></a>

#### Water-vapour mixing ratio from vapour pressure, `mixing_ratio`

Mass of water vapour per mass of dry air from the vapour partial pressure and the total pressure.

$$
w = \frac{\varepsilon\,e}{p-e}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `e` | in | water vapour partial pressure | Pa |
| `p` | in | total air pressure | Pa |
| `w` | out | mixing ratio (kg/kg) | - |
| `w_g_per_kg` | out | mixing ratio (g/kg) | - |

**Applies when:** 0 < e < p.

```shell
$ earth-science solve mixing_ratio "e=2000 Pa" "p=100000 Pa"
w = 0.01269299816
w_g_per_kg = 12.69299816
  by: Water-vapour mixing ratio from vapour pressure  (w = epsilon e/(p - e))
  e = 2000 Pa
  p = 100000 Pa
  assumes: 0 < e < p
```

**What the worked example means:** This calculation determines the mass of water vapour per unit mass of dry air using the vapour partial pressure and total atmospheric pressure. With an input vapour pressure of 2000 Pa and total pressure of 100000 Pa, the resulting mixing ratio is 0.012693. This value represents the absolute amount of moisture in the air parcel and remains constant during adiabatic expansion or compression unless condensation occurs. The equivalent value in grams per kilogram is 12.693, which is often more intuitive for practical meteorological applications.

### Atmospheric thermodynamics (16)

<a id="scale_height"></a>

#### Atmospheric scale height: `scale_height`

The height over which pressure falls by a factor e in an isothermal dry atmosphere.

$$
H = \frac{R_d T}{g}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | temperature | K |
| `H` | out | scale height (m) | m |

**Applies when:** dry air, constant gravity.

```shell
$ earth-science solve scale_height "T=288.15 K"
H = 8434.351641 m
  by: Atmospheric scale height  (H = R_d T/g)
  T = 288.15 K
  assumes: dry air, constant gravity
```

**What the worked example means:** This calculation determines the characteristic vertical distance over which atmospheric pressure decreases by a factor of e, assuming dry air and constant gravity. Using the temperature input of 288.15 K, the resulting scale height is 8434.35 m. This value represents the thickness of the atmosphere layer where pressure decays exponentially with altitude.

<a id="moist_air_density"></a>

#### Density of moist air: `moist_air_density`

Density of moist air from the ideal-gas law with the virtual temperature.

$$
\rho = \frac{p}{R_d\,T_v}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | air pressure | Pa |
| `T` | in | air temperature | K |
| `w` | in | water-vapour mixing ratio (kg/kg) | - |
| `rho` | out | air density (kg/m^3) | kg/m^3 |
| `Tv` | out | virtual temperature (K) | K |

**Applies when:** 0 <= w <= 0.06.

```shell
$ earth-science solve moist_air_density "p=100000 Pa" "T=298.15 K" w=0.015
rho = 1.158051084 kg/m^3
Tv = 300.8281879 K
  by: Density of moist air  (rho = p/(R_d T_v))
  p = 100000 Pa
  T = 298.15 K
  w = 0.015
  assumes: 0 <= w <= 0.06
```

**What the worked example means:** This calculation determines the mass per unit volume of humid air, yielding a density of 1.15805 kg/m^3. The process relies on the ideal gas law adjusted by virtual temperature, which was computed as 300.828 K based on an air pressure of 100000 Pa and a water-vapour mixing ratio of 0.015. This result reflects how moisture reduces air density compared to dry conditions at the same temperature and pressure.

<a id="equivalent_potential_temperature"></a>

#### Equivalent potential temperature (Bolton 1980), `equivalent_potential_temperature`

The potential temperature a parcel would have after all its vapour condensed and the latent heat warmed it.

$$
\theta_e = T\left(\frac{p_0}{p-e}\right)^{\kappa}\left(\frac{T}{T_L}\right)^{0.28r}\exp\!\left[r(1+0.448r)\left(\frac{3036}{T_L}-1.78\right)\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | air pressure | Pa |
| `T` | in | air temperature | K |
| `Td` | in | dewpoint | K |
| `theta_e` | out | equivalent potential temperature (K) | K |

**Applies when:** Td <= T; 100 hPa <= p <= 1100 hPa.

```shell
$ earth-science solve equivalent_potential_temperature "p=85000 Pa" "T=293.15 K" "Td=288.15 K"
theta_e = 345.6759213 K
  by: Equivalent potential temperature (Bolton 1980)  (Bolton (1980): T_L from T and Td, theta_L with the dry-air partial pressure, then the exponential factor)
  p = 85000 Pa
  T = 293.15 K
  Td = 288.15 K
  assumes: Td <= T; 100 hPa <= p <= 1100 hPa
```

**What the worked example means:** This calculation determines the equivalent potential temperature, which is 345.676 K. It represents the theoretical temperature an air parcel would reach if all its water vapor condensed and the released latent heat warmed the parcel. This value helps meteorologists assess atmospheric instability by comparing it to the environment's equivalent potential temperature.

<a id="geopotential_height"></a>

#### Geopotential and geopotential height: `geopotential_height`

Geopotential and geopotential height of a geometric height above sea level, with gravity falling off as 1/(R_e + z)^2 (as MetPy).

$$
\Phi = \frac{g\,R_e\,z}{R_e+z},\quad Z = \Phi/g
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `z` | in | geometric height above sea level | m |
| `phi` | out | geopotential (m^2/s^2) | m^2/s^2 |
| `Z` | out | geopotential height (m) | m |

**Applies when:** -1 km <= z <= 200 km.

```shell
$ earth-science solve geopotential_height "z=10000 m"
phi = 97912.81505 m^2/s^2
Z = 9984.328497 m
  by: Geopotential and geopotential height  (Phi = g R_e z/(R_e + z))
  z = 10000 m
  assumes: -1 km <= z <= 200 km
```

**What the worked example means:** This calculation converts the input geometric height, 10000 m, into geopotential energy per unit mass, 97912.8 m^2/s^2, and its corresponding geopotential height, 9984.33 m. The process accounts for the decrease in gravitational acceleration as altitude increases above Earth's surface. The resulting geopotential height is the vertical coordinate typically used on weather maps because it represents the work required to lift a unit mass from sea level to that elevation against gravity.

<a id="standard_atmosphere_height"></a>

#### Height of a pressure level in the standard atmosphere, `standard_atmosphere_height`

The height of a pressure level in the tropospheric International Standard Atmosphere (pressure altitude; T0 = 288.15 K, 6.5 K/km, 1013.25 hPa).

$$
z = \frac{T_0}{\gamma}\left[1-\left(\frac{p}{p_0}\right)^{R_d\gamma/g}\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | pressure | Pa |
| `z` | out | pressure altitude (m) | m |

**Applies when:** 226 hPa <= p <= 1075 hPa.

```shell
$ earth-science solve standard_atmosphere_height "p=50000 Pa"
z = 5574.336212 m
  by: Height of a pressure level in the standard atmosphere  (z = (T0/gamma)(1 - (p/p0)^(R_d gamma/g)))
  p = 50000 Pa
  assumes: 226 hPa <= p <= 1075 hPa
```

**What the worked example means:** This calculation determines the geometric height within the troposphere where the atmospheric pressure equals the input value. Using the International Standard Atmosphere model, which assumes a linear temperature decrease with height and constant gravity, the method converts the pressure reading into an altitude estimate. The result indicates that at a pressure of 50000 Pa, the corresponding standard atmosphere height is 5574.34 m, providing a reference level for flight operations or meteorological analysis.

<a id="lifting_condensation_level"></a>

#### Lifting condensation level (Romps 2017): `lifting_condensation_level`

Pressure and temperature at which a parcel lifted dry-adiabatically from (p, T, Td) becomes saturated, from Romps' exact solution with the Lambert W function.

$$
T_{LCL} = \frac{c}{W_{-1}\!\left(RH^{1/a}\,c\,e^{c}\right)}\,T,\quad p_{LCL} = p\left(\frac{T_{LCL}}{T}\right)^{c_{pm}/R_m}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | starting pressure | Pa |
| `T` | in | starting temperature | K |
| `Td` | in | starting dewpoint | K |
| `p_lcl` | out | LCL pressure (Pa) | Pa |
| `T_lcl` | out | LCL temperature (K) | K |

**Applies when:** Td <= T; 100 hPa <= p <= 1100 hPa.

```shell
$ earth-science solve lifting_condensation_level "p=100000 Pa" "T=303.15 K" "Td=293.15 K"
p_lcl = 86421.30262 Pa
T_lcl = 290.8124964 K
  by: Lifting condensation level (Romps 2017)  (Romps (2017) Eq. 22: T_LCL = c/W_{-1}(RH^{1/a} c e^c) T, p_LCL = p (T_LCL/T)^{c_pm/R_m})
  p = 100000 Pa
  T = 303.15 K
  Td = 293.15 K
  assumes: Td <= T; 100 hPa <= p <= 1100 hPa
```

**What the worked example means:** This calculation determines the exact point where an unsaturated air parcel becomes saturated during dry-adiabatic ascent. Starting from a pressure of 100000 Pa, a temperature of 303.15 K, and a dewpoint of 293.15 K, the model computes the resulting LCL pressure as 86421.3 Pa and the LCL temperature as 290.812 K. These values represent the altitude conditions necessary for cloud base formation.

<a id="moist_static_energy"></a>

#### Moist static energy: `moist_static_energy`

Moist static energy c_pd T + g z + L_v q (and the dry static energy c_pd T + g z) per unit mass.

$$
h = c_{pd}T + g z + L_v q
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `z` | in | height above sea level | m |
| `T` | in | air temperature | K |
| `q` | in | specific humidity (kg/kg) | - |
| `h` | out | moist static energy (J/kg) | J/kg |
| `s` | out | dry static energy (J/kg) | J/kg |

**Applies when:** 0 <= q <= 0.06.

```shell
$ earth-science solve moist_static_energy "z=1500 m" "T=288.15 K" q=0.008
h = 324211.2658 J/kg
s = 304204.5458 J/kg
  by: Moist static energy  (h = c_pd T + g z + L_v q (MetPy's L_v = 2.50084e6 J/kg))
  z = 1500 m
  T = 288.15 K
  q = 0.008
  assumes: 0 <= q <= 0.06
```

**What the worked example means:** This calculation determines the total energy per unit mass of an air parcel by combining its thermal energy, gravitational potential energy, and latent heat content. Using an input height of 1500 m, temperature of 288.15 K, and specific humidity of 0.008, the resulting moist static energy is 324211 J/kg, while the dry static energy, which excludes moisture effects, is 304205 J/kg. These values indicate the parcel's capacity for vertical motion and stability under adiabatic conditions.

<a id="potential_temperature"></a>

#### Potential temperature: `potential_temperature`

The temperature an air parcel would have if brought dry-adiabatically to 1000 hPa (Poisson's equation).

$$
\theta = T\left(\frac{p_0}{p}\right)^{R_d/c_{pd}},\quad p_0 = 1000\ \mathrm{hPa}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | air temperature | K |
| `p` | in | air pressure | Pa |
| `theta` | out | potential temperature (K) | K |

**Applies when:** 150 K <= T <= 350 K; 1 hPa <= p <= 1100 hPa.

```shell
$ earth-science solve potential_temperature "T=298.15 K" "p=85000 Pa"
theta = 312.3207471 K
  by: Potential temperature  (Poisson's equation with kappa = R_d/c_pd)
  T = 298.15 K
  p = 85000 Pa
  assumes: 150 K <= T <= 350 K; 1 hPa <= p <= 1100 hPa
```

**What the worked example means:** This calculation determines the potential temperature, which represents the temperature an air parcel would possess if it were moved dry-adiabatically to a reference pressure level. By using an input air temperature of 298.15 K and an input air pressure of 85000 Pa, the method applies Poisson's equation to remove the effects of expansion or compression caused by pressure changes. The resulting value of 312.321 K allows meteorologists to compare the thermal state of air parcels at different altitudes on a consistent basis.

<a id="standard_atmosphere_pressure"></a>

#### Pressure at a height in the standard atmosphere, `standard_atmosphere_pressure`

Pressure at a height in the tropospheric International Standard Atmosphere (T0 = 288.15 K, 6.5 K/km, 1013.25 hPa).

$$
p = p_0\left(1-\frac{\gamma z}{T_0}\right)^{g/(R_d\gamma)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `z` | in | height above sea level | m |
| `p` | out | standard-atmosphere pressure (Pa) | Pa |

**Applies when:** -500 m <= z <= 11 km (troposphere).

```shell
$ earth-science solve standard_atmosphere_pressure "z=1500 m"
p = 84555.70741 Pa
  by: Pressure at a height in the standard atmosphere  (p = p0 (1 - gamma z/T0)^(g/(R_d gamma)))
  z = 1500 m
  assumes: -500 m <= z <= 11 km (troposphere)
```

**What the worked example means:** This calculation determines the atmospheric pressure at a specific altitude within the troposphere using the standard atmosphere model. Given a height of 1500 m, the resulting pressure is 84555.7 Pa. This value represents the idealized pressure expected under standard conditions, where temperature decreases linearly with height and air behaves as an ideal gas in hydrostatic equilibrium.

<a id="isothermal_pressure"></a>

#### Pressure at height in an isothermal atmosphere, `isothermal_pressure`

The barometric formula: pressure at height z in an isothermal dry atmosphere, and its scale height R_d T/g.

$$
p = p_0\,e^{-z/H},\quad H = \frac{R_d T}{g}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p0` | in | pressure at z = 0 | Pa |
| `T` | in | temperature of the layer | K |
| `z` | in | height above the reference level | m |
| `p` | out | pressure at height z (Pa) | Pa |
| `H` | out | scale height (m) | m |

**Applies when:** isothermal dry air; constant gravity g = 9.80665 m/s^2.

```shell
$ earth-science solve isothermal_pressure "p0=101325 Pa" "T=273.15 K" "z=5000 m"
p = 54215.40421 Pa
H = 7995.291171 m
  by: Pressure at height in an isothermal atmosphere  (p = p0 exp(-z/H), H = R_d T/g)
  p0 = 101325 Pa
  T = 273.15 K
  z = 5000 m
  assumes: isothermal dry air; constant gravity g = 9.80665 m/s^2
```

**What the worked example means:** This calculation determines the atmospheric pressure at a specific elevation by applying the barometric formula for an isothermal dry atmosphere. Using the surface pressure 101325 Pa and temperature 273.15 K, the model computes a scale height of 7995.29 m. Consequently, the pressure at the requested height of 5000 m is found to be 54215.4 Pa. This result reflects how air density and pressure decrease exponentially with altitude under constant thermal conditions.

<a id="pressure_with_lapse_rate"></a>

#### Pressure at height with a constant lapse rate, `pressure_with_lapse_rate`

Pressure at height z in a dry atmosphere whose temperature falls linearly with height (troposphere model).

$$
p = p_0\left(\frac{T_0-\gamma z}{T_0}\right)^{g/(R_d\gamma)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p0` | in | pressure at z = 0 | Pa |
| `T0` | in | temperature at z = 0 | K |
| `gamma` | in | temperature lapse rate (positive when temperature falls with height) | K/m |
| `z` | in | height above the reference level | m |
| `p` | out | pressure at height z (Pa) | Pa |
| `T` | out | temperature at height z (K) | K |

**Applies when:** 0 < gamma <= 9.8 K/km; dry air, constant gravity.

```shell
$ earth-science solve pressure_with_lapse_rate "p0=101325 Pa" "T0=288.15 K" "gamma=0.0065 K/m" "z=3000 m"
p = 70108.04265 Pa
T = 268.65 K
  by: Pressure at height with a constant lapse rate  (p = p0 (T/T0)^(g/(R_d gamma)), T = T0 - gamma z)
  p0 = 101325 Pa
  T0 = 288.15 K
  gamma = 0.0065 K/m
  z = 3000 m
  assumes: 0 < gamma <= 9.8 K/km; dry air, constant gravity
```

**What the worked example means:** This calculation determines the atmospheric pressure and temperature at a specific altitude using a constant lapse rate model. Given a surface pressure of 101325 Pa and a surface temperature of 288.15 K, with a lapse rate of 0.0065 K/m over a height of 3000 m, the resulting pressure is 70108 Pa and the temperature is 268.65 K. This reflects how pressure decreases exponentially while temperature drops linearly in a dry troposphere.

<a id="moist_adiabatic_lapse_rate"></a>

#### Saturated (moist) adiabatic lapse rate: `moist_adiabatic_lapse_rate`

The rate at which a saturated parcel cools as it rises, with condensation releasing latent heat.

$$
\Gamma_m = g\,\frac{1+\dfrac{L_v r_s}{R_d T}}{c_{pd}+\dfrac{L_v^2 r_s}{R_v T^2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | air temperature | K |
| `p` | in | air pressure | Pa |
| `lapse_rate` | out | moist adiabatic lapse rate (K/m) | K/m |
| `rs` | out | saturation mixing ratio (kg/kg) | - |

**Applies when:** -40 °C <= T <= 50 °C; constant latent heat L_v = 2.50084e6 J/kg in the lapse-rate formula.

```shell
$ earth-science solve moist_adiabatic_lapse_rate "T=293.15 K" "p=100000 Pa"
lapse_rate = 0.004221886202 K/m
rs = 0.0148682638
  by: Saturated (moist) adiabatic lapse rate  (Gamma_m = g (1 + L_v r_s/(R_d T)) / (c_pd + L_v^2 r_s/(R_v T^2)), r_s from Ambaum (2020) e_s)
  T = 293.15 K
  p = 100000 Pa
  assumes: -40 °C <= T <= 50 °C; constant latent heat L_v = 2.50084e6 J/kg in the lapse-rate formula
```

**What the worked example means:** This calculation determines the moist adiabatic lapse rate, which describes how quickly a saturated air parcel cools as it rises through the atmosphere. Using an input temperature of 293.15 K and pressure of 100000 Pa, the model computes the saturation mixing ratio, resulting in a value of 0.0148683. This moisture content influences the release of latent heat during condensation, which partially offsets the cooling caused by expansion. The final computed lapse rate is 0.00422189 K/m K/m. Unlike the dry adiabatic lapse rate, this value varies significantly with atmospheric conditions because warmer air holds more water vapor, leading to greater latent heat release and slower cooling rates.

<a id="temperature_from_potential_temperature"></a>

#### Temperature from potential temperature: `temperature_from_potential_temperature`

The air temperature at pressure p of a parcel with a given potential temperature.

$$
T = \theta\left(\frac{p}{p_0}\right)^{R_d/c_{pd}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `theta` | in | potential temperature | K |
| `p` | in | air pressure | Pa |
| `T` | out | air temperature (K) | K |

**Applies when:** 150 K <= theta <= 600 K; 1 hPa <= p <= 1100 hPa.

```shell
$ earth-science solve temperature_from_potential_temperature "theta=300 K" "p=70000 Pa"
T = 270.9340268 K
  by: Temperature from potential temperature  (Poisson's equation inverted)
  theta = 300 K
  p = 70000 Pa
  assumes: 150 K <= theta <= 600 K; 1 hPa <= p <= 1100 hPa
```

**What the worked example means:** This calculation determines the actual air temperature, resulting in 270.934 K, for an air parcel with a potential temperature of 300 K at a pressure of 70000 Pa. It applies Poisson's equation to find what the observed temperature should be based on the thermodynamic state of the dry air mass.

<a id="dry_adiabatic_ascent"></a>

#### Temperature of a dry parcel after adiabatic ascent, `dry_adiabatic_ascent`

The temperature of an unsaturated air parcel lifted dry-adiabatically through a height dz, and the dry adiabatic lapse rate g/c_pd.

$$
T_2 = T - \Gamma_d\,\Delta z,\quad \Gamma_d = g/c_{pd}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | starting temperature of the parcel | K |
| `dz` | in | height through which the parcel rises | m |
| `T2` | out | parcel temperature after the ascent (K) | K |
| `lapse_rate` | out | dry adiabatic lapse rate g/c_pd (K/m) | K/m |

**Applies when:** 0 < dz <= 20 km; the parcel stays unsaturated.

```shell
$ earth-science solve dry_adiabatic_ascent "T=293.15 K" "dz=1500 m"
T2 = 278.5083462 K
lapse_rate = 0.009761102563 K/m
  by: Temperature of a dry parcel after adiabatic ascent  (dry adiabatic lapse rate g/c_pd times the height gained)
  T = 293.15 K
  dz = 1500 m
  assumes: 0 < dz <= 20 km; the parcel stays unsaturated
```

**What the worked example means:** This calculation determines the final temperature of an unsaturated air parcel that has risen vertically by a specific height. Starting from an initial temperature of 293.15 K, the parcel cools as it ascends through a distance of 1500 m. The resulting temperature is 278.508 K. This process assumes no heat exchange with the surroundings and uses the standard dry adiabatic lapse rate, which is calculated as 0.0097611 K/m. This rate represents how much the temperature drops for every meter of elevation gained under these ideal conditions.

<a id="hypsometric_thickness"></a>

#### Thickness of a layer (hypsometric equation), `hypsometric_thickness`

The geometric thickness between two pressure levels from the layer-mean virtual temperature.

$$
\Delta z = \frac{R_d\overline{T_v}}{g}\ln\frac{p_1}{p_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p1` | in | pressure at the bottom of the layer | Pa |
| `p2` | in | pressure at the top of the layer | Pa |
| `Tv` | in | layer-mean virtual temperature | K |
| `dz` | out | layer thickness (m) | m |

**Applies when:** p1 > p2.

```shell
$ earth-science solve hypsometric_thickness "p1=100000 Pa" "p2=50000 Pa" "Tv=263.15 K"
dz = 5339.024515 m
  by: Thickness of a layer (hypsometric equation)  (dz = (R_d Tv/g) ln(p1/p2))
  p1 = 100000 Pa
  p2 = 50000 Pa
  Tv = 263.15 K
  assumes: p1 > p2
```

**What the worked example means:** This calculation determines the geometric thickness between two atmospheric pressure levels using the hypsometric equation. By inputting the bottom pressure, top pressure, and the layer-mean virtual temperature, the model computes the vertical distance in meters. The result of 5339.02 m represents the physical height span of the air column where the pressure drops from the lower to the upper level, accounting for moisture effects through virtual temperature.

<a id="virtual_temperature"></a>

#### Virtual temperature: `virtual_temperature`

The temperature dry air would need to have the density of the moist air at the same pressure.

$$
T_v = T\,\frac{w+\varepsilon}{\varepsilon(1+w)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | air temperature | K |
| `w` | in | water-vapour mixing ratio (kg/kg) | - |
| `Tv` | out | virtual temperature (K) | K |

**Applies when:** 0 <= w <= 0.06.

```shell
$ earth-science solve virtual_temperature "T=303.15 K" w=0.02
Tv = 306.7630037 K
  by: Virtual temperature  (Tv = T (w + epsilon)/(epsilon (1 + w)))
  T = 303.15 K
  w = 0.02
  assumes: 0 <= w <= 0.06
```

**What the worked example means:** This calculation determines the virtual temperature, defined as the temperature that dry air would require to possess the same density as the current moist air parcel under identical pressure conditions. By using the input air temperature of 303.15 K and a water-vapour mixing ratio of 0.02, the model applies the relationship between moisture content and density. The resulting virtual temperature of 306.763 K allows practitioners to use the ideal gas law for moist air without altering the fundamental density calculation method, effectively treating the moist air as if it were dry air with this adjusted temperature.

### Climate (1)

<a id="energy_balance_response"></a>

#### Warming of a one-box energy-balance model: `energy_balance_response`

Temperature response after t years to a constant radiative forcing F in a one-box model C dT/dt = F - T/lambda, the equilibrium warming lambda F and the e-folding time lambda C.

$$
\Delta T(t) = \lambda F\left(1-e^{-t/\tau}\right),\quad \tau = \lambda C
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F` | in | radiative forcing applied at t = 0 | W/m^2 |
| `lam` | in | climate sensitivity parameter | K*m^2/W |
| `C` | in | effective heat capacity per unit area | J/(m^2*K) |
| `t` | in | time since the forcing began (years) | - |
| `dT` | out | warming after t (K) | K |
| `dT_eq` | out | equilibrium warming (K) | K |
| `tau_years` | out | e-folding time (years) | - |

**Applies when:** lambda > 0, C > 0; the forcing is switched on at t = 0 and then held constant.

```shell
$ earth-science solve energy_balance_response "F=3.7 W/m^2" "lam=0.8 K*m^2/W" "C=420000000 J/(m^2*K)" t=10
dT = 1.802832762 K
dT_eq = 2.96 K
tau_years = 10.64719751
  by: Warming of a one-box energy-balance model  (dT(t) = lambda F (1 - exp(-t/(lambda C))))
  F = 3.7 W/m^2
  lam = 0.8 K*m^2/W
  C = 420000000 J/(m^2*K)
  t = 10
  assumes: lambda > 0, C > 0; the forcing is switched on at t = 0 and then held constant
```

**What the worked example means:** This calculation determines the temperature response to a constant radiative forcing of 3.7 W/m^2 applied for 10. Using a climate sensitivity parameter of 0.8 K*m^2/W and heat capacity of 4.2e+08 J/(m^2*K), the model computes a current warming of 1.80283 K. The system tends toward an equilibrium warming of 2.96 K, governed by an e-folding time of 10.6472, which represents the delay caused by thermal inertia.

### Climate and radiation (2)

<a id="co2_radiative_forcing"></a>

#### Radiative forcing of a change in CO2 (Myhre et al. 1998), `co2_radiative_forcing`

The radiative forcing dF = 5.35 ln(C/C0) W/m^2 of a change in CO2 concentration from C0 to C, and with a climate sensitivity parameter the equilibrium warming lambda dF.

$$
\Delta F = 5.35\,\ln\frac{C}{C_0}\ \mathrm{W\,m^{-2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C` | in | CO2 concentration (ppm) | - |
| `C0` | in | reference CO2 concentration (ppm) | - |
| `lam` | in | climate sensitivity parameter (K per W/m^2, optional) (optional) | K*m^2/W |
| `dF` | out | radiative forcing (W/m^2) | W/m^2 |
| `doublings` | out | number of CO2 doublings log2(C/C0) | - |
| `dT_eq` | out | equilibrium warming lambda dF (K) | K |

**Applies when:** 1 <= C, C0 <= 100,000 ppm; the simplified logarithmic expression of Myhre et al. (1998).

```shell
$ earth-science solve co2_radiative_forcing C=560 C0=280
dF = 3.708337416 W/m^2
doublings = 1
  by: Radiative forcing of a change in CO2 (Myhre et al. 1998)  (dF = 5.35 ln(C/C0) W/m^2; dT_eq = lambda dF)
  C = 560
  C0 = 280
  assumes: 1 <= C, C0 <= 100,000 ppm; the simplified logarithmic expression of Myhre et al. (1998)
```

In the worked example, C = 560, C0 = 280 go through the equation above and give dF = 3.708337416 W/m^2, doublings = 1. It applies only when 1 <= C, C0 <= 100,000 ppm; the simplified logarithmic expression of Myhre et al. (1998): outside that it says what is wrong instead of returning a number.

<a id="solar_elevation_angle"></a>

#### Solar elevation and azimuth: `solar_elevation_angle`

The sun's elevation above the horizon, its zenith angle and azimuth (clockwise from north) for a latitude, a solar declination and an hour angle (15 degrees per hour from solar noon, negative in the morning).

$$
\sin\alpha = \sin\varphi\sin\delta + \cos\varphi\cos\delta\cos H
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lat` | in | latitude (degrees, north positive) | - |
| `decl` | in | solar declination (degrees) | - |
| `H` | in | hour angle (degrees, negative before solar noon) | - |
| `elevation` | out | solar elevation angle (degrees) | - |
| `zenith` | out | solar zenith angle (degrees) | - |
| `azimuth` | out | solar azimuth, clockwise from north (degrees) | - |

**Applies when:** |declination| <= 23.5 degrees; -180 <= hour angle <= 180 degrees; geometric (no refraction).

```shell
$ earth-science solve solar_elevation_angle lat=40 decl=23.44 H=0
elevation = 73.44
zenith = 16.56
azimuth = 180
  by: Solar elevation and azimuth  (sin(elevation) = sin(phi) sin(delta) + cos(phi) cos(delta) cos(H); azimuth from the east and north components)
  lat = 40
  decl = 23.44
  H = 0
  assumes: |declination| <= 23.5 degrees; -180 <= hour angle <= 180 degrees; geometric (no refraction)
```

**What the worked example means:** This calculation determines the sun's position in the sky based on your latitude, the solar declination, and the hour angle. The resulting elevation of 73.44 indicates how high the sun sits above the horizon, while the zenith angle of 16.56 represents the distance from directly overhead. The azimuth value of 180 specifies the compass direction where the sun is located, rotating clockwise from true north.

### Geochronology (5)

<a id="radiocarbon_age"></a>

#### Conventional radiocarbon age: `radiocarbon_age`

Conventional radiocarbon age (years BP) from the fraction modern carbon, with the Libby mean life of 8033 years (Stuiver and Polach 1977); not calibrated to calendar years.

$$
t = -8033\,\ln F
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F` | in | fraction modern (14C activity relative to the modern standard; or % in a question) | - |
| `age` | out | conventional radiocarbon age (years BP) | - |

**Applies when:** 1e-5 <= F <= 1 (no bomb carbon).

```shell
$ earth-science solve radiocarbon_age F=0.5
age = 5568.051301
  by: Conventional radiocarbon age  (t = -8033 ln F)
  F = 0.5
  assumes: 1e-5 <= F <= 1 (no bomb carbon)
```

**What the worked example means:** This calculation determines the conventional radiocarbon age of a sample, expressed as 5568.05, by applying the Libby mean life convention to the fraction modern carbon value of 0.5. The result represents the time elapsed before present based on radioactive decay, assuming a closed system and constant initial atmospheric carbon ratios. This uncalibrated age serves as a standard reference point for archaeology and geology but requires further adjustment to match calendar years.

<a id="radioactive_decay_remaining"></a>

#### Fraction of a radioactive parent remaining, `radioactive_decay_remaining`

Fraction of the parent nuclide left (and decayed) after a time t, and the decay constant ln 2/half-life.

$$
\frac{N}{N_0} = 2^{-t/t_{1/2}} = e^{-\lambda t},\quad \lambda = \frac{\ln 2}{t_{1/2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `half_life` | in | half-life (years) | - |
| `t` | in | elapsed time (years) | - |
| `fraction` | out | fraction of the parent remaining | - |
| `decayed` | out | fraction decayed | - |
| `lambda` | out | decay constant (1/yr) | - |

**Applies when:** half-life > 0; t <= 60 half-lives.

```shell
$ earth-science solve radioactive_decay_remaining half_life=5730 t=10000
fraction = 0.2982924364
decayed = 0.7017075636
lambda = 0.0001209680943
  by: Fraction of a radioactive parent remaining  (N/N0 = 2^(-t/t_half), lambda = ln 2/t_half)
  half_life = 5730
  t = 10000
  assumes: half-life > 0; t <= 60 half-lives
```

**What the worked example means:** This calculation determines the proportion of a radioactive parent isotope that remains after a specified elapsed time. Using a half-life of 5730 years and an elapsed time of 10000 years, the model computes that 0.298292 of the original atoms are still present. Consequently, the fraction that has decayed is 0.701708. The decay constant, which represents the probability of decay per unit time, is calculated as 0.000120968 inverse years.

<a id="isochron_age"></a>

#### Isochron age (e.g. Rb-Sr) from sample ratios, `isochron_age`

Age and initial ratio from an isochron: a least-squares line through (parent/stable, daughter/stable) ratios of cogenetic samples (ordinary least squares, not a York fit). Default decay constant 1.42e-11 /yr (87Rb).

$$
t = \frac{\ln(1+m)}{\lambda},\quad \frac{D}{S} = \left(\frac{D}{S}\right)_0 + m\,\frac{P}{S}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | parent/stable isotope ratios (list, e.g. 87Rb/86Sr) | - |
| `y` | in | daughter/stable isotope ratios (list, e.g. 87Sr/86Sr) | - |
| `lam` | in | decay constant of the parent (default 1.42e-11 /yr) (optional) | - |
| `age` | out | isochron age (years) | - |
| `slope` | out | isochron slope e^(lambda t) - 1 | - |
| `initial_ratio` | out | initial daughter/stable ratio (intercept) | - |

**Applies when:** cogenetic samples, closed systems, same initial ratio.

```shell
$ earth-science solve isochron_age "x=[0.5, 1.2, 2.8, 4.1]" "y=[0.7121, 0.7221, 0.7449, 0.7634]" lam=1.42e-11
age = 996371909.4
slope = 0.01424904459
initial_ratio = 0.7049895541
  by: Isochron age (e.g. Rb-Sr) from sample ratios  (t = ln(1 + slope)/lambda)
  lam = 1.42e-11
  assumes: cogenetic samples, closed systems, same initial ratio
```

**What the worked example means:** This calculation determines the age of a rock suite by fitting a line to parent and daughter isotope ratios. The resulting slope of 0.014249 yields an age of 9.96372e+08, while the intercept provides the initial ratio of 0.70499. These values assume that all samples formed simultaneously and remained closed systems, allowing the decay constant to convert the measured isotopic spread into a specific time duration.

<a id="potassium_argon_age"></a>

#### Potassium-argon age (branched decay): `potassium_argon_age`

K-Ar age from the radiogenic 40Ar*/40K ratio, allowing for the branching of 40K decay to 40Ar (electron capture) and 40Ca. Defaults: lambda = 5.543e-10 /yr, lambda_e = 0.581e-10 /yr (Steiger and Jäger 1977).

$$
t = \frac{1}{\lambda}\ln\!\left(1+\frac{\lambda}{\lambda_e}\frac{^{40}\mathrm{Ar}^*}{^{40}\mathrm{K}}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `ratio` | in | radiogenic 40Ar*/40K atom ratio | - |
| `lam` | in | total decay constant of 40K (default 5.543e-10 /yr) (optional) | - |
| `lam_e` | in | partial decay constant to 40Ar (default 0.581e-10 /yr) (optional) | - |
| `age` | out | K-Ar age (years) | - |

**Applies when:** closed system, no initial argon.

```shell
$ earth-science solve potassium_argon_age ratio=0.0055
age = 92264335.51
  by: Potassium-argon age (branched decay)  (t = (1/lambda) ln(1 + (lambda/lambda_e) Ar*/K))
  ratio = 0.0055
  assumes: closed system, no initial argon
```

**What the worked example means:** This calculation determines the time since mineral formation using the potassium-argon method, specifically adjusting for the branching decay where potassium-40 transforms into either argon-40 or calcium-40. By analyzing the measured ratio of radiogenic argon-40 to remaining potassium-40, which is 0.0055, and applying the total and partial decay constants, the model computes an age of 9.22643e+07. This result represents the duration for which the system has remained closed, allowing argon to accumulate without loss.

<a id="radiometric_age"></a>

#### Radiometric age from a daughter/parent ratio, `radiometric_age`

Age of a closed system from the ratio of radiogenic daughter to remaining parent atoms and the half-life (no initial daughter).

$$
t = \frac{1}{\lambda}\ln\!\left(1+\frac{D^*}{P}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `ratio` | in | radiogenic daughter / parent atom ratio | - |
| `half_life` | in | half-life of the parent (years) | - |
| `age` | out | age (years) | - |
| `lambda` | out | decay constant (1/yr) | - |

**Applies when:** closed system; no daughter at the start.

```shell
$ earth-science solve radiometric_age ratio=0.5 half_life=1250000000
age = 731203125.9
lambda = 5.545177444e-10
  by: Radiometric age from a daughter/parent ratio  (t = ln(1 + D/P)/lambda)
  ratio = 0.5
  half_life = 1250000000
  assumes: closed system; no daughter at the start
```

**What the worked example means:** This calculation determines the time elapsed since a mineral formed by measuring how much radiogenic daughter product has accumulated relative to the remaining parent atoms. Using an input ratio of 0.5 and a known half-life of 1.25e+09, the model computes a decay constant of 5.54518e-10 and solves for the age, resulting in 7.31203e+08. This result indicates the duration over which radioactive decay has proceeded within a closed system.

### Geodynamics (2)

<a id="pratt_isostasy"></a>

#### Column density by Pratt isostasy: `pratt_isostasy`

The density of a crustal column standing h above sea level under Pratt isostasy (columns of different density down to a common compensation depth D), given the density of a column whose top is at sea level.

$$
\rho = \rho_0\,\frac{D}{D + h}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h` | in | elevation of the column above sea level | m |
| `rho0` | in | density of a column whose top is at sea level | kg/m^3 |
| `D` | in | depth of compensation below sea level | m |
| `rho` | out | density of the elevated column (kg/m^3) | kg/m^3 |
| `drho` | out | density deficit rho0 - rho (kg/m^3) | kg/m^3 |

**Applies when:** h > 0; rho0, D > 0; a column on land (no water load).

```shell
$ earth-science solve pratt_isostasy "h=3000 m" "rho0=2800 kg/m^3" "D=100000 m"
rho = 2718.446602 kg/m^3
drho = 81.55339806 kg/m^3
  by: Column density by Pratt isostasy  (rho = rho0 D/(D + h))
  h = 3000 m
  rho0 = 2800 kg/m^3
  D = 100000 m
  assumes: h > 0; rho0, D > 0; a column on land (no water load)
```

**What the worked example means:** This calculation finds the average density of a crustal column standing above sea level under Pratt isostasy. Using an elevation of 3000 m, a reference density of 2800 kg/m^3, and a compensation depth of 100000 m, the resulting column density is 2718.45 kg/m^3. The difference between the reference and calculated density, known as the density deficit, is 81.5534 kg/m^3. This model assumes that higher elevations are supported by lower-density material extending to a common base rather than by thickened roots.

<a id="airy_isostasy"></a>

#### Mountain root by Airy isostasy: `airy_isostasy`

The crustal root below topography of height h compensated by Airy isostasy (crust of uniform density floating on denser mantle), and with a normal crustal thickness the total thickness under the mountain.

$$
r = \frac{\rho_c}{\rho_m - \rho_c}\,h
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h` | in | elevation of the topography above sea level | m |
| `rho_c` | in | crustal density | kg/m^3 |
| `rho_m` | in | mantle density | kg/m^3 |
| `tc` | in | normal crustal thickness (optional) (optional) | m |
| `root` | out | root depth below the normal crust (m) | m |
| `root_km` | out | root depth (km) | - |
| `crust_thickness` | out | total crustal thickness under the topography (m) | m |

**Applies when:** h > 0; rho_m > rho_c; local (Airy) compensation, uniform crustal density.

```shell
$ earth-science solve airy_isostasy "h=3000 m" "rho_c=2800 kg/m^3" "rho_m=3300 kg/m^3"
root = 16800 m
root_km = 16.8
  by: Mountain root by Airy isostasy  (r = h rho_c/(rho_m - rho_c))
  h = 3000 m
  rho_c = 2800 kg/m^3
  rho_m = 3300 kg/m^3
  assumes: h > 0; rho_m > rho_c; local (Airy) compensation, uniform crustal density
```

In the worked example, h = 3000 m, rho_c = 2800 kg/m^3, rho_m = 3300 kg/m^3 go through the equation above and give root = 16800 m, root_km = 16.8. It applies only when h > 0; rho_m > rho_c; local (Airy) compensation, uniform crustal density: outside that it says what is wrong instead of returning a number.

### Geomagnetism (2)

<a id="dipole_inclination"></a>

#### Inclination and strength of a geocentric dipole field, `dipole_inclination`

Magnetic inclination (dip) and field strength relative to the equator at a magnetic latitude for a geocentric axial dipole.

$$
\tan I = 2\tan\lambda,\quad \frac{F}{F_{eq}} = \sqrt{1+3\sin^2\lambda}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lat` | in | magnetic latitude (degrees) | - |
| `inclination` | out | inclination (degrees, positive downward) | - |
| `F_ratio` | out | field strength / equatorial strength | - |

**Applies when:** geocentric axial dipole.

```shell
$ earth-science solve dipole_inclination lat=45
inclination = 63.43494882
F_ratio = 1.58113883
  by: Inclination and strength of a geocentric dipole field  (tan I = 2 tan(latitude), F/F_eq = sqrt(1 + 3 sin^2 latitude))
  lat = 45
  assumes: geocentric axial dipole
```

**What the worked example means:** This calculation determines the magnetic inclination and relative field strength for a geocentric axial dipole at the specified magnetic latitude. Using the input latitude of 45, the model computes an inclination of 63.4349 degrees, where positive values indicate downward direction. It also calculates the ratio of the local field strength to the equatorial field strength as 1.58114. These results illustrate how the Earth's magnetic field geometry changes with position, showing that both the dip angle and the intensity increase as one moves away from the magnetic equator toward the poles.

<a id="paleolatitude"></a>

#### Paleolatitude from magnetic inclination: `paleolatitude`

The magnetic (paleo)latitude at which a rock acquired its magnetisation, from the inclination of its remanence, for a geocentric axial dipole; and the angular distance to the paleopole.

$$
\lambda = \arctan\!\left(\tfrac12\tan I\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I_deg` | in | inclination of the remanent magnetisation (degrees) | - |
| `paleolatitude` | out | paleolatitude (degrees) | - |
| `pole_distance` | out | distance to the pole (degrees) | - |

**Applies when:** geocentric axial dipole; |I| < 90.

```shell
$ earth-science solve paleolatitude I_deg=60
paleolatitude = 40.89339465
pole_distance = 49.10660535
  by: Paleolatitude from magnetic inclination  (latitude = atan(tan I/2))
  I_deg = 60
  assumes: geocentric axial dipole; |I| < 90
```

**What the worked example means:** This calculation determines the magnetic latitude at which a rock acquired its magnetization, using the inclination of its remanence under the geocentric axial dipole assumption. The resulting paleolatitude of 40.8934 indicates the angular distance from the equator, while the pole distance of 49.1066 represents how far the site was from the magnetic pole.

### Geomorphology (1)

<a id="stream_power"></a>

#### Stream power of a river: `stream_power`

The total stream power per unit channel length Omega = rho g Q S and, with the channel width, the unit stream power omega = Omega/w.

$$
\Omega = \rho g Q S,\quad \omega = \frac{\Omega}{w}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | discharge | m^3/s |
| `S` | in | channel slope (m/m) | - |
| `w` | in | channel width (optional) (optional) | m |
| `rho` | in | water density (default 1000 kg/m^3) (optional) | kg/m^3 |
| `Omega` | out | total stream power per unit length (W/m) | W/m |
| `omega` | out | unit stream power (W/m^2) | W/m^2 |

**Applies when:** Q > 0; 0 < S <= 1; steady uniform flow.

```shell
$ earth-science solve stream_power "Q=100 m^3/s" S=0.002
Omega = 1961.33 W/m
  by: Stream power of a river  (Omega = rho g Q S, omega = Omega/w)
  Q = 100 m^3/s
  S = 0.002
  assumes: Q > 0; 0 < S <= 1; steady uniform flow
```

In the worked example, Q = 100 m^3/s, S = 0.002 go through the equation above and give Omega = 1961.33 W/m. It applies only when Q > 0; 0 < S <= 1; steady uniform flow: outside that it says what is wrong instead of returning a number.

### Geophysics (3)

<a id="gravity_at_latitude"></a>

#### Gravitational acceleration at a latitude and height, `gravity_at_latitude`

Normal gravity at sea level for a latitude (International Gravity Formula 1967, as UNESCO 1983) reduced for height with the inverse-square law.

$$
g = \frac{9.780318\,(1+5.2788\times10^{-3}\sin^2\varphi+2.36\times10^{-5}\sin^4\varphi)}{(1+z/R)^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lat` | in | latitude (degrees, north positive) | - |
| `z` | in | height above sea level (default 0) (optional) | m |
| `g` | out | gravitational acceleration (m/s^2) | m/s^2 |

**Applies when:** -90 <= latitude <= 90; spherical inverse-square reduction for height (R = 6371 km).

```shell
$ earth-science solve gravity_at_latitude lat=45 "z=0 m"
g = 9.806189875 m/s^2
  by: Gravitational acceleration at a latitude and height  (g = 9.780318 (1 + 5.2788e-3 sin^2 phi + 2.36e-5 sin^4 phi)/(1 + z/R)^2)
  lat = 45
  z = 0 m
  assumes: -90 <= latitude <= 90; spherical inverse-square reduction for height (R = 6371 km)
```

**What the worked example means:** This calculation determines the normal gravity value based on the International Gravity Formula, adjusted for the input latitude of 45 and reduced for a height above sea level of 0 m. The resulting gravitational acceleration of 9.80619 m/s^2 represents the theoretical standard force experienced at that location, assuming a spherical Earth model without local geological anomalies.

<a id="buried_sphere_anomaly"></a>

#### Gravity anomaly above a buried sphere: `buried_sphere_anomaly`

Peak vertical gravity anomaly directly above a buried sphere of radius R, depth z to its centre and density contrast drho.

$$
\Delta g = \frac{4\pi G\,\Delta\rho\,R^3}{3z^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `drho` | in | density contrast of the sphere | kg/m^3 |
| `R` | in | radius of the sphere | m |
| `z` | in | depth to the centre of the sphere | m |
| `dg_mGal` | out | peak gravity anomaly (mGal) | - |
| `dg` | out | peak gravity anomaly (m/s^2) | m/s^2 |

**Applies when:** 0 < R < z; observation point directly above the centre.

```shell
$ earth-science solve buried_sphere_anomaly "drho=500 kg/m^3" "R=100 m" "z=200 m"
dg_mGal = 0.3494655308
dg = 3.494655308e-06 m/s^2
  by: Gravity anomaly above a buried sphere  (dg = (4/3) pi G drho R^3/z^2)
  drho = 500 kg/m^3
  R = 100 m
  z = 200 m
  assumes: 0 < R < z; observation point directly above the centre
```

**What the worked example means:** This calculation determines the maximum vertical gravity anomaly directly above a buried sphere. It uses the density contrast, the sphere's radius, and the depth to its center. The resulting anomaly is 0.349466, which helps interpret subsurface structures like ore bodies or voids based on their physical properties.

<a id="bouguer_slab"></a>

#### Gravity anomaly of an infinite horizontal slab (Bouguer), `bouguer_slab`

Vertical gravity anomaly of an infinite horizontal slab of thickness h and density contrast drho.

$$
\Delta g = 2\pi G\,\Delta\rho\,h
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `drho` | in | density contrast of the slab | kg/m^3 |
| `h` | in | slab thickness | m |
| `dg_mGal` | out | gravity anomaly (mGal) | - |
| `dg` | out | gravity anomaly (m/s^2) | m/s^2 |

**Applies when:** laterally infinite slab; G = 6.6743e-11 N m^2/kg^2.

```shell
$ earth-science solve bouguer_slab "drho=2670 kg/m^3" "h=100 m"
dg_mGal = 11.19687561
dg = 0.0001119687561 m/s^2
  by: Gravity anomaly of an infinite horizontal slab (Bouguer)  (dg = 2 pi G drho h)
  drho = 2670 kg/m^3
  h = 100 m
  assumes: laterally infinite slab; G = 6.6743e-11 N m^2/kg^2
```

**What the worked example means:** This calculation determines the vertical gravity anomaly caused by an infinite horizontal slab. It uses the density contrast of 2670 kg/m^3 and the slab thickness of 100 m to produce a result of 11.1969. This value represents the gravitational effect of the layer, which is used to correct observed data for terrain or sediment mass.

### Geothermics (2)

<a id="geotherm_radiogenic"></a>

#### Steady geotherm with radiogenic heat production, `geotherm_radiogenic`

Temperature and heat flow at depth in a conducting layer with uniform radiogenic heat production, from the surface temperature and surface heat flow.

$$
T(z) = T_0 + \frac{q_0}{k}z - \frac{A}{2k}z^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T0` | in | surface temperature | K |
| `q0` | in | surface heat flow | W/m^2 |
| `k` | in | thermal conductivity | W/(m*K) |
| `A` | in | radiogenic heat production per unit volume | W/m^3 |
| `z` | in | depth | m |
| `T` | out | temperature at depth (K) | K |
| `q` | out | heat flow at depth (W/m^2) | W/m^2 |

**Applies when:** steady one-dimensional conduction; uniform heat production and conductivity.

```shell
$ earth-science solve geotherm_radiogenic "T0=283.15 K" "q0=0.065 W/m^2" "k=2.5 W/(m*K)" "A=1e-06 W/m^3" "z=10000 m"
T = 523.15 K
q = 0.055 W/m^2
  by: Steady geotherm with radiogenic heat production  (T(z) = T0 + q0 z/k - A z^2/(2k), q(z) = q0 - A z)
  T0 = 283.15 K
  q0 = 0.065 W/m^2
  k = 2.5 W/(m*K)
  A = 1e-06 W/m^3
  z = 10000 m
  assumes: steady one-dimensional conduction; uniform heat production and conductivity
```

**What the worked example means:** This calculation determines the steady-state temperature profile within a crustal layer where radioactive elements produce heat uniformly. Starting from a surface temperature of 283.15 K and an initial heat flow of 0.065 W/m^2, the model accounts for conductive heat transfer through material with thermal conductivity 2.5 W/(m*K). As depth increases, radiogenic heating adds energy to the system, while conduction moves it upward. The resulting temperature at depth 10000 m is 523.15 K, reflecting the balance between these processes. Simultaneously, the heat flow decreases with depth as some generated heat is absorbed locally, yielding a value of 0.055 W/m^2 at that specific point.

<a id="half_space_heat_flow"></a>

#### Surface heat flow of cooling oceanic lithosphere (half-space model), `half_space_heat_flow`

The conductive surface heat flow q = k dT/sqrt(pi kappa t) of oceanic lithosphere of age t (half-space cooling) and its thermal thickness. Defaults: dT 1300 K, k 3.3 W/(m K), kappa 1e-6 m^2/s.

$$
q = \frac{k\,\Delta T}{\sqrt{\pi\kappa t}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `t` | in | age of the seafloor (millions of years) | - |
| `dT` | in | mantle minus surface temperature (default 1300 K) (optional) | K |
| `k` | in | thermal conductivity (default 3.3 W/(m K)) (optional) | W/(m*K) |
| `kappa` | in | thermal diffusivity (default 1e-6 m^2/s) (optional) | m^2/s |
| `q` | out | surface heat flow (W/m^2) | W/m^2 |
| `q_mW` | out | surface heat flow (mW/m^2) | - |
| `thickness` | out | thermal thickness, where (T - Ts)/dT = 0.9 (m) | m |

**Applies when:** 0 < t <= 200 Myr; conductive half-space cooling.

```shell
$ earth-science solve half_space_heat_flow t=50
q = 0.06093199112 W/m^2
q_mW = 60.93199112
thickness = 92401.54658 m
  by: Surface heat flow of cooling oceanic lithosphere (half-space model)  (q = k dT/sqrt(pi kappa t); thermal thickness 2 erfinv(0.9) sqrt(kappa t) = 2.326 sqrt(kappa t))
  t = 50
  assumes: 0 < t <= 200 Myr; conductive half-space cooling
```

**What the worked example means:** For a seafloor age of 50, the surface heat flow is 0.060932 W/m^2 and the thermal thickness is 92401.5 m.

### Hydrogeology (1)

<a id="darcy_flow"></a>

#### Groundwater discharge by Darcy's law: `darcy_flow`

The volumetric discharge Q = K A dh/L through a porous medium, the specific discharge (Darcy flux) and, with a porosity, the average linear (seepage) velocity.

$$
Q = K A\,\frac{\Delta h}{L}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `K` | in | hydraulic conductivity | m/s |
| `A` | in | cross-sectional area of flow | m^2 |
| `dh` | in | head difference | m |
| `L` | in | flow length | m |
| `n` | in | effective porosity (optional) (optional) | - |
| `Q` | out | discharge (m^3/s) | m^3/s |
| `q` | out | specific discharge (Darcy flux, m/s) | m/s |
| `gradient` | out | hydraulic gradient dh/L | - |
| `v` | out | average linear velocity q/n (m/s) | m/s |

**Applies when:** K, A, L > 0; dh > 0; laminar flow in a saturated, homogeneous medium.

```shell
$ earth-science solve darcy_flow "K=0.0001 m/s" "A=50 m^2" "dh=2 m" "L=100 m"
Q = 0.0001 m^3/s
q = 2e-06 m/s
gradient = 0.02
  by: Groundwater discharge by Darcy's law  (Q = K A dh/L, q = Q/A, v = q/n)
  K = 0.0001 m/s
  A = 50 m^2
  dh = 2 m
  L = 100 m
  assumes: K, A, L > 0; dh > 0; laminar flow in a saturated, homogeneous medium
```

**What the worked example means:** This calculation determines the volumetric flow rate of water through a porous medium using Darcy's Law. By applying the hydraulic conductivity, cross-sectional area, head difference, and flow length, the model computes the total discharge as 0.0001 m^3/s. The resulting specific discharge represents the flux per unit area, while the average linear velocity indicates how fast water actually moves through the pore spaces when porosity is considered.

### Maps (1)

<a id="map_gradient_relief"></a>

#### Relief and gradient between two points of a topographic map, `map_gradient_relief`

The relief (difference in elevation) between two points, the ground distance (map distance times the scale denominator, or as given), the gradient in m per km, the slope as a fraction and percent and the slope angle.

$$
\text{gradient} = \frac{|z_2 - z_1|}{d_{map}\cdot S},\quad \theta = \arctan\frac{|z_2 - z_1|}{d}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `z1` | in | elevation of the first point | m |
| `z2` | in | elevation of the second point | m |
| `d_map` | in | distance measured on the map (optional: or give the ground distance) (optional) | m |
| `scale` | in | scale denominator (24000 for 1:24000; with the map distance) (optional) | - |
| `d` | in | ground (horizontal) distance (optional: or give the map distance and scale) (optional) | m |
| `relief` | out | relief \|z2 - z1\| (m) | m |
| `distance` | out | ground distance (m) | m |
| `gradient_m_per_km` | out | gradient (m per km) | - |
| `slope` | out | slope rise/run | - |
| `slope_percent` | out | slope (percent) | - |
| `slope_deg` | out | slope angle (degrees) | - |

**Applies when:** elevations between -11000 and 9000 m; the ground distance, or the map distance and the scale; 0 < map distance <= 2 m; 100 <= scale <= 1e7; relief is the absolute difference (uphill or downhill).

```shell
$ earth-science solve map_gradient_relief "z1=1200 m" "z2=1450 m" "d_map=0.05 m" scale=24000
relief = 250 m
distance = 1200 m
gradient_m_per_km = 208.3333333
slope = 0.2083333333
slope_percent = 20.83333333
slope_deg = 11.76828893
  by: Relief and gradient between two points of a topographic map  (relief = |z2 - z1|; run = d_map x scale; gradient = relief/run (x 1000 per km, x 100 percent); angle = atan(relief/run))
  z1 = 1200 m
  z2 = 1450 m
  d_map = 0.05 m
  scale = 24000
  assumes: elevations between -11000 and 9000 m; the ground distance, or the map distance and the scale; 0 < map distance <= 2 m; 100 <= scale <= 1e7; relief is the absolute difference (uphill or downhill)
```

In the worked example, z1 = 1200 m, z2 = 1450 m, d_map = 0.05 m, scale = 24000 go through the equation above and give relief = 250 m, distance = 1200 m, gradient_m_per_km = 208.3333333, slope = 0.2083333333. It applies only when elevations between -11000 and 9000 m; the ground distance, or the map distance and the scale; 0 < map distance <= 2 m; 100 <= scale <= 1e7; relief is the absolute difference (uphill or downhill): outside that it says what is wrong instead of returning a number.

### Ocean dynamics (3)

<a id="ekman_depth"></a>

#### Ekman layer depth: `ekman_depth`

Depth of frictional influence of the wind on the ocean (the current reverses direction at D_E = pi delta).

$$
D_E = \pi\sqrt{\frac{2A_z}{|f|}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Az` | in | vertical eddy viscosity | m^2/s |
| `lat` | in | latitude (degrees, north positive) | - |
| `D_E` | out | Ekman depth pi delta (m) | m |
| `delta` | out | Ekman e-folding depth sqrt(2 Az/\|f\|) (m) | m |

**Applies when:** 5 <= |latitude| <= 89; constant eddy viscosity; deep ocean.

```shell
$ earth-science solve ekman_depth "Az=0.05 m^2/s" lat=45
D_E = 97.82855061 m
delta = 31.13979481 m
  by: Ekman layer depth  (D_E = pi sqrt(2 Az/|f|))
  Az = 0.05 m^2/s
  lat = 45
  assumes: 5 <= |latitude| <= 89; constant eddy viscosity; deep ocean
```

**What the worked example means:** This calculation determines the vertical extent of wind-driven ocean currents. Using a vertical eddy viscosity of 0.05 m^2/s and a latitude of 45, it computes the Ekman depth, which is the level where the current direction reverses relative to the surface flow, yielding 97.8286 m. It also reports the e-folding depth, representing the scale over which frictional influence diminishes, calculated as 31.1398 m.

<a id="ekman_transport"></a>

#### Ekman transport: `ekman_transport`

Volume transport per unit width of the wind-driven Ekman layer, directed 90 degrees to the right of the wind in the northern hemisphere (left in the southern).

$$
M_E = \frac{\tau}{\rho\,|f|}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `tau` | in | wind stress on the sea surface | Pa |
| `lat` | in | latitude (degrees, north positive) | - |
| `rho` | in | seawater density (default 1025 kg/m^3) (optional) | kg/m^3 |
| `M` | out | Ekman volume transport per unit width (m^2/s) | m^2/s |
| `mass_transport` | out | Ekman mass transport per unit width (kg/(m s)) | kg/(m*s) |

**Applies when:** 5 <= |latitude| <= 89; steady state, deep ocean.

```shell
$ earth-science solve ekman_transport "tau=0.1 Pa" lat=45 "rho=1025 kg/m^3"
M = 0.9460359228 m^2/s
mass_transport = 969.6868209 kg/(m*s)
  by: Ekman transport  (M = tau/(rho |f|))
  tau = 0.1 Pa
  lat = 45
  rho = 1025 kg/m^3
  assumes: 5 <= |latitude| <= 89; steady state, deep ocean
```

**What the worked example means:** This calculation determines the net water movement driven by wind stress, known as Ekman transport. Using a wind stress of 0.1 Pa at a latitude of 45 degrees and a seawater density of 1025 kg/m^3, the model computes a volume transport per unit width of 0.946036 m^2/s. This value represents the total amount of water moving perpendicular to the wind direction due to the balance between frictional forces and the Coriolis effect.

<a id="shallow_water_wave"></a>

#### Speed of a shallow-water (long) wave and tsunami travel time, `shallow_water_wave`

Speed sqrt(g h) of a wave much longer than the water depth (tsunami, tide), and its travel time over a distance.

$$
c = \sqrt{g h},\quad t = d/c
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h` | in | water depth | m |
| `distance` | in | distance travelled (optional) (optional) | m |
| `c` | out | wave speed (m/s) | m/s |
| `travel_time` | out | travel time (s) | s |
| `travel_time_h` | out | travel time (hours) | - |

**Applies when:** wavelength much longer than the depth; standard gravity.

```shell
$ earth-science solve shallow_water_wave "h=4000 m" "distance=5000000 m"
c = 198.0570625 m/s
travel_time = 25245.24971 s
travel_time_h = 7.012569365
  by: Speed of a shallow-water (long) wave and tsunami travel time  (c = sqrt(g h), t = d/c)
  h = 4000 m
  distance = 5000000 m
  assumes: wavelength much longer than the depth; standard gravity
```

**What the worked example means:** The model calculates the speed of a long wave using the square root of gravity times water depth, yielding 198.057 m/s. It then determines the travel time over the given distance, resulting in 7.01257. This helps estimate when coastal areas might experience tsunami or tide effects.

### Oceanography (10)

<a id="seawater_adiabatic_gradient"></a>

#### Adiabatic temperature gradient of seawater (Bryden 1973), `seawater_adiabatic_gradient`

How fast the temperature of seawater rises under adiabatic compression, per decibar.

$$
\Gamma(S,T,p) = \left(\frac{\partial T}{\partial p}\right)_{\eta}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `S` | in | practical salinity (PSS-78) | - |
| `T` | in | in-situ temperature | K |
| `p` | in | sea pressure (0 at the surface) | Pa |
| `gamma` | out | adiabatic gradient (K per dbar) | - |
| `gamma_per_km_dbar` | out | adiabatic gradient (K per 1000 dbar) | - |

**Applies when:** 0 <= S <= 42; -2 °C <= T <= 40 °C.

```shell
$ earth-science solve seawater_adiabatic_gradient S=35 "T=283.15 K" "p=10000000 Pa"
gamma = 0.0001273870565
gamma_per_km_dbar = 0.1273870565
  by: Adiabatic temperature gradient of seawater (Bryden 1973)  (Bryden (1973) polynomial (UNESCO 1983 Eq. 30))
  S = 35
  T = 283.15 K
  p = 10000000 Pa
  assumes: 0 <= S <= 42; -2 °C <= T <= 40 °C
```

**What the worked example means:** This calculation determines the rate at which seawater temperature rises under adiabatic compression. Using practical salinity of 35, in-situ temperature of 283.15 K, and pressure of 1e+07 Pa, the model computes an adiabatic gradient of 0.000127387 K per dbar, which scales to 0.127387 K per 1000 dbar. This value quantifies how much heat is generated by pressure alone when water moves vertically without exchanging energy with its surroundings.

<a id="pure_water_density"></a>

#### Density of pure water (SMOW, Bigg 1967): `pure_water_density`

Density of standard mean ocean water (no salt) at atmospheric pressure, UNESCO 1983 polynomial.

$$
\rho_w = \sum_{i=0}^{5} a_i\,T_{68}^{\,i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | water temperature | K |
| `rho` | out | density (kg/m^3) | kg/m^3 |

**Applies when:** 0 °C <= T <= 40 °C; atmospheric pressure.

```shell
$ earth-science solve pure_water_density "T=277.15 K"
rho = 999.9749579 kg/m^3
  by: Density of pure water (SMOW, Bigg 1967)  (Bigg (1967) polynomial in T68 (UNESCO 1983 Eq. 14))
  T = 277.15 K
  assumes: 0 °C <= T <= 40 °C; atmospheric pressure
```

**What the worked example means:** This calculation determines the density of standard mean ocean water, which represents pure water without salt, based on its temperature. Using a polynomial approximation valid for temperatures between zero and forty degrees Celsius at atmospheric pressure, the model computes that at an input temperature of 277.15 K, the resulting water density is 999.975 kg/m^3. This value reflects how thermal energy affects the molecular spacing in liquid water, indicating that as the temperature rises within this range, the density decreases.

<a id="seawater_density_surface"></a>

#### Density of seawater at the surface (UNESCO 1981), `seawater_density_surface`

Density of seawater at atmospheric pressure from practical salinity and temperature (EOS-80), and sigma-t.

$$
\rho(S,T,0) = \rho_w + B(T)\,S + C(T)\,S^{3/2} + d\,S^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `S` | in | practical salinity (PSS-78) | - |
| `T` | in | water temperature | K |
| `rho` | out | density (kg/m^3) | kg/m^3 |
| `sigma_t` | out | sigma-t = rho - 1000 (kg/m^3) | - |

**Applies when:** 0 <= S <= 42; -2 °C <= T <= 40 °C.

```shell
$ earth-science solve seawater_density_surface S=35 "T=288.15 K"
rho = 1025.971963 kg/m^3
sigma_t = 25.97196289
  by: Density of seawater at the surface (UNESCO 1981)  (UNESCO (1981) one-atmosphere equation of state (EOS-80))
  S = 35
  T = 288.15 K
  assumes: 0 <= S <= 42; -2 °C <= T <= 40 °C
```

**What the worked example means:** This calculation determines the density of seawater at the surface using the UNESCO one-atmosphere equation of state. Given a practical salinity of 35 and a temperature of 288.15 K K, the resulting density is 1025.97 kg/m^3 kg/m3, which corresponds to a sigma-t value of 25.972. These values indicate how heavy the water is relative to fresh water, influencing its tendency to sink or float in ocean stratification.

<a id="ocean_depth_from_pressure"></a>

#### Depth from sea pressure (Fofonoff and Millard 1983), `ocean_depth_from_pressure`

Depth in the ocean from sea pressure and latitude (UNESCO 1983 standard algorithm).

$$
z = \frac{\sum c_i p^i}{g(\varphi) + \tfrac12\gamma' p}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | sea pressure (0 at the surface) | Pa |
| `lat` | in | latitude (degrees, north positive) | - |
| `depth` | out | depth (m) | m |

**Applies when:** 0 <= p <= 12,000 dbar.

```shell
$ earth-science solve ocean_depth_from_pressure "p=10000000 Pa" lat=30
depth = 990.8082107 m
  by: Depth from sea pressure (Fofonoff and Millard 1983)  (z = (9.72659 p - 2.2512e-5 p^2 + 2.279e-10 p^3 - 1.82e-15 p^4)/(g(phi) + 1.092e-6 p))
  p = 10000000 Pa
  lat = 30
  assumes: 0 <= p <= 12,000 dbar
```

**What the worked example means:** This calculation determines the ocean depth using sea pressure and latitude. By applying the standard UNESCO algorithm, it converts the measured pressure of 1e+07 Pa into a physical depth of 990.808 m. This result represents the vertical position of a sensor in the water column, accounting for local gravity variations based on the input latitude.

<a id="seawater_freezing_point"></a>

#### Freezing point of seawater (UNESCO 1983): `seawater_freezing_point`

The freezing temperature of seawater from practical salinity and sea pressure (Millero 1978).

$$
t_f = -0.0575S + 1.710523\times10^{-3}S^{3/2} - 2.154996\times10^{-4}S^2 - 7.53\times10^{-4}p
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `S` | in | practical salinity (PSS-78) | - |
| `p` | in | sea pressure (0 at the surface; default 0) (optional) | Pa |
| `Tf` | out | freezing temperature (K) | K |
| `Tf_C` | out | freezing temperature (°C, ITS-90) | - |

**Applies when:** 4 <= S <= 40; 0 <= p <= 500 dbar.

```shell
$ earth-science solve seawater_freezing_point S=35 "p=0 Pa"
Tf = 271.2281599 K
Tf_C = -1.9218401
  by: Freezing point of seawater (UNESCO 1983)  (t_f = -0.0575 S + 1.710523e-3 S^1.5 - 2.154996e-4 S^2 - 7.53e-4 p (p in dbar))
  S = 35
  p = 0 Pa
  assumes: 4 <= S <= 40; 0 <= p <= 500 dbar
```

**What the worked example means:** This calculation determines the point at which seawater turns to ice using the UNESCO 1983 algorithm. It takes the practical salinity value of 35 and the sea pressure of 0 Pa as inputs to produce a freezing temperature result of -1.92184 degrees Celsius. This metric is critical for modeling sea ice formation in polar regions, where both salt content and depth influence the phase change of water.

<a id="seawater_density"></a>

#### In-situ density of seawater (EOS-80): `seawater_density`

In-situ density of seawater from practical salinity, temperature and sea pressure with the secant bulk modulus.

$$
\rho(S,T,p) = \frac{\rho(S,T,0)}{1-p/K(S,T,p)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `S` | in | practical salinity (PSS-78) | - |
| `T` | in | in-situ temperature | K |
| `p` | in | sea pressure (0 at the surface) | Pa |
| `rho` | out | in-situ density (kg/m^3) | kg/m^3 |
| `sigma` | out | rho - 1000 (kg/m^3) | - |

**Applies when:** 0 <= S <= 42; -2 °C <= T <= 40 °C; 0 <= p <= 10,000 dbar.

```shell
$ earth-science solve seawater_density S=35 "T=275.15 K" "p=40000000 Pa"
rho = 1046.016843 kg/m^3
sigma = 46.01684344
  by: In-situ density of seawater (EOS-80)  (rho = rho(S,T,0)/(1 - P/K(S,T,P)), P in bar (UNESCO 1983))
  S = 35
  T = 275.15 K
  p = 40000000 Pa
  assumes: 0 <= S <= 42; -2 °C <= T <= 40 °C; 0 <= p <= 10,000 dbar
```

**What the worked example means:** This calculation determines the actual density of seawater at a specific depth using practical salinity, temperature, and pressure. The resulting in-situ density is 1046.02 kg/m^3, which reflects how compression increases mass per unit volume under high pressure. The derived sigma-t value is 46.0168, representing the density relative to pure water. This metric helps assess vertical stability and buoyancy forces within the ocean interior.

<a id="seawater_potential_temperature"></a>

#### Potential temperature of seawater (Fofonoff 1977), `seawater_potential_temperature`

The temperature a seawater parcel would have if moved adiabatically to a reference pressure (default the surface), by a fourth-order Runge-Kutta integration of the adiabatic gradient.

$$
\theta = T + \int_{p}^{p_r}\Gamma(S,\theta',p')\,dp'
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `S` | in | practical salinity (PSS-78) | - |
| `T` | in | in-situ temperature | K |
| `p` | in | sea pressure (0 at the surface) | Pa |
| `pr` | in | reference sea pressure (default 0, the surface) (optional) | Pa |
| `theta` | out | potential temperature (K) | K |
| `theta_C` | out | potential temperature (°C) | - |
| `dtheta` | out | theta - T (K) | K |

**Applies when:** 0 <= S <= 42; -2 °C <= T <= 40 °C.

```shell
$ earth-science solve seawater_potential_temperature S=35 "T=276.15 K" "p=50000000 Pa" "pr=0 Pa"
theta = 275.6632032 K
theta_C = 2.513203213
dtheta = -0.486796787 K
  by: Potential temperature of seawater (Fofonoff 1977)  (Fofonoff (1977) fourth-order Runge-Kutta integration of the adiabatic gradient from p to p_r)
  S = 35
  T = 276.15 K
  p = 50000000 Pa
  pr = 0 Pa
  assumes: 0 <= S <= 42; -2 °C <= T <= 40 °C
```

**What the worked example means:** This calculation determines the potential temperature of a seawater parcel by integrating the adiabatic gradient from its current pressure to a reference pressure. Using practical salinity 35, in-situ temperature 276.15 K K, and pressure 5e+07 Pa Pa, the method computes that the parcel would have a potential temperature of 275.663 K K if moved adiabatically to the reference pressure 0 Pa Pa. The difference between this value and the original temperature is -0.486797 K K, representing the removal of pressure-induced warming effects to provide a more accurate measure of heat content.

<a id="ocean_pressure_from_depth"></a>

#### Sea pressure at a depth (Saunders 1981): `ocean_pressure_from_depth`

Sea pressure at a depth in the ocean, allowing for latitude (Saunders 1981).

$$
p = \frac{(1-c_1)-\sqrt{(1-c_1)^2-8.84\times10^{-6}z}}{4.42\times10^{-6}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `depth` | in | depth below the sea surface | m |
| `lat` | in | latitude (degrees, north positive) | - |
| `p` | out | sea pressure (Pa) | Pa |
| `p_dbar` | out | sea pressure (dbar) | - |

**Applies when:** 0 <= depth <= 11,000 m.

```shell
$ earth-science solve ocean_pressure_from_depth "depth=1000 m" lat=30
p = 10095540.3 Pa
p_dbar = 1009.55403
  by: Sea pressure at a depth (Saunders 1981)  (Saunders (1981): p = [(1 - c1) - sqrt((1 - c1)^2 - 8.84e-6 z)]/4.42e-6)
  depth = 1000 m
  lat = 30
  assumes: 0 <= depth <= 11,000 m
```

**What the worked example means:** This calculation determines the absolute sea pressure in Pascals for a specific location in the ocean. By using a depth of 1000 m and a latitude of 30, the algorithm accounts for variations in gravity to produce a result of 1.00955e+07 Pa. This value is often converted to decibars, yielding 1009.55, which is useful for interpreting data from oceanographic sensors that measure pressure rather than direct depth.

<a id="sound_speed_seawater"></a>

#### Speed of sound in seawater (Chen and Millero 1977), `sound_speed_seawater`

Speed of sound in seawater from practical salinity, temperature and sea pressure (UNESCO 1983 algorithm).

$$
c(S,T,P) = C_w(T,P) + A(T,P)\,S + B(T,P)\,S^{3/2} + D(P)\,S^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `S` | in | practical salinity (PSS-78) | - |
| `T` | in | water temperature | K |
| `p` | in | sea pressure (0 at the surface) | Pa |
| `c` | out | speed of sound (m/s) | m/s |

**Applies when:** 0 <= S <= 40; 0 °C <= T <= 40 °C; 0 <= p <= 10,000 dbar.

```shell
$ earth-science solve sound_speed_seawater S=35 "T=283.15 K" "p=10000000 Pa"
c = 1506.346784 m/s
  by: Speed of sound in seawater (Chen and Millero 1977)  (Chen and Millero (1977) polynomial, P in bar (UNESCO 1983 Eq. 33))
  S = 35
  T = 283.15 K
  p = 10000000 Pa
  assumes: 0 <= S <= 40; 0 °C <= T <= 40 °C; 0 <= p <= 10,000 dbar
```

**What the worked example means:** This calculation determines the speed at which sound waves propagate through seawater using the Chen and Millero polynomial. By inputting the practical salinity of 35, the water temperature of 283.15 K, and the sea pressure of 1e+07 Pa, the model yields a result of 1506.35 m/s m/s. This value is essential for interpreting acoustic data, as it reflects how the physical properties of the ocean medium influence wave velocity under standard conditions.

<a id="wave_dispersion"></a>

#### Water-wave dispersion at finite depth: `wave_dispersion`

Wavelength, phase speed and group speed of a linear surface gravity wave of period T in water of depth h, from the full dispersion relation omega^2 = g k tanh(k h) (deep and shallow limits included).

$$
\omega^2 = g k \tanh(kh),\quad c_g = \frac{c}{2}\left(1 + \frac{2kh}{\sinh 2kh}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | wave period | s |
| `h` | in | water depth | m |
| `L` | out | wavelength (m) | m |
| `c` | out | phase speed (m/s) | m/s |
| `cg` | out | group speed (m/s) | m/s |
| `kh` | out | relative depth k h | - |
| `L0` | out | deep-water wavelength g T^2/(2 pi) (m) | m |

**Applies when:** 0.5 <= T <= 60 s; 0 < h <= 11,000 m; linear (small-amplitude) waves.

```shell
$ earth-science solve wave_dispersion "T=10 s" "h=20 m"
L = 121.209844 m
c = 12.1209844 m/s
cg = 9.271612121 m/s
kh = 1.036745053
L0 = 156.0776823 m
  by: Water-wave dispersion at finite depth  (omega^2 = g k tanh(k h) solved by Newton's method; c = omega/k; c_g = c (1 + 2kh/sinh 2kh)/2)
  T = 10 s
  h = 20 m
  assumes: 0.5 <= T <= 60 s; 0 < h <= 11,000 m; linear (small-amplitude) waves
```

**What the worked example means:** This calculation determines the properties of a linear surface gravity wave given its period and the water depth. By solving the dispersion relation, it finds that a wave with period 10 s traveling through 20 m deep water has a wavelength of 121.21 m meters. The associated phase speed is 12.121 m/s meters per second, while the energy propagates at a group speed of 9.27161 m/s meters per second. The relative depth parameter kh is calculated as 1.03675, indicating the regime between shallow and deep water limits.

### Plate tectonics (2)

<a id="seafloor_depth_age"></a>

#### Depth of the ocean floor with age (half-space cooling), `seafloor_depth_age`

Seafloor depth as oceanic lithosphere cools and contracts with age (half-space cooling, isostatic compensation under water). Defaults: ridge depth 2500 m, mantle 1300 °C, water 0 °C, alpha 3e-5 1/K, kappa 1e-6 m^2/s, rho_m 3300 and rho_w 1000 kg/m^3.

$$
d = d_r + \frac{2\rho_m\alpha(T_m-T_w)}{\rho_m-\rho_w}\sqrt{\frac{\kappa t}{\pi}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `t` | in | age of the seafloor (millions of years) | - |
| `dr` | in | depth at the ridge (default 2500 m) (optional) | m |
| `Tm` | in | mantle temperature (default 1300 °C) (optional) | K |
| `Tw` | in | seawater temperature (default 0 °C) (optional) | K |
| `alpha` | in | volumetric thermal expansion coefficient (default 3e-5 1/K) (optional) | 1/K |
| `kappa` | in | thermal diffusivity (default 1e-6 m^2/s) (optional) | m^2/s |
| `rho_m` | in | mantle density (default 3300 kg/m^3) (optional) | kg/m^3 |
| `rho_w` | in | seawater density (default 1000 kg/m^3) (optional) | kg/m^3 |
| `depth` | out | seafloor depth (m) | m |
| `subsidence` | out | subsidence below the ridge (m) | m |

**Applies when:** 0 < age <= 200 Myr (half-space cooling; old seafloor flattens in reality).

```shell
$ earth-science solve seafloor_depth_age t=50
depth = 5008.087917 m
subsidence = 2508.087917 m
  by: Depth of the ocean floor with age (half-space cooling)  (d = d_r + 2 rho_m alpha (Tm - Tw) sqrt(kappa t/pi)/(rho_m - rho_w))
  t = 50
  assumes: 0 < age <= 200 Myr (half-space cooling; old seafloor flattens in reality)
```

**What the worked example means:** This calculation determines the ocean floor's elevation based on the age of the tectonic plate. Using a half-space cooling model, it computes how much the lithosphere contracts as it loses heat to the surrounding water. The result shows that at an age of 50, the total depth reaches 5008.09 m, meaning the crust has sunk 2508.09 m below the initial ridge level. This relationship illustrates the fundamental link between thermal evolution and marine bathymetry.

<a id="plate_velocity"></a>

#### Plate velocity from an Euler pole: `plate_velocity`

Speed and direction of a point on a rigid plate rotating about an Euler pole (sphere of radius 6371 km).

$$
v = \omega R \sin\Delta
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `pole_lat` | in | latitude of the Euler pole | - |
| `pole_lon` | in | longitude of the Euler pole | - |
| `omega` | in | rotation rate about the pole, counter-clockwise positive (degrees per Myr) | - |
| `lat` | in | latitude of the site | - |
| `lon` | in | longitude of the site | - |
| `speed` | out | plate speed (mm/yr) | - |
| `azimuth` | out | direction of motion (degrees from north) | - |

**Applies when:** rigid plate on a sphere of radius 6371 km; site 0.5 to 179.5 degrees from the pole.

```shell
$ earth-science solve plate_velocity pole_lat=61.5 pole_lon=-73.7 omega=0.75 lat=35 lon=-120
speed = 52.79395817
azimuth = 123.0201167
  by: Plate velocity from an Euler pole  (v = omega R sin(Delta), direction perpendicular to the great circle to the pole)
  pole_lat = 61.5
  pole_lon = -73.7
  omega = 0.75
  lat = 35
  lon = -120
  assumes: rigid plate on a sphere of radius 6371 km; site 0.5 to 179.5 degrees from the pole
```

**What the worked example means:** This calculation determines the instantaneous velocity of a specific site on a rigid tectonic plate. Using the Euler pole coordinates and rotation rate, it computes that the point moves with a speed of 52.794 in the direction of 123.02 degrees from north. This result helps interpret how fast the crust is moving relative to the fixed reference frame defined by the pole.

### Radiation balance (6)

<a id="daily_mean_insolation"></a>

#### Daily mean insolation at the top of the atmosphere, `daily_mean_insolation`

The 24-hour mean solar flux on a horizontal surface at the top of the atmosphere for a latitude and solar declination (polar day and polar night included).

$$
\bar Q = \frac{S_0}{\pi}\left(h_0\sin\varphi\sin\delta + \cos\varphi\cos\delta\sin h_0\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lat` | in | latitude (degrees, north positive) | - |
| `decl` | in | solar declination (degrees) | - |
| `S0` | in | solar constant (at the actual Earth-Sun distance) | W/m^2 |
| `Q` | out | daily mean insolation (W/m^2) | W/m^2 |
| `day_length_h` | out | length of the day (hours) | - |

**Applies when:** |latitude| < 89.9; |declination| <= 23.5.

```shell
$ earth-science solve daily_mean_insolation lat=45 decl=23.44 "S0=1361 W/m^2"
Q = 499.3191739 W/m^2
day_length_h = 15.42589241
  by: Daily mean insolation at the top of the atmosphere  (Q = (S0/pi)(h0 sin phi sin delta + cos phi cos delta sin h0))
  lat = 45
  decl = 23.44
  S0 = 1361 W/m^2
  assumes: |latitude| < 89.9; |declination| <= 23.5
```

**What the worked example means:** This calculation determines the average solar energy received per square meter at the top of the atmosphere over a full day, based on the latitude of 45, the solar declination of 23.44, and a solar constant of 1361 W/m^2. The resulting daily mean insolation is 499.319 W/m^2, indicating the overall solar availability for that location and date. The corresponding length of the day is 15.4259, which reflects how long the sun remains above the horizon given these geometric conditions.

<a id="planetary_equilibrium_temperature"></a>

#### Planetary equilibrium (effective radiating) temperature, `planetary_equilibrium_temperature`

The temperature at which a planet radiating as a black body balances the sunlight it absorbs.

$$
T_e = \left(\frac{S(1-\alpha)}{4\sigma}\right)^{1/4}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `S` | in | solar flux at the planet's distance (solar constant) | W/m^2 |
| `albedo` | in | planetary (Bond) albedo, a fraction | - |
| `Te` | out | equilibrium temperature (K) | K |
| `absorbed` | out | absorbed flux per unit surface area, S(1 - albedo)/4 (W/m^2) | W/m^2 |

**Applies when:** 0 <= albedo < 1; fast rotation: absorbed sunlight spread over the whole sphere.

```shell
$ earth-science solve planetary_equilibrium_temperature "S=1361 W/m^2" albedo=0.3
Te = 254.5781401 K
absorbed = 238.175 W/m^2
  by: Planetary equilibrium (effective radiating) temperature  (T_e = (S (1 - albedo)/(4 sigma))^(1/4))
  S = 1361 W/m^2
  albedo = 0.3
  assumes: 0 <= albedo < 1; fast rotation: absorbed sunlight spread over the whole sphere
```

**What the worked example means:** This calculation determines the planetary equilibrium temperature, which represents the average surface temperature assuming the planet radiates energy as a perfect black body in balance with absorbed sunlight. The result depends on the solar flux at the planet's distance, given as 1361 W/m^2, and the Bond albedo, provided as 0.3. Using these inputs, the absorbed flux per unit surface area is calculated to be 238.175 W/m^2, leading to an equilibrium temperature of 254.578 K. This value serves as a baseline climate metric before accounting for atmospheric greenhouse effects.

<a id="blackbody_flux"></a>

#### Radiant flux emitted by a black or grey body (Stefan-Boltzmann), `blackbody_flux`

Power emitted per unit area by a surface of temperature T and (grey) emissivity eps.

$$
F = \varepsilon\,\sigma\,T^4
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | surface temperature | K |
| `eps` | in | emissivity (0 < eps <= 1; default 1, a black body) (optional) | - |
| `F` | out | emitted flux (W/m^2) | W/m^2 |

**Applies when:** 0 < eps <= 1; grey body: emissivity independent of wavelength.

```shell
$ earth-science solve blackbody_flux "T=288 K" eps=1
F = 390.1051535 W/m^2
  by: Radiant flux emitted by a black or grey body (Stefan-Boltzmann)  (F = eps sigma T^4 (sigma = 5.670374419e-8 W/(m^2 K^4)))
  T = 288 K
  eps = 1
  assumes: 0 < eps <= 1; grey body: emissivity independent of wavelength
```

**What the worked example means:** This calculation determines the radiant flux emitted by a surface, which represents the power radiated per unit area. The result of 390.105 W/m^2 is derived using the Stefan-Boltzmann law with a surface temperature of 288 K and an emissivity of 1. This value indicates how much thermal energy the body loses to its surroundings via radiation under these specific conditions.

<a id="greenhouse_single_layer"></a>

#### Surface temperature in the single-layer greenhouse model, `greenhouse_single_layer`

Surface and atmosphere temperatures when one isothermal layer, transparent to sunlight, absorbs and emits a fraction eps of thermal radiation.

$$
T_s = \left[\frac{S(1-\alpha)}{4\sigma(1-\varepsilon/2)}\right]^{1/4},\quad T_a = 2^{-1/4}T_s
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `S` | in | solar constant | W/m^2 |
| `albedo` | in | planetary (Bond) albedo, a fraction | - |
| `eps` | in | long-wave emissivity (absorptivity) of the atmospheric layer | - |
| `Ts` | out | surface temperature (K) | K |
| `Ta` | out | atmosphere temperature (K) | K |

**Applies when:** the layer is transparent to sunlight; 0 < eps <= 1.

```shell
$ earth-science solve greenhouse_single_layer "S=1361 W/m^2" albedo=0.3 eps=0.78
Ts = 288.0637769 K
Ta = 242.2317974 K
  by: Surface temperature in the single-layer greenhouse model  (T_s = [S(1 - albedo)/(4 sigma (1 - eps/2))]^(1/4), T_a = T_s/2^(1/4))
  S = 1361 W/m^2
  albedo = 0.3
  eps = 0.78
  assumes: the layer is transparent to sunlight; 0 < eps <= 1
```

**What the worked example means:** This calculation determines the equilibrium temperatures for a planet with a single atmospheric layer. Using a solar constant of 1361 W/m^2, a planetary albedo of 0.3, and an emissivity of 0.78, the formula produces a surface temperature of 288.064 K and an atmosphere temperature of 242.232 K. These values illustrate how the greenhouse effect raises the surface above the no-atmosphere baseline by trapping outgoing thermal radiation.

<a id="greenhouse_n_layers"></a>

#### Surface temperature with N fully absorbing layers, `greenhouse_n_layers`

Surface and layer temperatures of the N-layer grey-atmosphere model (each layer absorbs all thermal radiation).

$$
T_s = (N+1)^{1/4}\,T_e,\quad T_e = \left(\frac{S(1-\alpha)}{4\sigma}\right)^{1/4}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `S` | in | solar constant | W/m^2 |
| `albedo` | in | planetary (Bond) albedo, a fraction | - |
| `N` | in | number of absorbing layers (whole number) | - |
| `Ts` | out | surface temperature (K) | K |
| `T_top` | out | temperature of the top layer (K) | K |
| `T_bottom_layer` | out | temperature of the lowest layer (K) | K |

**Applies when:** 1 <= N <= 100 whole layers; layers transparent to sunlight, opaque to thermal radiation.

```shell
$ earth-science solve greenhouse_n_layers "S=1361 W/m^2" albedo=0.3 N=2
Ts = 335.0436745 K
T_top = 254.5781401 K
T_bottom_layer = 302.7461356 K
  by: Surface temperature with N fully absorbing layers  (T_s = (N + 1)^(1/4) T_e, layer i from the top at i^(1/4) T_e)
  S = 1361 W/m^2
  albedo = 0.3
  assumes: 1 <= N <= 100 whole layers; layers transparent to sunlight, opaque to thermal radiation
```

**What the worked example means:** This calculation determines the equilibrium temperatures for a planet with an atmosphere modeled as 2 fully absorbing layers. Using a solar constant of 1361 W/m^2 and an albedo of 0.3, the effective emission temperature is derived to find that the surface reaches 335.044 K. The atmospheric layers exhibit a temperature gradient, with the bottom layer at 302.746 K and the top layer radiating to space at 254.578 K. This demonstrates how multiple insulating layers trap heat, raising the surface temperature significantly above what it would be without an atmosphere.

<a id="wien_peak_wavelength"></a>

#### Wavelength of peak blackbody emission (Wien), `wien_peak_wavelength`

The wavelength at which a black body of temperature T emits most strongly per unit wavelength.

$$
\lambda_{\max} = \frac{b}{T},\quad b = 2.897771955\times10^{-3}\ \mathrm{m\,K}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | temperature of the body | K |
| `lambda_max` | out | peak wavelength (m) | m |
| `lambda_um` | out | peak wavelength (micrometres) | - |

**Applies when:** 1 K <= T <= 1e5 K.

```shell
$ earth-science solve wien_peak_wavelength "T=5772 K"
lambda_max = 5.020394932e-07 m
lambda_um = 0.5020394932
  by: Wavelength of peak blackbody emission (Wien)  (lambda_max = b/T, b = 2.897771955e-3 m K)
  T = 5772 K
  assumes: 1 K <= T <= 1e5 K
```

**What the worked example means:** This calculation determines the specific wavelength where a black body radiates most intensely per unit wavelength. Using Wien's displacement law with the constant b, the tool divides this constant by the input temperature 5772 K to find the peak wavelength. The result, 5.02039e-07 m, indicates that hotter objects emit their strongest light at shorter wavelengths, while cooler objects peak at longer wavelengths. This value is also provided as 0.502039 for easier comparison with common spectral bands.

### Sedimentology (1)

<a id="stokes_settling_velocity"></a>

#### Settling velocity of a small grain (Stokes), `stokes_settling_velocity`

Terminal settling velocity of a small sphere in a viscous fluid (Stokes' law), with the grain Reynolds number (refused when it exceeds 1).

$$
w = \frac{(\rho_s-\rho_f)\,g\,d^2}{18\,\mu}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d` | in | grain diameter | m |
| `rho_s` | in | grain density | kg/m^3 |
| `rho_f` | in | fluid density | kg/m^3 |
| `mu` | in | dynamic viscosity of the fluid | Pa*s |
| `w` | out | settling velocity (m/s) | m/s |
| `Re` | out | grain Reynolds number | - |

**Applies when:** grain Reynolds number <= 1; a smooth sphere, no neighbouring grains.

```shell
$ earth-science solve stokes_settling_velocity "d=5e-05 m" "rho_s=2650 kg/m^3" "rho_f=1000 kg/m^3" "mu=0.001 Pa*s"
w = 0.002247357292 m/s
Re = 0.1123678646
  by: Settling velocity of a small grain (Stokes)  (w = (rho_s - rho_f) g d^2/(18 mu))
  d = 5e-05 m
  rho_s = 2650 kg/m^3
  rho_f = 1000 kg/m^3
  mu = 0.001 Pa*s
  assumes: grain Reynolds number <= 1; a smooth sphere, no neighbouring grains
```

**What the worked example means:** This calculation determines the terminal settling velocity of a small spherical grain moving through a viscous fluid under gravity. Using Stokes' law, the result depends on the difference between the grain density and the fluid density, the gravitational acceleration, the square of the grain diameter, and the dynamic viscosity of the fluid. The output value for settling velocity is 0.00224736 m/s, indicating how fast the particle falls. Additionally, the grain Reynolds number is calculated as 0.112368 to verify that the flow remains laminar, which is a necessary condition for this method to be valid.

### Seismology (11)

<a id="sp_epicentral_distance"></a>

#### Distance to an earthquake from the S-P interval, `sp_epicentral_distance`

The distance from a station to the hypocentre from the delay between the S and P arrivals, for straight rays at constant P and S speeds.

$$
d = \frac{\Delta t\,v_p v_s}{v_p - v_s}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dt` | in | S minus P arrival time | s |
| `vp` | in | P-wave speed | m/s |
| `vs` | in | S-wave speed | m/s |
| `distance` | out | distance to the source (m) | m |
| `distance_km` | out | distance (km) | - |
| `tp` | out | P travel time (s) | s |
| `ts` | out | S travel time (s) | s |

**Applies when:** vp > vs > 0; straight rays at constant speeds (a local, crustal distance).

```shell
$ earth-science solve sp_epicentral_distance "dt=10 s" "vp=6000 m/s" "vs=3500 m/s"
distance = 84000 m
distance_km = 84
tp = 14 s
ts = 24 s
  by: Distance to an earthquake from the S-P interval  (d = dt vp vs/(vp - vs))
  dt = 10 s
  vp = 6000 m/s
  vs = 3500 m/s
  assumes: vp > vs > 0; straight rays at constant speeds (a local, crustal distance)
```

**What the worked example means:** This calculation determines the hypocentral distance by analyzing the time gap between the arrival of P and S waves. Using a measured S minus P interval of 10 s and assuming constant wave speeds, the model computes a source distance of 84000 m. This result implies that the seismic station is located 84 away from the earthquake's origin point in a straight line through the crust.

<a id="earthquake_energy"></a>

#### Earthquake energy and the energy and amplitude ratios of two magnitudes, `earthquake_energy`

The seismic energy radiated by an earthquake of magnitude M, log10 E = 1.5 M + 4.8 (E in J, Gutenberg-Richter), as joules and tonnes of TNT (4.184e9 J per tonne), and for a second magnitude its energy, the energy ratio 10^(1.5 (M2 - M1)) and the ground-amplitude ratio 10^(M2 - M1) (same distance and instrument).

$$
\log_{10}E = 1.5M + 4.8,\quad \frac{E_2}{E_1} = 10^{1.5(M_2 - M_1)},\quad \frac{A_2}{A_1} = 10^{M_2 - M_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M1` | in | magnitude of the (first) earthquake | - |
| `M2` | in | magnitude of a second earthquake (optional) (optional) | - |
| `E1` | out | energy released by the first earthquake (J) | J |
| `E1_TNT_t` | out | the same in tonnes of TNT | - |
| `E2` | out | energy released by the second earthquake (J) | J |
| `energy_ratio` | out | energy ratio E2/E1 | - |
| `amplitude_ratio` | out | amplitude ratio A2/A1 | - |

**Applies when:** -2 <= M <= 10; Gutenberg-Richter energy relation log10 E = 1.5 M + 4.8 (E in J); amplitude ratio for the same distance and instrument.

```shell
$ earth-science solve earthquake_energy M1=6 M2=7
E1 = 6.309573445e+13 J
E1_TNT_t = 15080.24246
E2 = 1.995262315e+15 J
energy_ratio = 31.6227766
amplitude_ratio = 10
  by: Earthquake energy and the energy and amplitude ratios of two magnitudes  (E = 10^(1.5 M + 4.8) J; E2/E1 = 10^(1.5 (M2 - M1)); A2/A1 = 10^(M2 - M1))
  M1 = 6
  M2 = 7
  assumes: -2 <= M <= 10; Gutenberg-Richter energy relation log10 E = 1.5 M + 4.8 (E in J); amplitude ratio for the same distance and instrument
```

In the worked example, M1 = 6, M2 = 7 go through the equation above and give E1 = 6.309573445e+13 J, E1_TNT_t = 15080.24246, E2 = 1.995262315e+15 J, energy_ratio = 31.6227766. It applies only when -2 <= M <= 10; Gutenberg-Richter energy relation log10 E = 1.5 M + 4.8 (E in J); amplitude ratio for the same distance and instrument: outside that it says what is wrong instead of returning a number.

<a id="elastic_constants_from_velocities"></a>

#### Elastic moduli and Poisson's ratio from Vp, Vs and density, `elastic_constants_from_velocities`

Shear modulus, Lamé's lambda, bulk modulus, Young's modulus and Poisson's ratio of an isotropic rock from its seismic velocities and density.

$$
\mu = \rho V_S^2,\quad \lambda = \rho(V_P^2-2V_S^2),\quad \nu = \frac{V_P^2-2V_S^2}{2(V_P^2-V_S^2)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vp` | in | P-wave speed | m/s |
| `Vs` | in | S-wave speed | m/s |
| `rho` | in | density of the rock | kg/m^3 |
| `mu` | out | shear modulus (Pa) | Pa |
| `lam` | out | Lamé's first parameter (Pa) | Pa |
| `K` | out | bulk modulus (Pa) | Pa |
| `E` | out | Young's modulus (Pa) | Pa |
| `nu` | out | Poisson's ratio | - |

**Applies when:** isotropic, linear elastic; Vp > sqrt(4/3) Vs.

```shell
$ earth-science solve elastic_constants_from_velocities "Vp=6000 m/s" "Vs=3460 m/s" "rho=2700 kg/m^3"
mu = 3.232332e+10 Pa
lam = 3.255336e+10 Pa
K = 5.410224e+10 Pa
E = 8.086560608e+10 Pa
nu = 0.250886451
  by: Elastic moduli and Poisson's ratio from Vp, Vs and density  (mu = rho Vs^2, lambda = rho (Vp^2 - 2 Vs^2), K = lambda + 2 mu/3, nu = (Vp^2 - 2Vs^2)/(2(Vp^2 - Vs^2)))
  Vp = 6000 m/s
  Vs = 3460 m/s
  rho = 2700 kg/m^3
  assumes: isotropic, linear elastic; Vp > sqrt(4/3) Vs
```

**What the worked example means:** This calculation determines the elastic properties of an isotropic rock from its measured P-wave speed, S-wave speed, and density. Using these inputs gives the shear modulus as 3.23233e+10 Pa, which resists shape change; Lamé's first parameter as 3.25534e+10 Pa; the bulk modulus as 5.41022e+10 Pa, which resists volume change; Young's modulus as 8.08656e+10 Pa, representing stiffness under tension or compression; and Poisson's ratio as 0.250886, describing lateral expansion when compressed axially.

<a id="epicentral_distance"></a>

#### Epicentral distance on a spherical Earth: `epicentral_distance`

Great-circle angular distance between an epicentre and a station, and its length on a sphere of radius 6371 km.

$$
\Delta = \operatorname{atan2}\!\left(\sqrt{(\cos\varphi_2\sin\Delta\lambda)^2+(\cos\varphi_1\sin\varphi_2-\sin\varphi_1\cos\varphi_2\cos\Delta\lambda)^2},\ \sin\varphi_1\sin\varphi_2+\cos\varphi_1\cos\varphi_2\cos\Delta\lambda\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lat1` | in | latitude of the first point (epicentre) | - |
| `lon1` | in | longitude of the first point (epicentre) | - |
| `lat2` | in | latitude of the second point (station) | - |
| `lon2` | in | longitude of the second point (station) | - |
| `delta` | out | epicentral distance (degrees) | - |
| `distance_km` | out | great-circle distance (km) | - |

**Applies when:** spherical Earth of radius 6371 km.

```shell
$ earth-science solve epicentral_distance lat1=35.7 lon1=139.7 lat2=51.5 lon2=-0.1
delta = 85.95388548
distance_km = 9557.635991
  by: Epicentral distance on a spherical Earth  (great-circle angle by atan2 (Vincenty's formula on the sphere))
  lat1 = 35.7
  lon1 = 139.7
  lat2 = 51.5
  lon2 = -0.1
  assumes: spherical Earth of radius 6371 km
```

**What the worked example means:** This calculation determines the shortest surface distance between an earthquake epicenter and a seismic station. Using the provided coordinates for the epicenter at latitude 35.7 and longitude 139.7, and the station at latitude 51.5 and longitude -0.1, the method computes the central angle via the Vincenty formula for spheres. The resulting angular separation is 85.9539 degrees, which translates to a physical distance of 9557.64 kilometers assuming a constant Earth radius of six thousand three hundred seventy-one kilometers.

<a id="geodesic_distance_azimuth"></a>

#### Geodesic distance and azimuths on the WGS84 ellipsoid (Vincenty), `geodesic_distance_azimuth`

Distance along the WGS84 ellipsoid between two points, the azimuth from the first to the second and the back-azimuth, by Vincenty's inverse method.

$$
s = bA(\sigma-\Delta\sigma)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lat1` | in | latitude of the first point (epicentre) | - |
| `lon1` | in | longitude of the first point (epicentre) | - |
| `lat2` | in | latitude of the second point (station) | - |
| `lon2` | in | longitude of the second point (station) | - |
| `distance` | out | geodesic distance (m) | m |
| `azimuth` | out | azimuth at point 1 (degrees from north) | - |
| `back_azimuth` | out | back-azimuth from point 2 (degrees from north) | - |

**Applies when:** WGS84 ellipsoid (a = 6378137 m, f = 1/298.257223563); 0.01 to 120 degrees apart.

```shell
$ earth-science solve geodesic_distance_azimuth lat1=35.7 lon1=139.7 lat2=51.5 lon2=-0.1
distance = 9581233.264 m
azimuth = 336.2493599
back_azimuth = 31.66321763
  by: Geodesic distance and azimuths on the WGS84 ellipsoid (Vincenty)  (Vincenty (1975) inverse iteration on the reduced latitudes)
  lat1 = 35.7
  lon1 = 139.7
  lat2 = 51.5
  lon2 = -0.1
  assumes: WGS84 ellipsoid (a = 6378137 m, f = 1/298.257223563); 0.01 to 120 degrees apart
```

**What the worked example means:** This calculation determines the precise shortest path along the Earth's surface, modeled as a WGS84 ellipsoid, connecting the seismic epicentre to the recording station. Using Vincenty's inverse method on reduced latitudes, it computes the total distance in meters alongside the initial direction from the source and the return direction at the receiver. The resulting values indicate that the wave traveled 9.58123e+06 m away from the origin point, arriving from a bearing of 31.6632 degrees relative to true north.

<a id="gutenberg_richter_b_value"></a>

#### Gutenberg-Richter b-value (maximum likelihood), `gutenberg_richter_b_value`

The b-value of the Gutenberg-Richter law log10 N = a - b M by Aki's maximum-likelihood estimator, with Utsu's correction for magnitudes binned at width dM; also the a-value of the cumulative count at Mc.

$$
b = \frac{\log_{10}e}{\overline{M}-\left(M_c-\Delta M/2\right)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `mags` | in | list of event magnitudes | - |
| `Mc` | in | magnitude of completeness | - |
| `dM` | in | magnitude bin width (0 for unbinned magnitudes; default 0) (optional) | - |
| `b` | out | b-value | - |
| `a` | out | a-value (log10 of the number of events >= Mc, plus b Mc) | - |
| `n` | out | number of events used | - |
| `mean` | out | mean magnitude of the events used | - |

**Applies when:** at least 10 events with M >= Mc; magnitudes complete above Mc.

```shell
$ earth-science solve gutenberg_richter_b_value "mags=[2.1, 2.4, 2, 3.1, 2.6, 2.2, 2.9, 2.3, 2, 4, 2.5, 2.1]" Mc=2 dM=0.1
b = 0.7664020269
a = 2.6119853
n = 12
mean = 2.516666667
  by: Gutenberg-Richter b-value (maximum likelihood)  (b = log10(e)/(mean(M) - (Mc - dM/2)) (Aki 1965, Utsu 1966))
  Mc = 2
  dM = 0.1
  assumes: at least 10 events with M >= Mc; magnitudes complete above Mc
```

**What the worked example means:** This calculation determines the Gutenberg-Richter b-value using Aki's maximum likelihood estimator, adjusted for magnitude binning width. Based on 12 earthquake events exceeding the completeness threshold of 2, the resulting slope is 0.766402. This value quantifies the relative frequency of small versus large earthquakes in the dataset, serving as a key indicator of seismic hazard and regional stress conditions.

<a id="local_magnitude"></a>

#### Local (Richter) magnitude from Wood-Anderson amplitude, `local_magnitude`

Local magnitude from the zero-to-peak Wood-Anderson amplitude and the hypocentral distance, with the attenuation of Bakun and Joyner (1984), as ObsPy.

$$
M_L = \log_{10}A + \log_{10}\frac{r}{100} + 0.00301\,(r-100) + 3.0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A` | in | zero-to-peak Wood-Anderson amplitude | m |
| `r` | in | hypocentral distance | m |
| `ML` | out | local magnitude | - |

**Applies when:** 10 km <= r <= 600 km; calibration of central California (Bakun and Joyner 1984).

```shell
$ earth-science solve local_magnitude "A=0.01 m" "r=100000 m"
ML = 4
  by: Local (Richter) magnitude from Wood-Anderson amplitude  (ML = log10 A + log10(r/100) + 0.00301 (r - 100) + 3.0 (A in mm, r in km))
  A = 0.01 m
  r = 100000 m
  assumes: 10 km <= r <= 600 km; calibration of central California (Bakun and Joyner 1984)
```

**What the worked example means:** This calculation determines the local magnitude, denoted as ML, using the zero-to-peak Wood-Anderson amplitude and the hypocentral distance. The result is 4, derived from an input amplitude of 0.01 m measured over a distance of 100000 m. This method applies specific attenuation corrections for central California to standardize the magnitude estimate.

<a id="moment_magnitude"></a>

#### Moment magnitude from seismic moment: `moment_magnitude`

Moment magnitude Mw of an earthquake from its scalar seismic moment (Hanks and Kanamori 1979, as ObsPy: Mw = log10(M0 in dyne cm)/1.5 - 10.7).

$$
M_w = \tfrac{2}{3}\log_{10}M_0[\mathrm{dyn\,cm}] - 10.7
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M0` | in | scalar seismic moment | N*m |
| `Mw` | out | moment magnitude | - |
| `M0_dyne_cm` | out | seismic moment (dyne cm) | - |

**Applies when:** 1e6 <= M0 <= 1e25 N m.

```shell
$ earth-science solve moment_magnitude "M0=1.1e+21 N*m"
Mw = 7.99426179
M0_dyne_cm = 1.1e+28
  by: Moment magnitude from seismic moment  (Mw = (2/3) log10(M0 [dyne cm]) - 10.7)
  M0 = 1.1e+21 N*m
  assumes: 1e6 <= M0 <= 1e25 N m
```

**What the worked example means:** This calculation determines the moment magnitude, which represents the total energy released by an earthquake. It is derived from the scalar seismic moment, given as 1.1e+21 N*m. The resulting value, 7.99426, provides a standardized measure of size that does not saturate like older magnitude scales.

<a id="nmo_velocity"></a>

#### Normal-moveout velocity from reflection picks, `nmo_velocity`

Stacking (NMO) velocity, zero-offset time and reflector depth from reflection travel times at several offsets, by a least-squares fit of t^2 = t0^2 + x^2/V^2.

$$
t^2 = t_0^2 + \frac{x^2}{V_{\mathrm{nmo}}^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | source-receiver offsets (list, in metres) | - |
| `t` | in | two-way reflection times (list, in seconds) | - |
| `V_nmo` | out | NMO velocity (m/s) | m/s |
| `t0` | out | zero-offset two-way time (s) | s |
| `depth` | out | reflector depth V t0/2 for a constant velocity (m) | m |

**Applies when:** flat reflector, constant velocity above it; offsets in m, times in s.

```shell
$ earth-science solve nmo_velocity "x=[0, 500, 1000, 1500, 2000]" "t=[1, 1.0198, 1.077, 1.1662, 1.2806]"
V_nmo = 2500.069557 m/s
t0 = 0.9999946612 s
depth = 1250.028105 m
  by: Normal-moveout velocity from reflection picks  (least squares of t^2 against x^2: slope 1/V^2, intercept t0^2)
  assumes: flat reflector, constant velocity above it; offsets in m, times in s
```

**What the worked example means:** This calculation determines the stacking velocity and intercept time by performing a least-squares linear regression on squared travel times plotted against squared source-receiver offsets. The slope of this line corresponds to the inverse square of the normal moveout velocity, while the y-intercept represents the square of the zero-offset two-way time. For the given data, the resulting normal moveout velocity is 2500.07 m/s, which allows for proper alignment of seismic traces during processing.

<a id="seismic_velocities_from_moduli"></a>

#### P- and S-wave speeds from bulk and shear moduli, `seismic_velocities_from_moduli`

Body-wave speeds of an isotropic elastic rock from its bulk modulus, shear modulus and density.

$$
V_P = \sqrt{\frac{K+\tfrac43\mu}{\rho}},\quad V_S = \sqrt{\frac{\mu}{\rho}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `K` | in | bulk modulus (incompressibility) | Pa |
| `mu` | in | shear modulus (rigidity) | Pa |
| `rho` | in | density of the rock | kg/m^3 |
| `Vp` | out | P-wave speed (m/s) | m/s |
| `Vs` | out | S-wave speed (m/s) | m/s |
| `Vp_Vs` | out | Vp/Vs | - |

**Applies when:** isotropic, linear elastic; K > 0, mu > 0.

```shell
$ earth-science solve seismic_velocities_from_moduli "K=5e+10 Pa" "mu=3e+10 Pa" "rho=2700 kg/m^3"
Vp = 5773.502692 m/s
Vs = 3333.333333 m/s
Vp_Vs = 1.732050808
  by: P- and S-wave speeds from bulk and shear moduli  (Vp = sqrt((K + 4 mu/3)/rho), Vs = sqrt(mu/rho))
  K = 5e+10 Pa
  mu = 3e+10 Pa
  rho = 2700 kg/m^3
  assumes: isotropic, linear elastic; K > 0, mu > 0
```

**What the worked example means:** This calculation determines the velocity of seismic body waves traveling through an isotropic rock. Combining the bulk modulus, which measures resistance to compression, with the shear modulus, which measures rigidity, and dividing by the density gives the P-wave speed of 5773.5 m/s and the S-wave speed of 3333.33 m/s. The ratio of these velocities, 1.73205, provides insight into the material's compressibility relative to its shear stiffness.

<a id="seismic_velocities_from_young"></a>

#### P- and S-wave speeds from Young's modulus and Poisson's ratio, `seismic_velocities_from_young`

Body-wave speeds of an isotropic elastic rock from Young's modulus, Poisson's ratio and density.

$$
V_P = \sqrt{\frac{E(1-\nu)}{\rho(1+\nu)(1-2\nu)}},\quad V_S = \sqrt{\frac{E}{2\rho(1+\nu)}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E` | in | Young's modulus | Pa |
| `nu` | in | Poisson's ratio | - |
| `rho` | in | density of the rock | kg/m^3 |
| `Vp` | out | P-wave speed (m/s) | m/s |
| `Vs` | out | S-wave speed (m/s) | m/s |
| `Vp_Vs` | out | Vp/Vs | - |

**Applies when:** isotropic, linear elastic; -1 < nu < 0.5.

```shell
$ earth-science solve seismic_velocities_from_young "E=7e+10 Pa" nu=0.25 "rho=2700 kg/m^3"
Vp = 5577.73351 m/s
Vs = 3220.305944 m/s
Vp_Vs = 1.732050808
  by: P- and S-wave speeds from Young's modulus and Poisson's ratio  (Lamé constants from E and nu, then Vp = sqrt((lambda + 2 mu)/rho), Vs = sqrt(mu/rho))
  E = 7e+10 Pa
  nu = 0.25
  rho = 2700 kg/m^3
  assumes: isotropic, linear elastic; -1 < nu < 0.5
```

**What the worked example means:** This calculation determines seismic wave velocities for an isotropic elastic rock using Young's modulus, Poisson's ratio, and density. By converting these engineering constants into Lamé parameters, the model derives the primary wave speed as 5577.73 m/s and the secondary wave speed as 3220.31 m/s. The resulting velocity ratio of 1.73205 indicates how much faster compressional waves travel compared to shear waves in this specific material configuration.

Questions and reports: support@ai999x.com. AI999X, The Future Is Self-Improving.
