# Earth Science: the desktop app, user guide

**Earth Science**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Earth Science covers atmospheric thermodynamics and meteorology, climate and radiation balance, oceanography and seawater properties, seismology and geophysics, and geology. Its desktop app holds all 87 calculations in 21 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-earth-science-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-earth-science-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-earth-science-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-earth-science.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-earth-science-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-earth-science-1.6.5-linux-x86_64.tar.gz
./ai999x-earth-science-1.6.5-linux-x86_64/ai999x-earth-science
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Earth Science app, opened on Lifting condensation level (Romps 2017) with its worked example.](https://www.ai999x.com/?ax_shot=earth-science-ai999x/hero.webp)

*The Earth Science app, opened on Lifting condensation level (Romps 2017) with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Atmospheric thermodynamics › Lifting condensation level (Romps 2017) (or type part of its name in the search box). It opens with a worked example filled in: “Lifting condensation level with pressure = 1000 hPa, temperature = 30 °C and dewpoint = 20 °C.” The answer: p_lcl = 86421.30262 Pa, T_lcl = 290.8124964 K.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Lifting condensation level (Romps 2017): the live scene of the worked example.](https://www.ai999x.com/?ax_shot=earth-science-ai999x/scene-1.webp)

*Lifting condensation level (Romps 2017): the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Lifting condensation level with pressure = 1000 hPa, temperature = 30 °C and dewpoint = 20 °C.” The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Lifting condensation level (Romps 2017)
p_lcl = 86421.30262 Pa
T_lcl = 290.8124964 K
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![In-situ density of seawater (EOS-80)](https://www.ai999x.com/?ax_shot=earth-science-ai999x/scene-2.webp)

*In-situ density of seawater (EOS-80)*

![Planetary equilibrium (effective radiating) temperature](https://www.ai999x.com/?ax_shot=earth-science-ai999x/scene-3.webp)

*Planetary equilibrium (effective radiating) temperature*

![Moment magnitude from seismic moment](https://www.ai999x.com/?ax_shot=earth-science-ai999x/scene-4.webp)

*Moment magnitude from seismic moment*

![Radiometric age from a daughter/parent ratio](https://www.ai999x.com/?ax_shot=earth-science-ai999x/scene-5.webp)

*Radiometric age from a daughter/parent ratio*

![Heat index (NWS)](https://www.ai999x.com/?ax_shot=earth-science-ai999x/scene-6.webp)

*Heat index (NWS)*

![Period of inertial oscillations](https://www.ai999x.com/?ax_shot=earth-science-ai999x/scene-7.webp)

*Period of inertial oscillations*

![Column density by Pratt isostasy](https://www.ai999x.com/?ax_shot=earth-science-ai999x/scene-8.webp)

*Column density by Pratt isostasy*

![Lifting condensation level (Romps 2017): a chart, enlarged.](https://www.ai999x.com/?ax_shot=earth-science-ai999x/chart-1.webp)

*Lifting condensation level (Romps 2017): a chart, enlarged.*

![In-situ density of seawater (EOS-80): a chart, enlarged.](https://www.ai999x.com/?ax_shot=earth-science-ai999x/chart-2.webp)

*In-situ density of seawater (EOS-80): a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Earth Science app in the dark theme.](https://www.ai999x.com/?ax_shot=earth-science-ai999x/hero-dark.webp)

*The Earth Science app in the dark theme.*

## 7. Every calculation

87 calculations in 21 topics; each has its own entry on the model's page (https://www.ai999x.com/models/earth-science-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Applied meteorology | Heat index (NWS), Wind chill temperature index |
| Atmospheric and ocean dynamics | Period of inertial oscillations |
| Atmospheric dynamics | Brunt-Väisälä (buoyancy) frequency, Coriolis parameter, Geostrophic wind from a height gradient, Gradient wind in curved flow, Phase speed of a barotropic Rossby wave, Rossby number, Wind components from speed and direction, Wind speed and direction from components |
| Atmospheric moisture | Dewpoint from temperature and relative humidity, Dewpoint from vapour pressure, Relative humidity from temperature and dewpoint, Saturation mixing ratio, Saturation vapour pressure over water, Specific humidity from dewpoint and pressure, Water-vapour mixing ratio from vapour pressure |
| Atmospheric thermodynamics | Atmospheric scale height, Density of moist air, Equivalent potential temperature (Bolton 1980), Geopotential and geopotential height, Height of a pressure level in the standard atmosphere, Lifting condensation level (Romps 2017), Moist static energy, Potential temperature, Pressure at a height in the standard atmosphere, Pressure at height in an isothermal atmosphere, Pressure at height with a constant lapse rate, Saturated (moist) adiabatic lapse rate, Temperature from potential temperature, Temperature of a dry parcel after adiabatic ascent, Thickness of a layer (hypsometric equation), Virtual temperature |
| Climate | Warming of a one-box energy-balance model |
| Climate and radiation | Radiative forcing of a change in CO2 (Myhre et al. 1998), Solar elevation and azimuth |
| Geochronology | Conventional radiocarbon age, Fraction of a radioactive parent remaining, Isochron age (e.g. Rb-Sr) from sample ratios, Potassium-argon age (branched decay), Radiometric age from a daughter/parent ratio |
| Geodynamics | Column density by Pratt isostasy, Mountain root by Airy isostasy |
| Geomagnetism | Inclination and strength of a geocentric dipole field, Paleolatitude from magnetic inclination |
| Geomorphology | Stream power of a river |
| Geophysics | Gravitational acceleration at a latitude and height, Gravity anomaly above a buried sphere, Gravity anomaly of an infinite horizontal slab (Bouguer) |
| Geothermics | Steady geotherm with radiogenic heat production, Surface heat flow of cooling oceanic lithosphere (half-space model) |
| Hydrogeology | Groundwater discharge by Darcy's law |
| Maps | Relief and gradient between two points of a topographic map |
| Ocean dynamics | Ekman layer depth, Ekman transport, Speed of a shallow-water (long) wave and tsunami travel time |
| Oceanography | Adiabatic temperature gradient of seawater (Bryden 1973), Density of pure water (SMOW, Bigg 1967), Density of seawater at the surface (UNESCO 1981), Depth from sea pressure (Fofonoff and Millard 1983), Freezing point of seawater (UNESCO 1983), In-situ density of seawater (EOS-80), Potential temperature of seawater (Fofonoff 1977), Sea pressure at a depth (Saunders 1981), Speed of sound in seawater (Chen and Millero 1977), Water-wave dispersion at finite depth |
| Plate tectonics | Depth of the ocean floor with age (half-space cooling), Plate velocity from an Euler pole |
| Radiation balance | Daily mean insolation at the top of the atmosphere, Planetary equilibrium (effective radiating) temperature, Radiant flux emitted by a black or grey body (Stefan-Boltzmann), Surface temperature in the single-layer greenhouse model, Surface temperature with N fully absorbing layers, Wavelength of peak blackbody emission (Wien) |
| Sedimentology | Settling velocity of a small grain (Stokes) |
| Seismology | Distance to an earthquake from the S-P interval, Earthquake energy and the energy and amplitude ratios of two magnitudes, Elastic moduli and Poisson's ratio from Vp, Vs and density, Epicentral distance on a spherical Earth, Geodesic distance and azimuths on the WGS84 ellipsoid (Vincenty), Gutenberg-Richter b-value (maximum likelihood), Local (Richter) magnitude from Wood-Anderson amplitude, Moment magnitude from seismic moment, Normal-moveout velocity from reflection picks, P- and S-wave speeds from bulk and shear moduli, P- and S-wave speeds from Young's modulus and Poisson's ratio |

## 8. Other ways to use it

The same engine runs on the command line (`earth-science`, its own guide), in Python (`pip install earth-science-ai999x`, `import earth_science_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/earth-science-ai999x/.

## Credits

- Coastlines and lakes: Natural Earth (public domain).

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/earth-science-ai999x/
