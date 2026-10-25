"""Earth science procedures: atmospheric thermodynamics and meteorology, climate and radiation balance, oceanography
and seawater properties, seismology and geophysics, and geology (domain code, loaded by the pack; enabled only once
tools/certify_procedures.py has certified them).

Every capability states its inputs, outputs, conditions, method and LaTeX, the documented question form it reads, worked
examples (their outputs were computed by the reference, not by the procedure), an INDEPENDENT reference computation
used only by certification (the checking libraries are imported inside the reference functions: MetPy, python-seawater,
ObsPy, SciPy, NumPy, SymPy) and a deliberately wrong variant that the checks must reject. The shipped code (run,
verify, parse) is plain Python.

Conventions (stated once, used everywhere):
- Inputs and outputs are SI (K, Pa, m, m/s, kg/m^3, W/m^2 ...). The question reader also accepts the usual spellings
  (°C, °F, hPa, mb, dbar, km, km/h, knots, g/cm^3, GPa, mW/m^2 ...) and converts them; solve() takes SI or any unit
  of the shared reader. Latitudes, longitudes, directions, declinations, hour angles and inclinations are plain numbers
  in DEGREES (north and east positive). Ages and half-lives are plain numbers in YEARS (the reader converts ka, Ma, Ga);
  seafloor ages in millions of years; plate rotation rates in degrees per million years; plate speeds in mm/yr;
  gravity anomalies in mGal. Relative humidity, mixing ratio and specific humidity are fractions (kg/kg); the reader
  turns "65%" into 0.65. A year is the Julian year, 365.25 days.
- Moist thermodynamics follows MetPy 1.7's constants and conventions: R_d = 287.04749097718457, R_v =
  461.52311572606084, c_pd = 1004.6662184201462, c_pl = 4219.4, c_pv = 1860.078011865639 J/(kg K), L_v(273.16 K) =
  2.50084e6 J/kg, epsilon = 0.6219569100577033, e_s(273.16 K) = 611.2 Pa, g = 9.80665 m/s^2, Omega = 7.292115e-5
  rad/s, Earth radius 6371008.7714 m, reference pressure 1000 hPa. Saturation vapour pressure over liquid water is the
  Ambaum (2020) formula with a temperature-dependent latent heat (as MetPy); dewpoint from vapour pressure is the
  inversion of Bolton's (1980) formula (as MetPy, so the two are not exact inverses of each other); the LCL is Romps'
  (2017) exact solution; equivalent potential temperature is Bolton's (1980) formula; the standard atmosphere is
  MetPy's (T0 = 288 K, lapse rate 6.5 K/km, p0 = 1013.25 hPa).
- Seawater follows EOS-80 / UNESCO 1983 as in python-seawater: practical salinity (PSS-78), ITS-90 temperatures
  (converted to IPTS-68 inside the formulas by T68 = 1.00024 T90), sea pressure in dbar (0 at the surface).
- Seismic magnitudes: moment magnitude Mw = log10(M0 [dyne cm])/1.5 - 10.7 (Hanks and Kanamori 1979, as ObsPy);
  local magnitude ML from the zero-to-peak Wood-Anderson amplitude in mm and the hypocentral distance in km after
  Bakun and Joyner (1984), as ObsPy. Geodesics on the WGS84 ellipsoid by Vincenty's inverse method.
- Outputs built from exp/log/trig are rounded to 12 significant digits so that every platform prints the same digits.
"""
from __future__ import annotations

import math
import re
from functools import lru_cache

# =============================================================================================== constants (MetPy 1.7)
RD = 287.04749097718457
RV = 461.52311572606084
CPD = 1004.6662184201462
CPL = 4219.400000000001
CPV = 1860.078011865639
EPS = 0.6219569100577033
LV0 = 2500840.0
T0TP = 273.16
ES0 = 611.2
KAPPA = 0.28571428571428564
G0 = 9.80665
OMEGA = 7.292115e-05
RE = 6371008.7714
P0 = 100000.0
SIGMA = 5.670374419e-08
WIEN_B = 2.897771955e-03
YEAR = 3.15576e7
G_NEWTON = 6.6743e-11
R_EARTH_KM = 6371.0


def _r(x):
    """12 significant digits, never a negative zero."""
    x = float(x)
    if x == 0.0 or not math.isfinite(x):
        return 0.0 if x == 0.0 else x
    return float(f"{x:.12g}") + 0.0


def _rel(a, b):
    try:
        a, b = float(a), float(b)
    except (TypeError, ValueError):
        return 1.0
    if not (math.isfinite(a) and math.isfinite(b)):
        return 1.0
    d = abs(a - b)
    if d == 0.0:
        return 0.0
    if d <= 1e-15:
        return 0.0
    return d / max(abs(a), abs(b))


def _need(cond, message):
    if not cond:
        raise ValueError(message)


def _finite(**kw):
    for k, v in kw.items():
        if isinstance(v, bool) or not isinstance(v, (int, float)) or not math.isfinite(v):
            raise ValueError(f"{k} must be a finite number")


# =============================================================================================== units of the reader
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_F = 5.0 / 9.0
_UNITS = {
    "temperature": {"K": (1, 0), "kelvin": (1, 0), "kelvins": (1, 0), "°C": (1, 273.15), "degC": (1, 273.15),
                    "C": (1, 273.15), "celsius": (1, 273.15), "°F": (_F, 273.15 - 32 * _F), "degF": (_F, 273.15 - 32 * _F),
                    "F": (_F, 273.15 - 32 * _F), "fahrenheit": (_F, 273.15 - 32 * _F)},
    "pressure": {"Pa": (1, 0), "hPa": (100, 0), "kPa": (1e3, 0), "MPa": (1e6, 0), "GPa": (1e9, 0), "mb": (100, 0),
                 "mbar": (100, 0), "millibar": (100, 0), "millibars": (100, 0), "bar": (1e5, 0), "atm": (101325, 0),
                 "dbar": (1e4, 0), "decibar": (1e4, 0), "decibars": (1e4, 0)},
    "length": {"m": (1, 0), "km": (1e3, 0), "cm": (1e-2, 0), "mm": (1e-3, 0), "µm": (1e-6, 0), "um": (1e-6, 0),
               "micron": (1e-6, 0), "microns": (1e-6, 0), "ft": (0.3048, 0), "metres": (1, 0), "meters": (1, 0),
               "kilometres": (1e3, 0), "kilometers": (1e3, 0)},
    "speed": {"m/s": (1, 0), "km/h": (1 / 3.6, 0), "kph": (1 / 3.6, 0), "km/hr": (1 / 3.6, 0), "knots": (1852 / 3600, 0),
              "knot": (1852 / 3600, 0), "kt": (1852 / 3600, 0), "kn": (1852 / 3600, 0), "mph": (0.44704, 0),
              "cm/s": (1e-2, 0), "km/s": (1e3, 0)},
    "density": {"kg/m^3": (1, 0), "kg/m3": (1, 0), "g/cm^3": (1e3, 0), "g/cm3": (1e3, 0), "g/cc": (1e3, 0)},
    "time": {"s": (1, 0), "min": (60, 0), "h": (3600, 0), "hr": (3600, 0), "hours": (3600, 0), "day": (86400, 0),
             "days": (86400, 0)},
    "lapse_rate": {"K/m": (1, 0), "K/km": (1e-3, 0), "°C/km": (1e-3, 0), "degC/km": (1e-3, 0), "C/km": (1e-3, 0),
                   "°C/m": (1, 0)},
    "radiative_flux": {"W/m^2": (1, 0), "W/m2": (1, 0), "Wm^-2": (1, 0), "kW/m^2": (1e3, 0)},
    "heat_flux": {"W/m^2": (1, 0), "W/m2": (1, 0), "mW/m^2": (1e-3, 0), "mW/m2": (1e-3, 0)},
    "specific_energy": {"J/kg": (1, 0), "kJ/kg": (1e3, 0)},
    "dynamic_viscosity": {"Pa*s": (1, 0), "Pas": (1, 0), "Pa.s": (1, 0), "mPa*s": (1e-3, 0), "mPa.s": (1e-3, 0),
                          "cP": (1e-3, 0), "N*s/m^2": (1, 0), "kg/(m*s)": (1, 0)},
    "kinematic_viscosity": {"m^2/s": (1, 0), "m2/s": (1, 0), "cm^2/s": (1e-4, 0)},
    "shear_stress": {"Pa": (1, 0), "N/m^2": (1, 0), "N/m2": (1, 0)},
    "modulus": {"Pa": (1, 0), "kPa": (1e3, 0), "MPa": (1e6, 0), "GPa": (1e9, 0)},
    "thermal_conductivity": {"W/(m*K)": (1, 0), "W/m/K": (1, 0), "W/mK": (1, 0), "W/(mK)": (1, 0), "W/(m*°C)": (1, 0)},
    "volumetric_heat_production": {"W/m^3": (1, 0), "W/m3": (1, 0), "µW/m^3": (1e-6, 0), "uW/m^3": (1e-6, 0),
                                   "µW/m3": (1e-6, 0), "uW/m3": (1e-6, 0)},
    "thermal_diffusivity": {"m^2/s": (1, 0), "m2/s": (1, 0), "mm^2/s": (1e-6, 0)},
    "seismic_moment": {"N*m": (1, 0), "Nm": (1, 0), "N·m": (1, 0), "dyne*cm": (1e-7, 0), "dyn*cm": (1e-7, 0),
                       "dyne-cm": (1e-7, 0)},
    "expansion_coefficient": {"1/K": (1, 0), "K^-1": (1, 0), "/K": (1, 0)},
    "heat_capacity_per_area": {"J/(m^2*K)": (1, 0), "J/m^2/K": (1, 0), "J/(m2*K)": (1, 0), "J/m2/K": (1, 0),
                               "J/(m^2K)": (1, 0)},
    "climate_sensitivity": {"K/(W/m^2)": (1, 0), "K*m^2/W": (1, 0), "Km^2/W": (1, 0), "K/(Wm^-2)": (1, 0)},
    "number": {"": (1, 0)},
    "fraction": {"": (1, 0), "%": (0.01, 0), "percent": (0.01, 0)},
    "degrees": {"": (1, 0), "°": (1, 0), "deg": (1, 0), "degree": (1, 0), "degrees": (1, 0)},
    "years": {"yr": (1, 0), "yrs": (1, 0), "year": (1, 0), "years": (1, 0), "a": (1, 0), "kyr": (1e3, 0),
              "ka": (1e3, 0), "Myr": (1e6, 0), "Ma": (1e6, 0), "Gyr": (1e9, 0), "Ga": (1e9, 0)},
    "myr": {"Myr": (1, 0), "Ma": (1, 0), "million years": (1, 0), "kyr": (1e-3, 0), "ka": (1e-3, 0)},
    "per_year": {"1/yr": (1, 0), "/yr": (1, 0), "yr^-1": (1, 0), "peryear": (1, 0), "1/a": (1, 0), "a^-1": (1, 0)},
    "deg_per_myr": {"°/Myr": (1, 0), "deg/Myr": (1, 0), "degrees/Myr": (1, 0), "°/Ma": (1, 0), "deg/Ma": (1, 0)},
    "mm": {"mm": (1, 0), "cm": (10, 0), "m": (1e3, 0)},
    "km_plain": {"km": (1, 0), "m": (1e-3, 0)},
    "per_dbar": {"K/dbar": (1, 0), "°C/dbar": (1, 0)},
}


def _norm_unit(u: str) -> str:
    u = u.strip().replace("²", "^2").replace("³", "^3").replace("·", "*").replace("−", "-").replace("º", "°")
    u = re.sub(r"\bdeg(?:rees?)?\s+C\b", "°C", u)
    u = re.sub(r"\bdeg(?:rees?)?\s+F\b", "°F", u)
    u = re.sub(r"\bdegrees?\s+(?:celsius|Celsius)\b", "°C", u)
    u = re.sub(r"\s+", "", u) if not re.fullmatch(r"[A-Za-z ]+", u) else re.sub(r"\s+", " ", u)
    return u.replace(" ", "") if u not in ("million years",) else u


def _convert(value: float, unit: str, table: str):
    t = _UNITS[table]
    u = _norm_unit(unit)
    if u in t:
        f, off = t[u]
    else:
        low = {k.lower(): v for k, v in t.items() if len(k) >= 3}
        if len(u) >= 3 and u.lower() in low:
            f, off = low[u.lower()]
        else:
            return None
    return value * f + off


# =============================================================================================== the procedure registry
PROCEDURES: dict = {}
_ROUTES: list = []          # (phrase, key): a question is routed to the procedure whose phrase is the longest found
_SPEC: dict = {}            # key -> internal reading information


def I(sym, kind, table, meaning, names=(), show="", required=True, typ=None, **extra):  # noqa: E743
    d = {"symbol": sym, "kind": kind, "meaning": meaning, "required": required}
    if typ:
        d["type"] = typ
    d.update(extra)
    return d, table, [sym] + list(names), show


def O(sym, kind, meaning):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


def _route(head: str):
    h = " " + re.sub(r"\s+", " ", head.lower().replace("-", " ")) + " "
    for phrase, key in _ROUTES:
        if re.search(r"(?<![a-z])" + re.escape(phrase) + r"(?![a-z])", h):
            return key
    return None


def _show_value(v):
    if isinstance(v, (list, tuple)):
        return "[" + ", ".join(f"{x:g}" for x in v) + "]"
    if isinstance(v, str):
        return v
    return f"{v:g}"


def _read(key, body):
    """'name = value unit, name = value unit and name = [a, b, c]' -> SI values; None for anything else."""
    info = _SPEC[key]
    body = body.strip().rstrip(".?! ")
    lists = []

    def keep(m):
        lists.append(m.group(1))
        return f"\x00{len(lists) - 1}\x00"
    body = re.sub(r"\[([^\[\]]*)\]", keep, body)
    items = [s for s in re.split(r"\s*(?:,|;|\band\b)\s*", body) if s.strip()]
    if not items:
        return None
    out = {}
    for item in items:
        m = re.fullmatch(r"([A-Za-z][\w ()'/\-]*?)\s*=\s*(.+)", item.strip())
        if not m:
            return None
        name, val = re.sub(r"\s+", " ", m.group(1).strip().lower()), m.group(2).strip()
        sym = info["names"].get(name)
        if sym is None or sym in out:
            return None
        spec, table = info["inputs"][sym]
        typ = spec.get("type", "number")
        lm = re.fullmatch("\x00(\\d+)\x00", val)
        if typ == "vector":
            if not lm:
                return None
            nums = [x for x in re.split(r"[\s,;]+", lists[int(lm.group(1))].strip()) if x]
            if not nums or not all(re.fullmatch(_NUM, x) for x in nums):
                return None
            out[sym] = ", ".join(nums)
            continue
        if lm:
            return None
        if typ in ("choice", "boolean"):
            out[sym] = val
            continue
        nm = re.match("(" + _NUM + r")\s*(.*)$", val)
        if not nm:
            return None
        number, unit = float(nm.group(1)), nm.group(2)
        si = _convert(number, unit, table)                 # an unknown unit (or a second number) is refused
        if si is None:
            return None
        if typ == "integer":
            if si != int(si):
                return None
            si = int(si)
        out[sym] = si
    return out


def _parser(key):
    def parse(question):
        if not isinstance(question, str):
            return None
        m = re.search(r"\s+with\s+|\s*:\s+", question)
        if not m:
            return None
        head, body = question[:m.start()], question[m.end():]
        if re.search(r"\d", head) or _route(head) != key:
            return None
        return _read(key, body)
    return parse


def _verifier(run, extra=None):
    def verify(inputs, outputs):
        try:
            again = run(**inputs)
        except Exception:
            return 1.0
        worst = 0.0
        for k, v in again.items():
            if k not in outputs:
                return 1.0
            if isinstance(v, str):
                worst = max(worst, 0.0 if outputs[k] == v else 1.0)
            else:
                worst = max(worst, _rel(outputs[k], v))
        if extra is not None:
            try:
                worst = max(worst, float(extra(inputs, outputs)))
            except Exception:
                return 1.0
        return worst
    return verify


def proc(key, name, topic, purpose, inputs, outputs, run, reference, wrong, rand, examples, phrases, *,
         conditions=(), method="", latex="", invalid=(), extra_verify=None, primary=None):
    """Register one procedure. `examples` are dicts of input values written in each input's display unit (lists for
    vectors); their outputs come from EXAMPLE_OUTPUTS (computed by the reference) or, until then, from `run`."""
    specs = {}
    names = {}
    shows = {}
    for d, table, aliases, show in inputs:
        specs[d["symbol"]] = (d, table)
        for a in aliases:
            names[a.lower()] = d["symbol"]
        shows[d["symbol"]] = (aliases[1] if len(aliases) > 1 else aliases[0], show, table)
    _SPEC[key] = {"inputs": specs, "names": names}
    for ph in phrases:
        _ROUTES.append((ph.lower(), key))
    _ROUTES.sort(key=lambda r: -len(r[0]))

    def to_si(ex):
        out = {}
        for s, v in ex.items():
            d, table = specs[s]
            if d.get("type") in ("vector", "choice", "boolean", "text"):
                out[s] = list(v) if isinstance(v, (list, tuple)) else v
            else:
                si = _convert(float(v), shows[s][1], table)
                out[s] = int(si) if d.get("type") == "integer" else si
        return out

    def as_text(ex, drop=None):
        parts = []
        for s, v in ex.items():
            if s == drop:
                continue
            label, show, _t = shows[s]
            parts.append(f"{label} = {_show_value(v)}" + (f" {show}" if show else ""))
        text = ", ".join(parts[:-1]) + (" and " if len(parts) > 1 else "") + parts[-1]
        return f"{phrases[0][0].upper()}{phrases[0][1:]} with {text}."

    stored = EXAMPLE_OUTPUTS.get(key)
    exs = []
    for i, ex in enumerate(examples):
        si = to_si(ex)
        outs = stored[i] if stored and i < len(stored) else {k: v for k, v in run(**si).items()}
        exs.append({"inputs": si, "outputs": outs})
    questions = [{"q": as_text(ex), "outputs": exs[i]["outputs"]} for i, ex in enumerate(examples[:2])]
    req = [d["symbol"] for d, *_ in inputs if d.get("required")]
    if len(req) >= 2:
        questions.append({"q": as_text(examples[0], drop=req[-1]), "outcome": "NEEDS_INPUT"})
    first = next(iter(examples[0]))
    questions.append({"q": as_text(examples[0])[:-1] + " and colour = 3.", "outcome": "UNRECOGNIZED_FORMAT"})
    if first:
        pass
    for bad in invalid:
        questions.append({"q": as_text(bad), "outcome": "INVALID_INPUT"})
    PROCEDURES[key] = {
        "name": name, "topic": topic, "purpose": purpose, "inputs": [d for d, *_ in inputs], "outputs": outputs,
        "primary": primary or outputs[0]["symbol"], "conditions": list(conditions), "method": method, "latex": latex,
        "run": run, "verify": _verifier(run, extra_verify), "parse": _parser(key), "reference": reference,
        "wrong": wrong, "random_inputs": rand, "templates": [q["q"] for q in questions[:2]], "examples": exs,
        "questions": questions, "keywords": [p.lower() for p in phrases]}


def _U(lo, hi):
    return lambda rng: rng.uniform(lo, hi)


# =============================================================================================== reference helpers
def _metpy():
    import metpy.calc as mc
    from metpy.units import units
    return mc, units


def _ivp(fun, t1, y0, t0=0.0):
    from scipy.integrate import solve_ivp
    s = solve_ivp(fun, (t0, t1), y0, method="DOP853", rtol=1e-13, atol=1e-30)
    if not s.success:
        raise RuntimeError(s.message)
    return s.y[:, -1]


def _brentq(f, a, b):
    from scipy.optimize import brentq
    return brentq(f, a, b, xtol=1e-300, rtol=8.9e-16, maxiter=500)


def _shoot(f, a, b, guess):
    """Root of f by bracketing: first a narrow bracket around a guess (speed only), else the wide bracket [a, b]. The
    root itself is always the zero of f, whatever the guess."""
    lo, hi = max(a, guess * 0.9), min(b, guess * 1.1)
    try:
        if lo < hi and f(lo) * f(hi) < 0:
            return _brentq(f, lo, hi)
    except (ValueError, ArithmeticError):
        pass
    return _brentq(f, a, b)


# Outputs of the documented examples, computed by each capability's INDEPENDENT reference (MetPy, python-seawater,
# ObsPy, SciPy, NumPy, SymPy) on the build machine and rounded to 12 significant digits; certification checks the
# procedure against them.
EXAMPLE_OUTPUTS: dict = {
    "potential_temperature": [{"theta": 312.320747133}, {"theta": 320.783443104}],
    "temperature_from_potential_temperature": [{"T": 270.934026832}, {"T": 222.073531784}],
    "dry_adiabatic_ascent": [{"T2": 278.508346155, "lapse_rate": 0.00976110256342}, {"T2": 295.341117949, "lapse_rate": 0.00976110256342}],
    "saturation_vapor_pressure": [{"es": 3162.34559911}, {"es": 421.541182336}],
    "relative_humidity_from_dewpoint": [{"RH": 0.53855662174, "RH_percent": 53.855662174}, {"RH": 0.80920181557, "RH_percent": 80.920181557}],
    "dewpoint_from_vapor_pressure": [{"Td": 288.113055117}, {"Td": 269.002621114}],
    "dewpoint_from_relative_humidity": [{"Td": 294.506499951}, {"Td": 270.921005962}],
    "mixing_ratio": [{"w": 0.0126929981644, "w_g_per_kg": 12.6929981644}, {"w": 0.00590932931171, "w_g_per_kg": 5.90932931171}],
    "saturation_mixing_ratio": [{"ws": 0.0203107222033, "ws_g_per_kg": 20.3107222033}, {"ws": 0.00156495980144, "ws_g_per_kg": 1.56495980144}],
    "virtual_temperature": [{"Tv": 306.763003676}, {"Tv": 284.176480829}],
    "moist_air_density": [{"rho": 1.15805108355}, {"rho": 0.890620427609}],
    "specific_humidity_from_dewpoint": [{"q": 0.0106612033174}, {"q": 0.00640330928366}],
    "moist_static_energy": [{"h": 324211.265838, "s": 304204.545838}, {"h": 347570.351677, "s": 302555.231677}],
    "lifting_condensation_level": [{"p_lcl": 86421.3026232, "T_lcl": 290.812496352}, {"p_lcl": 88111.3406942, "T_lcl": 282.030676296}],
    "equivalent_potential_temperature": [{"theta_e": 345.675921305}, {"theta_e": 360.128056911}],
    "moist_adiabatic_lapse_rate": [{"lapse_rate": 0.00422188620157, "rs": 0.0148682638045}, {"lapse_rate": 0.00576250471357, "rs": 0.00547439448727}],
    "wind_chill": [{"WCT": 253.629501967}, {"WCT": 267.90777672}],
    "heat_index": [{"HI": 313.55927368}, {"HI": 307.877031507}],
    "coriolis_parameter": [{"f": 0.000103126079314}, {"f": -7.292115e-05}],
    "inertial_period": [{"T_inertial": 60927.2198554, "T_hours": 16.9242277376, "f": 0.000103126079314}, {"T_inertial": 86164.1006372, "T_hours": 23.9344723992, "f": 7.292115e-05}],
    "wind_components": [{"u": 7.07106781187, "v": 7.07106781187}, {"u": 12.9903810568, "v": -7.5}],
    "wind_speed_direction": [{"speed": 5.83095189485, "direction": 300.963756532}, {"speed": 10.0, "direction": 126.869897646}],
    "geostrophic_wind": [{"Vg": 11.4112551144, "f": 0.000103126079314}, {"Vg": 17.9310574961, "f": 7.292115e-05}],
    "brunt_vaisala_frequency": [{"N": 0.00990285312423, "period": 634.482328311}, {"N": 0.0205008536408, "period": 306.484081945}],
    "isothermal_pressure": [{"p": 54215.4042096, "H": 7995.29117083}, {"p": 76344.8197229, "H": 7409.87720994}],
    "scale_height": [{"H": 8434.3516415}, {"H": 6531.75626861}],
    "hypsometric_thickness": [{"dz": 5339.02451459}, {"dz": 1580.74927838}],
    "pressure_with_lapse_rate": [{"p": 70108.0426498, "T": 268.65}, {"p": 83909.2913111, "T": 286.15}],
    "standard_atmosphere_pressure": [{"p": 84547.6012724}, {"p": 30720.3142695}],
    "standard_atmosphere_height": [{"z": 5571.43442321}, {"z": 1456.51399628}],
    "geopotential_height": [{"phi": 97912.8150522, "Z": 9984.3284967}, {"phi": 486514.311009, "Z": 49610.6530782}],
    "rossby_wave_speed": [{"c": 0.239432065076, "beta": 1.61867740281e-11, "Ls": 6048467.42382}, {"c": 5.36120103069, "beta": 1.14457776808e-11, "Ls": 5872962.33141}],
    "blackbody_flux": [{"F": 390.105153555}, {"F": 227.769759728}],
    "wien_peak_wavelength": [{"lambda_max": 5.02039493275e-07, "lambda_um": 0.502039493275}, {"lambda_max": 1.00617081777e-05, "lambda_um": 10.0617081777}],
    "planetary_equilibrium_temperature": [{"Te": 254.578140115, "absorbed": 238.175}, {"Te": 210.075887783, "absorbed": 110.4375}],
    "greenhouse_single_layer": [{"Ts": 288.063776894, "Ta": 242.231797354}, {"Ts": 302.746135548, "Ta": 254.578140115}],
    "greenhouse_n_layers": [{"Ts": 335.04367447, "T_top": 254.578140115, "T_bottom_layer": 302.746135548}, {"Ts": 421.405626127, "T_top": 231.394034436, "T_bottom_layer": 411.483247044}],
    "daily_mean_insolation": [{"Q": 499.319173851, "day_length_h": 15.4258924132}, {"Q": 458.417584902, "day_length_h": 24.0}],
    "energy_balance_response": [{"dT": 1.8028327616, "dT_eq": 2.96, "tau_years": 10.6471975055}, {"dT": 0.67966614889, "dT_eq": 0.75, "tau_years": 12.6752351256}],
    "pure_water_density": [{"rho": 999.974957887}, {"rho": 997.046419849}],
    "seawater_density_surface": [{"rho": 1025.97196289, "sigma_t": 25.971962894}, {"rho": 1023.96790757, "sigma_t": 23.9679075727}],
    "seawater_density": [{"rho": 1046.01684344, "sigma": 46.0168434363}, {"rho": 1031.04291696, "sigma": 31.0429169638}],
    "seawater_freezing_point": [{"Tf": 271.2281599, "Tf_C": -1.92184009952}, {"Tf": 271.059599388, "Tf_C": -2.0904006123}],
    "sound_speed_seawater": [{"c": 1506.34678363}, {"c": 1542.66352879}],
    "ocean_pressure_from_depth": [{"p": 10095540.3006, "p_dbar": 1009.55403006}, {"p": 51080151.798, "p_dbar": 5108.0151798}],
    "ocean_depth_from_pressure": [{"depth": 990.808210695}, {"depth": 4902.08075214}],
    "seawater_adiabatic_gradient": [{"gamma": 0.000127387056532, "gamma_per_km_dbar": 0.127387056532}, {"gamma": 0.000115045531173, "gamma_per_km_dbar": 0.115045531173}],
    "seawater_potential_temperature": [{"theta": 275.663203213, "theta_C": 2.513203213, "dtheta": -0.486796786996}, {"theta": 282.898148087, "theta_C": 9.7481480875, "dtheta": -0.251851912504}],
    "gravity_at_latitude": [{"g": 9.80618987521}, {"g": 9.76804851022}],
    "shallow_water_wave": [{"c": 198.057062485, "travel_time": 25245.2497138, "travel_time_h": 7.01256936494}, {"c": 9.90285312423, "travel_time": 100.980998855, "travel_time_h": 0.0280502774598}],
    "ekman_depth": [{"D_E": 97.8285506131, "delta": 31.1397948112}, {"D_E": 52.028117933, "delta": 16.5610642976}],
    "ekman_transport": [{"M": 0.946035922813, "mass_transport": 969.686820883}, {"M": 2.67058277186, "mass_transport": 2742.6885067}],
    "seismic_velocities_from_moduli": [{"Vp": 5773.5026919, "Vs": 3333.33333333, "Vp_Vs": 1.73205080757}, {"Vp": 8226.58906697, "Vs": 4605.66186472, "Vp_Vs": 1.78619041272}],
    "seismic_velocities_from_young": [{"Vp": 5577.73351023, "Vs": 3220.3059436, "Vp_Vs": 1.73205080757}, {"Vp": 7872.95821622, "Vs": 4351.94139889, "Vp_Vs": 1.80906806747}],
    "elastic_constants_from_velocities": [{"mu": 32323320000.0, "lam": 32553360000.0, "nu": 0.250886451033}, {"mu": 67837500000.0, "lam": 84118500000.0, "nu": 0.276785714286}],
    "epicentral_distance": [{"delta": 85.953885479, "distance_km": 9557.63599066}, {"delta": 48.4392374298, "distance_km": 5386.19745273}],
    "geodesic_distance_azimuth": [{"distance": 9581233.26406, "azimuth": 336.249359942, "back_azimuth": 31.6632176332}, {"distance": 12547900.1954, "azimuth": 304.496547621, "back_azimuth": 115.584076484}],
    "moment_magnitude": [{"Mw": 7.99426179011, "M0_dyne_cm": 1.1e+28}, {"Mw": 5.66271202957, "M0_dyne_cm": 3.5e+24}],
    "local_magnitude": [{"ML": 4.0}, {"ML": 2.24744000867}],
    "gutenberg_richter_b_value": [{"b": 0.766402026888, "a": 2.61198529982, "n": 12.0}, {"b": 0.945987980383, "a": 3.87935662631, "n": 11.0}],
    "nmo_velocity": [{"V_nmo": 2500.0695575, "t0": 0.999994661227, "depth": 1250.0281051}, {"V_nmo": 2294.14340241, "t0": 0.615844277064, "depth": 706.41754257}],
    "dipole_inclination": [{"inclination": 63.4349488229, "F_ratio": 1.58113883008}, {"inclination": -36.0523887324, "F_ratio": 1.16229657804}],
    "paleolatitude": [{"paleolatitude": 40.8933946491, "pole_distance": 49.1066053509}, {"paleolatitude": -16.102113752, "pole_distance": 106.102113752}],
    "plate_velocity": [{"speed": 52.79395817, "azimuth": 123.020116662}, {"speed": 32.7921105437, "azimuth": 235.508434817}],
    "bouguer_slab": [{"dg_mGal": 11.1968756068, "dg": 0.000111968756068}, {"dg_mGal": -33.5486909566, "dg": -0.000335486909566}],
    "buried_sphere_anomaly": [{"dg_mGal": 0.349465530798, "dg": 3.49465530798e-06}, {"dg_mGal": -0.931908082127, "dg": -9.31908082127e-06}],
    "geotherm_radiogenic": [{"T": 523.15, "q": 0.055}, {"T": 688.15, "q": 0.04}],
    "seafloor_depth_age": [{"depth": 5008.08791695, "subsidence": 2508.08791695}, {"depth": 6147.51667693, "subsidence": 3547.51667693}],
    "radioactive_decay_remaining": [{"fraction": 0.298292436424, "decayed": 0.701707563576, "lambda": 0.000120968094339}, {"fraction": 0.0824692444233, "decayed": 0.917530755577, "lambda": 5.54517744448e-10}],
    "radiometric_age": [{"age": 731203125.901, "lambda": 5.54517744448e-10}, {"age": 8936000000.0, "lambda": 1.5513589538e-10}],
    "potassium_argon_age": [{"age": 92264335.5068}, {"age": 1376639626.86}],
    "isochron_age": [{"age": 996371909.389, "slope": 0.014249044586, "initial_ratio": 0.70498955414}, {"age": 900876502.015, "slope": 0.0128746196958, "initial_ratio": 0.701950344275}],
    "radiocarbon_age": [{"age": 5568.05130144}, {"age": 11136.1026029}],
    "stokes_settling_velocity": [{"w": 0.00224735729167, "Re": 0.112367864583}, {"w": 7.3776880787e-05, "Re": 0.000630177523389}],
}


# =============================================================================================== moist thermodynamics core
def _latent(T):
    return LV0 - (CPL - CPV) * (T - T0TP)


def _es(T):
    """Saturation vapour pressure over liquid water (Ambaum 2020, Eq. 13, MetPy constants), Pa."""
    return ES0 * (T0TP / T) ** ((CPL - CPV) / RV) * math.exp((LV0 / T0TP - _latent(T) / T) / RV)


def _es_bolton(T):
    c = T - 273.15
    return 611.2 * math.exp(17.67 * c / (c + 243.5))


def _dewpoint_bolton(e):
    v = math.log(e / ES0)
    return 273.15 + 243.5 * v / (17.67 - v)


def _ws(p, T):
    e = _es(T)
    _need(e < p, "the saturation vapour pressure must be below the total pressure")
    return EPS * e / (p - e)


def _T_ok(T, lo=228.15, hi=333.15, name="temperature"):
    _finite(T=T)
    _need(lo <= T <= hi, f"{name} must lie between {lo - 273.15:g} and {hi - 273.15:g} °C")


def _p_ok(p, lo=1000.0, hi=110000.0, name="pressure"):
    _finite(p=p)
    _need(lo <= p <= hi, f"{name} must lie between {lo / 100:g} and {hi / 100:g} hPa")


_TEMP = lambda sym, meaning, names=("temperature",), show="°C", **kw: I(sym, "temperature", "temperature", meaning, names, show, **kw)  # noqa: E731
_PRES = lambda sym, meaning, names=("pressure",), show="hPa", **kw: I(sym, "pressure", "pressure", meaning, names, show, **kw)  # noqa: E731


# --- potential temperature ----------------------------------------------------------------------------------------
def potential_temperature(T, p):
    _T_ok(T, 150, 350)
    _p_ok(p, 100, 110000)
    return {"theta": _r(T * (P0 / p) ** KAPPA)}


PT_IN = [_TEMP("T", "air temperature"), _PRES("p", "air pressure")]
proc("potential_temperature", "Potential temperature", "atmospheric thermodynamics",
     "The temperature an air parcel would have if brought dry-adiabatically to 1000 hPa (Poisson's equation).",
     PT_IN, [O("theta", "temperature", "potential temperature (K)")], potential_temperature,
     lambda T, p: (lambda mc, u: {"theta": mc.potential_temperature(p * u.Pa, T * u.K).m_as("K")})(*_metpy()),
     lambda T, p: {"theta": _r(T * (p / P0) ** KAPPA)},
     lambda rng: {"T": rng.uniform(200, 320), "p": rng.uniform(15000, 105000)},
     [{"T": 25, "p": 850}, {"T": -10, "p": 500}], ["potential temperature", "theta of air"],
     conditions=["150 K <= T <= 350 K", "1 hPa <= p <= 1100 hPa"], method="Poisson's equation with kappa = R_d/c_pd",
     latex=r"\theta = T\left(\frac{p_0}{p}\right)^{R_d/c_{pd}},\quad p_0 = 1000\ \mathrm{hPa}")


def temperature_from_theta(theta, p):
    _T_ok(theta, 150, 600, "potential temperature")
    _p_ok(p, 100, 110000)
    return {"T": _r(theta * (p / P0) ** KAPPA)}


proc("temperature_from_potential_temperature", "Temperature from potential temperature", "atmospheric thermodynamics",
     "The air temperature at pressure p of a parcel with a given potential temperature.",
     [I("theta", "temperature", "temperature", "potential temperature", ("potential temperature",), "K"),
      _PRES("p", "air pressure")],
     [O("T", "temperature", "air temperature (K)")], temperature_from_theta,
     lambda theta, p: (lambda mc, u: {"T": mc.temperature_from_potential_temperature(p * u.Pa, theta * u.K).m_as("K")})(*_metpy()),
     lambda theta, p: {"T": _r(theta * (P0 / p) ** KAPPA)},
     lambda rng: {"theta": rng.uniform(260, 420), "p": rng.uniform(15000, 105000)},
     [{"theta": 300, "p": 700}, {"theta": 330, "p": 250}], ["temperature from potential temperature"],
     conditions=["150 K <= theta <= 600 K", "1 hPa <= p <= 1100 hPa"], method="Poisson's equation inverted",
     latex=r"T = \theta\left(\frac{p}{p_0}\right)^{R_d/c_{pd}}")


# --- dry adiabatic ascent --------------------------------------------------------------------------------------------
def dry_adiabatic_ascent(T, dz):
    _T_ok(T, 150, 350)
    _finite(dz=dz)
    _need(0 < dz <= 20000, "the ascent dz must be between 0 and 20 km")
    gam = G0 / CPD
    T2 = T - gam * dz
    _need(T2 > 100, "the parcel would cool below 100 K")
    return {"T2": _r(T2), "lapse_rate": _r(gam)}


def _ref_dry_ascent(T, dz):
    # hydrostatic balance dp/dz = -p g/(R_d T) integrated with T(p) from Poisson's equation (MetPy's kappa); the
    # parcel temperature at the top comes from the pressure reached, not from the lapse-rate formula
    p1 = 100000.0
    Tp = lambda p: T * (p / p1) ** KAPPA                                   # noqa: E731
    p2 = _ivp(lambda z, y: [-y[0] * G0 / (RD * Tp(y[0]))], dz, [p1])[0]
    T2 = Tp(p2)
    return {"T2": T2, "lapse_rate": (T - T2) / dz}


proc("dry_adiabatic_ascent", "Temperature of a dry parcel after adiabatic ascent", "atmospheric thermodynamics",
     "The temperature of an unsaturated air parcel lifted dry-adiabatically through a height dz, and the dry "
     "adiabatic lapse rate g/c_pd.",
     [_TEMP("T", "starting temperature of the parcel"),
      I("dz", "length", "length", "height through which the parcel rises", ("height gain", "ascent"), "m")],
     [O("T2", "temperature", "parcel temperature after the ascent (K)"),
      O("lapse_rate", "lapse_rate", "dry adiabatic lapse rate g/c_pd (K/m)")],
     dry_adiabatic_ascent, _ref_dry_ascent,
     lambda T, dz: {"T2": _r(T - G0 / 717.6187274429616 * dz), "lapse_rate": _r(G0 / 717.6187274429616)},
     lambda rng: {"T": rng.uniform(250, 310), "dz": rng.uniform(100, 6000)},
     [{"T": 20, "dz": 1500}, {"T": 30, "dz": 800}], ["dry adiabatic ascent", "dry adiabatic lapse rate",
                                                     "dry adiabatic lifting"],
     conditions=["0 < dz <= 20 km", "the parcel stays unsaturated"],
     method="dry adiabatic lapse rate g/c_pd times the height gained",
     latex=r"T_2 = T - \Gamma_d\,\Delta z,\quad \Gamma_d = g/c_{pd}")


# --- saturation vapour pressure, humidity -------------------------------------------------------------------------
def saturation_vapor_pressure(T):
    _T_ok(T)
    return {"es": _r(_es(T))}


proc("saturation_vapor_pressure", "Saturation vapour pressure over water", "atmospheric moisture",
     "The equilibrium vapour pressure over a plane surface of liquid water (Ambaum 2020, latent heat varying with "
     "temperature).",
     [_TEMP("T", "temperature")], [O("es", "pressure", "saturation vapour pressure (Pa)")],
     saturation_vapor_pressure,
     lambda T: (lambda mc, u: {"es": mc.saturation_vapor_pressure(T * u.K).m_as("Pa")})(*_metpy()),
     lambda T: {"es": _r(_es_bolton(T) * 1.02)},
     lambda rng: {"T": rng.uniform(230, 330)},
     [{"T": 25}, {"T": -5}], ["saturation vapour pressure", "saturation vapor pressure"],
     conditions=["-45 °C <= T <= 60 °C (liquid water, possibly supercooled)"],
     method="Ambaum (2020) Eq. 13 with L(T) = L0 - (c_pl - c_pv)(T - T0)", invalid=[{"T": 90}],
     latex=r"e_s = e_{s0}\left(\frac{T_0}{T}\right)^{(c_{pl}-c_{pv})/R_v}\exp\!\left(\frac{L_0}{R_vT_0}-\frac{L(T)}{R_vT}\right)")


def relative_humidity(T, Td):
    _T_ok(T)
    _T_ok(Td, name="dewpoint")
    _need(Td <= T, "the dewpoint cannot exceed the temperature")
    rh = _es(Td) / _es(T)
    return {"RH": _r(rh), "RH_percent": _r(100 * rh)}


proc("relative_humidity_from_dewpoint", "Relative humidity from temperature and dewpoint", "atmospheric moisture",
     "Relative humidity as the ratio of the vapour pressure at the dewpoint to the saturation vapour pressure.",
     [_TEMP("T", "air temperature"), _TEMP("Td", "dewpoint temperature", ("dewpoint", "dew point"))],
     [O("RH", "number", "relative humidity (fraction)"), O("RH_percent", "number", "relative humidity (%)")],
     relative_humidity,
     lambda T, Td: (lambda mc, u: (lambda r: {"RH": r, "RH_percent": 100 * r})(
         mc.relative_humidity_from_dewpoint(T * u.K, Td * u.K).m_as("dimensionless")))(*_metpy()),
     lambda T, Td: {"RH": _r(_es(T) / _es(Td)), "RH_percent": _r(100 * _es(T) / _es(Td))},
     lambda rng: (lambda T: {"T": T, "Td": T - rng.uniform(0.5, 25)})(rng.uniform(260, 320)),
     [{"T": 25, "Td": 15}, {"T": 5, "Td": 2}], ["relative humidity from dewpoint", "relative humidity"],
     conditions=["Td <= T", "-45 °C <= T, Td <= 60 °C"], method="e_s(Td)/e_s(T), Ambaum (2020) e_s",
     invalid=[{"T": 10, "Td": 15}], latex=r"RH = \frac{e_s(T_d)}{e_s(T)}")


def dewpoint_from_vapor_pressure(e):
    _finite(e=e)
    _need(10 <= e <= 20000, "the vapour pressure must lie between 0.1 and 200 hPa")
    return {"Td": _r(_dewpoint_bolton(e))}


proc("dewpoint_from_vapor_pressure", "Dewpoint from vapour pressure", "atmospheric moisture",
     "The dewpoint at which the given vapour pressure would saturate (inversion of Bolton's 1980 formula).",
     [_PRES("e", "water vapour partial pressure", ("vapour pressure", "vapor pressure"))],
     [O("Td", "temperature", "dewpoint (K)")], dewpoint_from_vapor_pressure,
     lambda e: (lambda mc, u: {"Td": mc.dewpoint(e * u.Pa).m_as("K")})(*_metpy()),
     lambda e: (lambda v: {"Td": _r(273.15 + 237.3 * v / (17.27 - v))})(math.log(e / ES0)),
     lambda rng: {"e": rng.uniform(100, 7000)},
     [{"e": 17}, {"e": 4.5}], ["dewpoint from vapour pressure", "dewpoint from vapor pressure",
                               "dew point from vapour pressure", "dew point from vapor pressure"],
     conditions=["0.1 hPa <= e <= 200 hPa"], method="Td = 243.5 v/(17.67 - v) °C with v = ln(e/6.112 hPa)",
     latex=r"T_d = \frac{243.5\,\ln(e/6.112)}{17.67-\ln(e/6.112)}\ ^\circ\mathrm{C}")


def dewpoint_from_rh(T, RH):
    _T_ok(T)
    _finite(RH=RH)
    _need(0.01 <= RH <= 1.0, "relative humidity must be a fraction between 0.01 and 1 (1% to 100%)")
    return {"Td": _r(_dewpoint_bolton(RH * _es(T)))}


proc("dewpoint_from_relative_humidity", "Dewpoint from temperature and relative humidity", "atmospheric moisture",
     "The dewpoint of air at temperature T and relative humidity RH: Bolton's inversion applied to RH e_s(T).",
     [_TEMP("T", "air temperature"), I("RH", "number", "fraction", "relative humidity (fraction, or % in a question)",
                                       ("relative humidity", "humidity"), "%")],
     [O("Td", "temperature", "dewpoint (K)")], dewpoint_from_rh,
     lambda T, RH: (lambda mc, u: {"Td": mc.dewpoint_from_relative_humidity(T * u.K, RH * u.dimensionless).m_as("K")})(*_metpy()),
     lambda T, RH: {"Td": _r(_dewpoint_bolton(RH * _es_bolton(T) * 1.01))},
     lambda rng: {"T": rng.uniform(250, 320), "RH": rng.uniform(0.05, 1.0)},
     [{"T": 30, "RH": 60}, {"T": 0, "RH": 85}], ["dewpoint from relative humidity", "dew point from relative humidity"],
     conditions=["0.01 <= RH <= 1"], invalid=[{"T": 20, "RH": 150}],
     method="Td = Bolton inversion of RH e_s(T), e_s from Ambaum (2020) (MetPy's convention)",
     latex=r"T_d = T_d^{\mathrm{Bolton}}\!\left(RH\,e_s(T)\right)")


def mixing_ratio(e, p):
    _finite(e=e, p=p)
    _need(0 < e < p, "the vapour pressure must be positive and below the total pressure")
    _p_ok(p, 100, 110000)
    w = EPS * e / (p - e)
    return {"w": _r(w), "w_g_per_kg": _r(1000 * w)}


proc("mixing_ratio", "Water-vapour mixing ratio from vapour pressure", "atmospheric moisture",
     "Mass of water vapour per mass of dry air from the vapour partial pressure and the total pressure.",
     [_PRES("e", "water vapour partial pressure", ("vapour pressure", "vapor pressure")), _PRES("p", "total air pressure")],
     [O("w", "number", "mixing ratio (kg/kg)"), O("w_g_per_kg", "number", "mixing ratio (g/kg)")], mixing_ratio,
     lambda e, p: (lambda mc, u: (lambda w: {"w": w, "w_g_per_kg": 1000 * w})(
         mc.mixing_ratio(e * u.Pa, p * u.Pa).m_as("dimensionless")))(*_metpy()),
     lambda e, p: {"w": _r(EPS * e / p), "w_g_per_kg": _r(1000 * EPS * e / p)},
     lambda rng: {"e": rng.uniform(100, 5000), "p": rng.uniform(50000, 105000)},
     [{"e": 20, "p": 1000}, {"e": 8, "p": 850}], ["mixing ratio from vapour pressure", "mixing ratio from vapor pressure",
                                                   "mixing ratio"],
     conditions=["0 < e < p"], method="w = epsilon e/(p - e)", latex=r"w = \frac{\varepsilon\,e}{p-e}")


def saturation_mixing_ratio(p, T):
    _T_ok(T)
    _p_ok(p, 100, 110000)
    w = _ws(p, T)
    return {"ws": _r(w), "ws_g_per_kg": _r(1000 * w)}


proc("saturation_mixing_ratio", "Saturation mixing ratio", "atmospheric moisture",
     "The mixing ratio of air saturated with respect to liquid water at temperature T and pressure p.",
     [_PRES("p", "total air pressure"), _TEMP("T", "air temperature")],
     [O("ws", "number", "saturation mixing ratio (kg/kg)"), O("ws_g_per_kg", "number", "saturation mixing ratio (g/kg)")],
     saturation_mixing_ratio,
     lambda p, T: (lambda mc, u: (lambda w: {"ws": w, "ws_g_per_kg": 1000 * w})(
         mc.saturation_mixing_ratio(p * u.Pa, T * u.K).m_as("dimensionless")))(*_metpy()),
     lambda p, T: {"ws": _r(EPS * _es(T) / p), "ws_g_per_kg": _r(1000 * EPS * _es(T) / p)},
     lambda rng: {"p": rng.uniform(30000, 105000), "T": rng.uniform(240, 310)},
     [{"p": 1000, "T": 25}, {"p": 500, "T": -20}], ["saturation mixing ratio"],
     conditions=["e_s(T) < p"], method="w_s = epsilon e_s/(p - e_s), Ambaum (2020) e_s",
     latex=r"w_s = \frac{\varepsilon\,e_s(T)}{p-e_s(T)}")


def virtual_temperature(T, w):
    _T_ok(T, 150, 350)
    _finite(w=w)
    _need(0 <= w <= 0.06, "the mixing ratio must be a fraction between 0 and 0.06 kg/kg")
    return {"Tv": _r(T * (w + EPS) / (EPS * (1 + w)))}


proc("virtual_temperature", "Virtual temperature", "atmospheric thermodynamics",
     "The temperature dry air would need to have the density of the moist air at the same pressure.",
     [_TEMP("T", "air temperature"), I("w", "number", "number", "water-vapour mixing ratio (kg/kg)", ("mixing ratio",), "")],
     [O("Tv", "temperature", "virtual temperature (K)")], virtual_temperature,
     lambda T, w: (lambda mc, u: {"Tv": mc.virtual_temperature(T * u.K, w * u.dimensionless).m_as("K")})(*_metpy()),
     lambda T, w: {"Tv": _r(T * (1 + w))},
     lambda rng: {"T": rng.uniform(230, 315), "w": rng.uniform(0.001, 0.03)},
     [{"T": 30, "w": 0.02}, {"T": 10, "w": 0.006}], ["virtual temperature"],
     conditions=["0 <= w <= 0.06"], method="Tv = T (w + epsilon)/(epsilon (1 + w))",
     latex=r"T_v = T\,\frac{w+\varepsilon}{\varepsilon(1+w)}")


def moist_air_density(p, T, w):
    _T_ok(T, 150, 350)
    _p_ok(p, 100, 110000)
    _finite(w=w)
    _need(0 <= w <= 0.06, "the mixing ratio must be a fraction between 0 and 0.06 kg/kg")
    Tv = T * (w + EPS) / (EPS * (1 + w))
    return {"rho": _r(p / (RD * Tv)), "Tv": _r(Tv)}


proc("moist_air_density", "Density of moist air", "atmospheric thermodynamics",
     "Density of moist air from the ideal-gas law with the virtual temperature.",
     [_PRES("p", "air pressure"), _TEMP("T", "air temperature"),
      I("w", "number", "number", "water-vapour mixing ratio (kg/kg)", ("mixing ratio",), "")],
     [O("rho", "density", "air density (kg/m^3)"), O("Tv", "temperature", "virtual temperature (K)")],
     moist_air_density,
     lambda p, T, w: (lambda mc, u: {"rho": mc.density(p * u.Pa, T * u.K, w * u.dimensionless).m_as("kg/m^3")})(*_metpy()),
     lambda p, T, w: {"rho": _r(p / (RD * T)), "Tv": _r(T)},
     lambda rng: {"p": rng.uniform(30000, 105000), "T": rng.uniform(230, 315), "w": rng.uniform(0.002, 0.03)},
     [{"p": 1000, "T": 25, "w": 0.015}, {"p": 700, "T": 0, "w": 0.004}], ["density of moist air", "moist air density",
                                                                          "air density"],
     conditions=["0 <= w <= 0.06"], method="rho = p/(R_d T_v)", latex=r"\rho = \frac{p}{R_d\,T_v}")


def specific_humidity_from_dewpoint(p, Td):
    _T_ok(Td, name="dewpoint")
    _p_ok(p, 100, 110000)
    w = _ws(p, Td)
    return {"q": _r(w / (1 + w)), "w": _r(w)}


proc("specific_humidity_from_dewpoint", "Specific humidity from dewpoint and pressure", "atmospheric moisture",
     "Specific humidity (mass of vapour per mass of moist air) from the dewpoint and the pressure.",
     [_PRES("p", "air pressure"), _TEMP("Td", "dewpoint temperature", ("dewpoint", "dew point"))],
     [O("q", "number", "specific humidity (kg/kg)"), O("w", "number", "mixing ratio (kg/kg)")],
     specific_humidity_from_dewpoint,
     lambda p, Td: (lambda mc, u: {"q": mc.specific_humidity_from_dewpoint(p * u.Pa, Td * u.K).m_as("dimensionless")})(*_metpy()),
     lambda p, Td: {"q": _r(_ws(p, Td)), "w": _r(_ws(p, Td))},
     lambda rng: {"p": rng.uniform(40000, 105000), "Td": rng.uniform(240, 300)},
     [{"p": 1000, "Td": 15}, {"p": 850, "Td": 5}], ["specific humidity from dewpoint", "specific humidity from dew point",
                                                    "specific humidity"],
     conditions=["e_s(Td) < p"], method="q = w_s(p, Td)/(1 + w_s(p, Td))", latex=r"q = \frac{w_s(p,T_d)}{1+w_s(p,T_d)}")


def moist_static_energy(z, T, q):
    _T_ok(T, 150, 350)
    _finite(z=z, q=q)
    _need(-500 <= z <= 30000, "height must be between -500 m and 30 km")
    _need(0 <= q <= 0.06, "specific humidity must be a fraction between 0 and 0.06 kg/kg")
    s = G0 * z + CPD * T
    return {"h": _r(s + LV0 * q), "s": _r(s)}


proc("moist_static_energy", "Moist static energy", "atmospheric thermodynamics",
     "Moist static energy c_pd T + g z + L_v q (and the dry static energy c_pd T + g z) per unit mass.",
     [I("z", "length", "length", "height above sea level", ("height",), "m"), _TEMP("T", "air temperature"),
      I("q", "number", "number", "specific humidity (kg/kg)", ("specific humidity",), "")],
     [O("h", "specific_energy", "moist static energy (J/kg)"), O("s", "specific_energy", "dry static energy (J/kg)")],
     moist_static_energy,
     lambda z, T, q: (lambda mc, u: {"h": mc.moist_static_energy(z * u.m, T * u.K, q * u.dimensionless).m_as("J/kg"),
                                     "s": mc.dry_static_energy(z * u.m, T * u.K).m_as("J/kg")})(*_metpy()),
     lambda z, T, q: {"h": _r(G0 * z + CPD * T), "s": _r(G0 * z + CPD * T)},
     lambda rng: {"z": rng.uniform(0, 12000), "T": rng.uniform(210, 310), "q": rng.uniform(0.001, 0.02)},
     [{"z": 1500, "T": 15, "q": 0.008}, {"z": 0, "T": 28, "q": 0.018}], ["moist static energy"],
     conditions=["0 <= q <= 0.06"], method="h = c_pd T + g z + L_v q (MetPy's L_v = 2.50084e6 J/kg)",
     latex=r"h = c_{pd}T + g z + L_v q")


# --- lifting condensation level (Romps 2017) -------------------------------------------------------------------------
def _lambertw_m1(x):
    """The k = -1 real branch of the Lambert W function on [-1/e, 0) (Halley's iteration)."""
    _need(-1 / math.e <= x < 0, "the Lambert W argument is outside [-1/e, 0)")
    if x <= -1 / math.e + 1e-300:
        return -1.0
    if x < -0.25:
        p = -math.sqrt(2 * (1 + math.e * x))
        w = -1 + p - p * p / 3 + 11 / 72 * p ** 3
    else:
        l1 = math.log(-x)
        l2 = math.log(-l1)
        w = l1 - l2 + l2 / l1
    for _ in range(100):
        ew = math.exp(w)
        f = w * ew - x
        wn = w - f / (ew * (w + 1) - (w + 2) * f / (2 * w + 2))
        if abs(wn - w) <= 4e-16 * abs(wn):
            return wn
        w = wn
    return w


def _lcl(p, T, Td, rh_power=None):
    w = _ws(p, Td)
    q = w / (1 + w)
    cpm = CPD + q * (CPV - CPD)
    rm = RD + q * (RV - RD)
    mhr = cpm / rm
    sd = CPL - CPV
    a = mhr + sd / RV
    b = -(LV0 + sd * T0TP) / (RV * T)
    c = b / a
    rh = _es(Td) / _es(T)
    wm1 = _lambertw_m1(rh ** (1 / a if rh_power is None else rh_power(a)) * c * math.exp(c))
    tl = c / wm1 * T
    return p * (tl / T) ** mhr, tl


def lcl(p, T, Td):
    _T_ok(T)
    _T_ok(Td, name="dewpoint")
    _p_ok(p, 10000, 110000)
    _need(Td <= T, "the dewpoint cannot exceed the temperature")
    pl, tl = _lcl(p, T, Td)
    return {"p_lcl": _r(pl), "T_lcl": _r(tl)}


proc("lifting_condensation_level", "Lifting condensation level (Romps 2017)", "atmospheric thermodynamics",
     "Pressure and temperature at which a parcel lifted dry-adiabatically from (p, T, Td) becomes saturated, from "
     "Romps' exact solution with the Lambert W function.",
     [_PRES("p", "starting pressure"), _TEMP("T", "starting temperature"),
      _TEMP("Td", "starting dewpoint", ("dewpoint", "dew point"))],
     [O("p_lcl", "pressure", "LCL pressure (Pa)"), O("T_lcl", "temperature", "LCL temperature (K)")], lcl,
     lambda p, T, Td: (lambda mc, u: (lambda r: {"p_lcl": r[0].m_as("Pa"), "T_lcl": r[1].m_as("K")})(
         mc.lcl(p * u.Pa, T * u.K, Td * u.K)))(*_metpy()),
     lambda p, T, Td: (lambda r: {"p_lcl": _r(r[0]), "T_lcl": _r(r[1])})(_lcl(p, T, Td, rh_power=lambda a: a)),
     lambda rng: (lambda T: {"p": rng.uniform(70000, 104000), "T": T, "Td": T - rng.uniform(1, 20)})(rng.uniform(270, 310)),
     [{"p": 1000, "T": 30, "Td": 20}, {"p": 950, "T": 15, "Td": 10}],
     ["lifting condensation level", "lifted condensation level", "lcl"],
     conditions=["Td <= T", "100 hPa <= p <= 1100 hPa"], invalid=[{"p": 1000, "T": 10, "Td": 12}],
     method="Romps (2017) Eq. 22: T_LCL = c/W_{-1}(RH^{1/a} c e^c) T, p_LCL = p (T_LCL/T)^{c_pm/R_m}",
     latex=r"T_{LCL} = \frac{c}{W_{-1}\!\left(RH^{1/a}\,c\,e^{c}\right)}\,T,\quad p_{LCL} = p\left(\frac{T_{LCL}}{T}\right)^{c_{pm}/R_m}")


# --- equivalent potential temperature (Bolton 1980) ------------------------------------------------------------------
def _theta_e(p, T, Td, use_partial=True):
    r = _ws(p, Td)
    e = _es(Td)
    tl = 56 + 1.0 / (1.0 / (Td - 56) + math.log(T / Td) / 800.0)
    thl = T * (P0 / ((p - e) if use_partial else p)) ** KAPPA * (T / tl) ** (0.28 * r)
    return thl * math.exp(r * (1 + 0.448 * r) * (3036.0 / tl - 1.78))


def equivalent_potential_temperature(p, T, Td):
    _T_ok(T)
    _T_ok(Td, name="dewpoint")
    _p_ok(p, 10000, 110000)
    _need(Td <= T, "the dewpoint cannot exceed the temperature")
    return {"theta_e": _r(_theta_e(p, T, Td))}


proc("equivalent_potential_temperature", "Equivalent potential temperature (Bolton 1980)", "atmospheric thermodynamics",
     "The potential temperature a parcel would have after all its vapour condensed and the latent heat warmed it.",
     [_PRES("p", "air pressure"), _TEMP("T", "air temperature"), _TEMP("Td", "dewpoint", ("dewpoint", "dew point"))],
     [O("theta_e", "temperature", "equivalent potential temperature (K)")], equivalent_potential_temperature,
     lambda p, T, Td: (lambda mc, u: {"theta_e": mc.equivalent_potential_temperature(p * u.Pa, T * u.K, Td * u.K).m_as("K")})(*_metpy()),
     lambda p, T, Td: {"theta_e": _r(_theta_e(p, T, Td, use_partial=False) * 1.0)},
     lambda rng: (lambda T: {"p": rng.uniform(50000, 104000), "T": T, "Td": T - rng.uniform(0.5, 20)})(rng.uniform(260, 310)),
     [{"p": 850, "T": 20, "Td": 15}, {"p": 1000, "T": 30, "Td": 24}], ["equivalent potential temperature", "theta e"],
     conditions=["Td <= T", "100 hPa <= p <= 1100 hPa"],
     method="Bolton (1980): T_L from T and Td, theta_L with the dry-air partial pressure, then the exponential factor",
     latex=r"\theta_e = T\left(\frac{p_0}{p-e}\right)^{\kappa}\left(\frac{T}{T_L}\right)^{0.28r}"
           r"\exp\!\left[r(1+0.448r)\left(\frac{3036}{T_L}-1.78\right)\right]")


# --- moist (saturated) adiabatic lapse rate ---------------------------------------------------------------------------
def moist_lapse_rate(T, p):
    _T_ok(T, 233.15, 323.15)
    _p_ok(p, 10000, 110000)
    rs = _ws(p, T)
    gam = G0 * (1 + LV0 * rs / (RD * T)) / (CPD + LV0 * LV0 * rs / (RV * T * T))
    return {"lapse_rate": _r(gam), "rs": _r(rs)}


@lru_cache(maxsize=1)
def _moist_lapse_derived():
    """dT/dz from the first law of a saturated parcel, c_p dT + L_v dr_s + g dz = 0, with r_s = eps e_s/p,
    Clausius-Clapeyron d ln e_s = L_v dT/(R_v T^2) and hydrostatic d ln p = -g dz/(R_d T), solved by SymPy."""
    import sympy as sp
    dT, dz, cp, Lv, rs, Rv, T, g, Rd = sp.symbols("dT dz c_p L_v r_s R_v T g R_d")
    drs = rs * (Lv * dT / (Rv * T ** 2) + g * dz / (Rd * T))
    sol = sp.solve(sp.Eq(cp * dT + Lv * drs + g * dz, 0), dT)[0]
    return sp.lambdify((cp, Lv, rs, Rv, T, g, Rd), sp.simplify(-sol / dz))


def _ref_moist_lapse(T, p):
    mc, u = _metpy()
    rs = mc.saturation_mixing_ratio(p * u.Pa, T * u.K).m_as("dimensionless")
    return {"lapse_rate": float(_moist_lapse_derived()(CPD, LV0, rs, RV, T, G0, RD)), "rs": rs}


proc("moist_adiabatic_lapse_rate", "Saturated (moist) adiabatic lapse rate", "atmospheric thermodynamics",
     "The rate at which a saturated parcel cools as it rises, with condensation releasing latent heat.",
     [_TEMP("T", "air temperature"), _PRES("p", "air pressure")],
     [O("lapse_rate", "lapse_rate", "moist adiabatic lapse rate (K/m)"), O("rs", "number", "saturation mixing ratio (kg/kg)")],
     moist_lapse_rate, _ref_moist_lapse,
     lambda T, p: {"lapse_rate": _r(G0 / CPD), "rs": _r(_ws(p, T))},
     lambda rng: {"T": rng.uniform(240, 310), "p": rng.uniform(40000, 104000)},
     [{"T": 20, "p": 1000}, {"T": 0, "p": 700}], ["moist adiabatic lapse rate", "saturated adiabatic lapse rate",
                                                 "pseudoadiabatic lapse rate"],
     conditions=["-40 °C <= T <= 50 °C", "constant latent heat L_v = 2.50084e6 J/kg in the lapse-rate formula"],
     method="Gamma_m = g (1 + L_v r_s/(R_d T)) / (c_pd + L_v^2 r_s/(R_v T^2)), r_s from Ambaum (2020) e_s",
     latex=r"\Gamma_m = g\,\frac{1+\dfrac{L_v r_s}{R_d T}}{c_{pd}+\dfrac{L_v^2 r_s}{R_v T^2}}")


# --- comfort indices -------------------------------------------------------------------------------------------------
def wind_chill(T, V):
    _finite(T=T, V=V)
    _need(T <= 283.15, "the wind chill index is defined for temperatures of 10 °C (50 °F) or below")
    _need(V > 1.34112, "the wind chill index is defined for wind speeds above 3 mph (4.8 km/h)")
    _need(T >= 200 and V < 60, "temperature or wind speed out of range")
    s = (V * 3.6) ** 0.16
    c = T - 273.15
    w = (0.6215 + 0.3965 * s) * c - 11.37 * s + 13.12
    return {"WCT": _r(w + 273.15)}


proc("wind_chill", "Wind chill temperature index", "applied meteorology",
     "The wind chill temperature index of the 2001 North American formula (wind measured at 10 m).",
     [_TEMP("T", "air temperature"), I("V", "speed", "speed", "wind speed at 10 m", ("wind speed", "wind"), "km/h")],
     [O("WCT", "temperature", "wind chill temperature (K)")], wind_chill,
     lambda T, V: (lambda mc, u: {"WCT": float(mc.windchill(T * u.K, V * u("m/s"), mask_undefined=False).m_as("K"))})(*_metpy()),
     lambda T, V: (lambda s, c: {"WCT": _r((0.6215 + 0.3965 * s) * c - 11.37 * s + 13.12 + 273.15)})(V ** 0.16, T - 273.15),
     lambda rng: {"T": rng.uniform(233.15, 283.15), "V": rng.uniform(2, 30)},
     [{"T": -10, "V": 30}, {"T": 0, "V": 20}], ["wind chill"],
     conditions=["T <= 10 °C", "V > 3 mph (4.8 km/h)"], invalid=[{"T": 20, "V": 30}],
     method="WCT = 13.12 + 0.6215 T - 11.37 V^0.16 + 0.3965 T V^0.16 (T in °C, V in km/h)",
     latex=r"WCT = 13.12 + 0.6215\,T - 11.37\,V^{0.16} + 0.3965\,T\,V^{0.16}")


def _heat_index_F(TF, RH):
    a = -10.3 + 1.1 * TF + 4.7 * RH
    if TF <= 40:
        hi = TF
    elif a < 79:
        hi = a
    else:
        hi = (-42.379 + 2.04901523 * TF + 1014.333127 * RH - 22.475541 * TF * RH - 6.83783e-3 * TF * TF
              - 5.481717e2 * RH * RH + 1.22874e-1 * TF * TF * RH + 8.5282 * TF * RH * RH - 1.99e-2 * TF * TF * RH * RH)
    if RH <= 0.13 and 80 <= TF <= 112:
        hi -= (13.0 - RH * 100.0) / 4.0 * math.sqrt((17.0 - abs(TF - 95.0)) / 17.0)
    if RH > 0.85 and 80 <= TF <= 87:
        hi += 0.02 * (RH * 100.0 - 85.0) * (87.0 - TF)
    return hi


def heat_index(T, RH):
    _finite(T=T, RH=RH)
    TF = (T - 273.15) * 9 / 5 + 32
    _need(80 <= TF <= 130, "the heat index is defined for temperatures from 80 °F (26.7 °C) to 130 °F")
    _need(0 <= RH <= 1, "relative humidity must be a fraction between 0 and 1 (0% to 100%)")
    hf = _heat_index_F(TF, RH)
    return {"HI": _r((hf - 32) * 5 / 9 + 273.15)}


proc("heat_index", "Heat index (NWS)", "applied meteorology",
     "The apparent temperature of the US National Weather Service: Rothfusz's regression with the NWS adjustments for "
     "low and high humidity.",
     [_TEMP("T", "air temperature"), I("RH", "number", "fraction", "relative humidity (fraction, or % in a question)",
                                       ("relative humidity", "humidity"), "%")],
     [O("HI", "temperature", "heat index (K)")], heat_index,
     lambda T, RH: (lambda mc, u: {"HI": float(mc.heat_index(T * u.K, RH * u.dimensionless, mask_undefined=False)[0].m_as("K"))})(*_metpy()),
     lambda T, RH: {"HI": _r((_heat_index_F((T - 273.15) * 9 / 5 + 32, RH * 100) - 32) * 5 / 9 + 273.15)},
     lambda rng: {"T": rng.uniform(300.0, 318.0), "RH": rng.uniform(0.05, 1.0)},
     [{"T": 32, "RH": 70}, {"T": 38, "RH": 10}], ["heat index"],
     conditions=["80 °F <= T <= 130 °F", "0 <= RH <= 1"], invalid=[{"T": 20, "RH": 50}],
     method="Rothfusz (1990) regression with the NWS adjustments (as MetPy), computed in °F",
     latex=r"HI = -42.379 + 2.049T + 10.143RH - 0.2248\,T\,RH - \dots\ (^\circ\mathrm{F},\ RH\ \mathrm{in}\ \%)")


# --- Coriolis, inertial motion, winds --------------------------------------------------------------------------------
def _lat_ok(lat, lim=90.0, name="latitude"):
    _finite(lat=lat)
    _need(-lim <= lat <= lim, f"{name} must be between -{lim:g} and {lim:g} degrees")


def coriolis(lat):
    _lat_ok(lat)
    return {"f": _r(2 * OMEGA * math.sin(math.radians(lat)))}


_LAT = lambda sym="lat", meaning="latitude (degrees, north positive)", names=("latitude",), **kw: I(sym, "number", "degrees", meaning, names, "°", **kw)  # noqa: E731
proc("coriolis_parameter", "Coriolis parameter", "atmospheric dynamics",
     "The Coriolis parameter f = 2 Omega sin(latitude).",
     [_LAT()], [O("f", "coriolis_parameter", "Coriolis parameter (1/s)")], coriolis,
     lambda lat: (lambda mc, u: {"f": float(mc.coriolis_parameter(lat * u.deg).m_as("1/s"))})(*_metpy()),
     lambda lat: {"f": _r(OMEGA * math.sin(math.radians(lat)))},
     lambda rng: {"lat": rng.choice([-1, 1]) * rng.uniform(1, 89)},
     [{"lat": 45}, {"lat": -30}], ["coriolis parameter", "coriolis frequency"],
     conditions=["-90 <= latitude <= 90 degrees"], invalid=[{"lat": 100}],
     method="f = 2 Omega sin(phi), Omega = 7.292115e-5 rad/s", latex=r"f = 2\Omega\sin\varphi")


def inertial_period(lat):
    _lat_ok(lat)
    _need(abs(lat) >= 0.5, "inertial oscillations need |latitude| >= 0.5 degrees (f = 0 at the equator)")
    f = 2 * OMEGA * math.sin(math.radians(lat))
    T = 2 * math.pi / abs(f)
    return {"T_inertial": _r(T), "T_hours": _r(T / 3600), "f": _r(f)}


def _ref_inertial(lat):
    import numpy as np
    mc, u = _metpy()
    f = float(mc.coriolis_parameter(lat * u.deg).m_as("1/s"))
    w = np.max(np.abs(np.linalg.eigvals(np.array([[0.0, f], [-f, 0.0]])).imag))   # du/dt = f v, dv/dt = -f u
    return {"T_inertial": 2 * np.pi / w, "T_hours": 2 * np.pi / w / 3600, "f": f}


proc("inertial_period", "Period of inertial oscillations", "atmospheric and ocean dynamics",
     "The period 2 pi/|f| of free inertial oscillations of the wind or an ocean current at a latitude.",
     [_LAT()], [O("T_inertial", "time", "inertial period (s)"), O("T_hours", "number", "inertial period (hours)"),
                O("f", "coriolis_parameter", "Coriolis parameter (1/s)")], inertial_period, _ref_inertial,
     lambda lat: (lambda f: {"T_inertial": _r(math.pi / abs(f)), "T_hours": _r(math.pi / abs(f) / 3600), "f": _r(f)})(
         2 * OMEGA * math.sin(math.radians(lat))),
     lambda rng: {"lat": rng.choice([-1, 1]) * rng.uniform(2, 89)},
     [{"lat": 45}, {"lat": 30}], ["inertial period", "inertial oscillation"],
     conditions=["|latitude| >= 0.5 degrees"], invalid=[{"lat": 0}],
     method="T = 2 pi/|f| with f = 2 Omega sin(phi)", latex=r"T_i = \frac{2\pi}{|f|}")


def wind_components(speed, direction):
    _finite(speed=speed, direction=direction)
    _need(speed >= 0, "the wind speed cannot be negative")
    _need(0 <= direction <= 360, "the direction must be between 0 and 360 degrees")
    d = math.radians(direction)
    u, v = -speed * math.sin(d), -speed * math.cos(d)
    snap = lambda x: 0.0 if abs(x) <= 1e-12 * max(speed, 1e-300) else x      # noqa: E731
    return {"u": _r(snap(u)), "v": _r(snap(v))}


proc("wind_components", "Wind components from speed and direction", "atmospheric dynamics",
     "The eastward (u) and northward (v) wind components from the speed and the direction the wind blows FROM.",
     [I("speed", "speed", "speed", "wind speed", ("wind speed",), "m/s"),
      I("direction", "number", "degrees", "direction the wind blows from (degrees clockwise from north)",
        ("wind direction",), "°")],
     [O("u", "speed", "eastward component (m/s)"), O("v", "speed", "northward component (m/s)")], wind_components,
     lambda speed, direction: (lambda mc, u: (lambda r: {"u": float(r[0].m_as("m/s")), "v": float(r[1].m_as("m/s"))})(
         mc.wind_components(speed * u("m/s"), direction * u.deg)))(*_metpy()),
     lambda speed, direction: {"u": _r(speed * math.sin(math.radians(direction))), "v": _r(speed * math.cos(math.radians(direction)))},
     lambda rng: {"speed": rng.uniform(0.5, 40), "direction": rng.uniform(0, 360)},
     [{"speed": 10, "direction": 225}, {"speed": 15, "direction": 300}], ["wind components"],
     conditions=["0 <= direction <= 360", "meteorological convention: the direction the wind comes from"],
     method="u = -V sin(dir), v = -V cos(dir)", latex=r"u = -V\sin\delta,\quad v = -V\cos\delta")


def wind_speed_direction(u, v):
    _finite(u=u, v=v)
    s = math.hypot(u, v)
    if u == 0 and v == 0:
        d = 0.0
    else:
        d = 90.0 - math.degrees(math.atan2(-v, -u))
        if d <= 0:
            d += 360.0
    return {"speed": _r(s), "direction": _r(d)}


proc("wind_speed_direction", "Wind speed and direction from components", "atmospheric dynamics",
     "Wind speed and the direction the wind blows from (degrees clockwise from north, (0, 360]) from u and v.",
     [I("u", "speed", "speed", "eastward wind component", ("u component", "eastward component"), "m/s"),
      I("v", "speed", "speed", "northward wind component", ("v component", "northward component"), "m/s")],
     [O("speed", "speed", "wind speed (m/s)"), O("direction", "number", "direction the wind comes from (degrees)")],
     wind_speed_direction,
     lambda u, v: (lambda mc, un: {"speed": float(mc.wind_speed(u * un("m/s"), v * un("m/s")).m_as("m/s")),
                                   "direction": float(mc.wind_direction(u * un("m/s"), v * un("m/s")).m_as("deg"))})(*_metpy()),
     lambda u, v: {"speed": _r(math.hypot(u, v)), "direction": _r((math.degrees(math.atan2(u, v))) % 360)},
     lambda rng: {"u": rng.uniform(-25, 25), "v": rng.uniform(-25, 25)},
     [{"u": 5, "v": -3}, {"u": -8, "v": 6}], ["wind speed and direction", "wind direction"],
     conditions=["calm (u = v = 0) gives direction 0"], method="V = hypot(u, v); direction = 90 - atan2(-v, -u) in degrees",
     latex=r"V = \sqrt{u^2+v^2},\quad \delta = 90^\circ - \operatorname{atan2}(-v,-u)")


def geostrophic_wind(lat, dz, dn):
    _lat_ok(lat)
    _finite(dz=dz, dn=dn)
    _need(5 <= abs(lat) <= 85, "geostrophic balance is used here for 5 to 85 degrees latitude")
    _need(dn > 0, "the distance must be positive")
    _need(dz != 0, "a height difference is needed (no gradient, no geostrophic wind)")
    f = 2 * OMEGA * math.sin(math.radians(lat))
    return {"Vg": _r(G0 / abs(f) * abs(dz) / dn), "f": _r(f)}


def _ref_geostrophic(lat, dz, dn):
    import numpy as np
    mc, u = _metpy()
    n = 5
    yy = np.arange(n)[:, None] * np.ones((1, n))
    Z = (5500.0 + dz * yy) * u.m                                       # the height rises by dz every dn northward
    la = np.full((n, n), lat) * u.deg
    ug, vg = mc.geostrophic_wind(Z, dx=dn * u.m, dy=dn * u.m, latitude=la)
    return {"Vg": float(np.hypot(ug.m_as("m/s"), vg.m_as("m/s"))[2, 2]),
            "f": float(mc.coriolis_parameter(lat * u.deg).m_as("1/s"))}


proc("geostrophic_wind", "Geostrophic wind from a height gradient", "atmospheric dynamics",
     "Geostrophic wind speed from the change of geopotential height of a pressure surface over a horizontal distance.",
     [_LAT(), I("dz", "length", "length", "change of geopotential height across the distance", ("height difference",), "m"),
      I("dn", "length", "length", "horizontal distance over which the height changes", ("distance",), "km")],
     [O("Vg", "speed", "geostrophic wind speed (m/s)"), O("f", "coriolis_parameter", "Coriolis parameter (1/s)")],
     geostrophic_wind, _ref_geostrophic,
     lambda lat, dz, dn: {"Vg": _r(G0 / abs(OMEGA * math.sin(math.radians(lat))) * abs(dz) / dn),
                          "f": _r(2 * OMEGA * math.sin(math.radians(lat)))},
     lambda rng: {"lat": rng.choice([-1, 1]) * rng.uniform(10, 80), "dz": rng.uniform(10, 200), "dn": rng.uniform(1e5, 1e6)},
     [{"lat": 45, "dz": 60, "dn": 500}, {"lat": 30, "dz": 40, "dn": 300}], ["geostrophic wind"],
     conditions=["5 <= |latitude| <= 85 degrees", "dn > 0"], invalid=[{"lat": 0, "dz": 60, "dn": 500}],
     method="V_g = (g/|f|) |dZ/dn|", latex=r"V_g = \frac{g}{|f|}\left|\frac{\Delta Z}{\Delta n}\right|")


def brunt_vaisala(theta, dtheta_dz):
    _T_ok(theta, 200, 600, "potential temperature")
    _finite(dtheta_dz=dtheta_dz)
    _need(0 < dtheta_dz <= 0.1, "the potential-temperature gradient must be positive (stable) and at most 0.1 K/m")
    n = math.sqrt(G0 / theta * dtheta_dz)
    return {"N": _r(n), "period": _r(2 * math.pi / n)}


def _ref_bv(theta, dtheta_dz):
    import numpy as np
    mc, u = _metpy()
    z = np.array([900.0, 1000.0, 1100.0])
    th = theta + dtheta_dz * (z - 1000.0)
    n = float(mc.brunt_vaisala_frequency(z * u.m, th * u.K).m_as("1/s")[1])
    return {"N": n, "period": 2 * np.pi / n}


proc("brunt_vaisala_frequency", "Brunt-Väisälä (buoyancy) frequency", "atmospheric dynamics",
     "The buoyancy frequency of a stably stratified atmosphere from the potential temperature and its vertical gradient.",
     [I("theta", "temperature", "temperature", "potential temperature at the level", ("potential temperature",), "K"),
      I("dtheta_dz", "temperature_gradient", "lapse_rate", "vertical gradient of potential temperature",
        ("potential temperature gradient", "dtheta/dz"), "K/km")],
     [O("N", "buoyancy_frequency", "Brunt-Väisälä frequency (1/s)"), O("period", "time", "buoyancy period 2 pi/N (s)")],
     brunt_vaisala, _ref_bv,
     lambda theta, dtheta_dz: {"N": _r(G0 / theta * dtheta_dz), "period": _r(2 * math.pi / (G0 / theta * dtheta_dz))},
     lambda rng: {"theta": rng.uniform(270, 400), "dtheta_dz": rng.uniform(5e-4, 0.02)},
     [{"theta": 300, "dtheta_dz": 3}, {"theta": 350, "dtheta_dz": 15}], ["brunt vaisala frequency",
                                                                       "brunt väisälä frequency", "buoyancy frequency"],
     conditions=["d theta/dz > 0 (stable stratification)"], invalid=[{"theta": 300, "dtheta_dz": -2}],
     method="N = sqrt((g/theta) d theta/dz)", latex=r"N = \sqrt{\frac{g}{\theta}\frac{\partial\theta}{\partial z}}")


# --- hydrostatic balance ------------------------------------------------------------------------------------------------
def isothermal_pressure(p0, T, z):
    _p_ok(p0, 1000, 110000, "surface pressure")
    _T_ok(T, 150, 350)
    _finite(z=z)
    _need(0 <= z <= 50000, "height must be between 0 and 50 km")
    H = RD * T / G0
    return {"p": _r(p0 * math.exp(-z / H)), "H": _r(H)}


def _ref_isothermal(p0, T, z):
    p = _ivp(lambda zz, y: [-y[0] * G0 / (RD * T)], z, [p0])[0] if z > 0 else p0
    H = _ivp(lambda pp, y: [-RD * T / (G0 * pp)], p0 / math.e, [0.0], t0=p0)[0]    # height at which p = p0/e
    return {"p": p, "H": H}


proc("isothermal_pressure", "Pressure at height in an isothermal atmosphere", "atmospheric thermodynamics",
     "The barometric formula: pressure at height z in an isothermal dry atmosphere, and its scale height R_d T/g.",
     [_PRES("p0", "pressure at z = 0", ("surface pressure",)), _TEMP("T", "temperature of the layer"),
      I("z", "length", "length", "height above the reference level", ("height",), "m")],
     [O("p", "pressure", "pressure at height z (Pa)"), O("H", "length", "scale height (m)")], isothermal_pressure,
     _ref_isothermal, lambda p0, T, z: {"p": _r(p0 * (1 - G0 * z / (RD * T))), "H": _r(RD * T / G0)},
     lambda rng: {"p0": rng.uniform(95000, 104000), "T": rng.uniform(220, 310), "z": rng.uniform(200, 15000)},
     [{"p0": 1013.25, "T": 0, "z": 5000}, {"p0": 1000, "T": -20, "z": 2000}],
     ["isothermal atmosphere", "barometric formula", "pressure at height"],
     conditions=["isothermal dry air", "constant gravity g = 9.80665 m/s^2"],
     method="p = p0 exp(-z/H), H = R_d T/g", latex=r"p = p_0\,e^{-z/H},\quad H = \frac{R_d T}{g}")


def scale_height(T):
    _T_ok(T, 150, 350)
    return {"H": _r(RD * T / G0)}


proc("scale_height", "Atmospheric scale height", "atmospheric thermodynamics",
     "The height over which pressure falls by a factor e in an isothermal dry atmosphere.",
     [_TEMP("T", "temperature")], [O("H", "length", "scale height (m)")], scale_height,
     lambda T: {"H": _ivp(lambda pp, y: [-RD * T / (G0 * pp)], 1e5 / math.e, [0.0], t0=1e5)[0]},
     lambda T: {"H": _r(8.314462618 * T / G0)},
     lambda rng: {"T": rng.uniform(180, 330)},
     [{"T": 15}, {"T": -50}], ["scale height"],
     conditions=["dry air, constant gravity"], method="H = R_d T/g", latex=r"H = \frac{R_d T}{g}")


def hypsometric_thickness(p1, p2, Tv):
    _p_ok(p1, 100, 110000, "bottom pressure")
    _p_ok(p2, 100, 110000, "top pressure")
    _T_ok(Tv, 150, 350, "mean virtual temperature")
    _need(p1 > p2, "the bottom pressure p1 must exceed the top pressure p2")
    return {"dz": _r(RD * Tv / G0 * math.log(p1 / p2))}


def _ref_hypso(p1, p2, Tv):
    from scipy.integrate import quad
    v, _ = quad(lambda p: RD * Tv / (G0 * p), p2, p1, epsabs=0, epsrel=1e-13, limit=200)
    return {"dz": v}


proc("hypsometric_thickness", "Thickness of a layer (hypsometric equation)", "atmospheric thermodynamics",
     "The geometric thickness between two pressure levels from the layer-mean virtual temperature.",
     [_PRES("p1", "pressure at the bottom of the layer", ("bottom pressure", "lower pressure")),
      _PRES("p2", "pressure at the top of the layer", ("top pressure", "upper pressure")),
      I("Tv", "temperature", "temperature", "layer-mean virtual temperature", ("mean virtual temperature",
                                                                             "mean temperature"), "°C")],
     [O("dz", "length", "layer thickness (m)")], hypsometric_thickness, _ref_hypso,
     lambda p1, p2, Tv: {"dz": _r(RD * Tv / G0 * math.log10(p1 / p2))},
     lambda rng: (lambda p1: {"p1": p1, "p2": p1 * rng.uniform(0.2, 0.9), "Tv": rng.uniform(220, 300)})(rng.uniform(70000, 104000)),
     [{"p1": 1000, "p2": 500, "Tv": -10}, {"p1": 850, "p2": 700, "Tv": 5}],
     ["hypsometric equation", "thickness of the layer", "layer thickness"],
     conditions=["p1 > p2"], invalid=[{"p1": 500, "p2": 1000, "Tv": -10}],
     method="dz = (R_d Tv/g) ln(p1/p2)", latex=r"\Delta z = \frac{R_d\overline{T_v}}{g}\ln\frac{p_1}{p_2}")


def lapse_rate_pressure(p0, T0, gamma, z):
    _p_ok(p0, 1000, 110000, "surface pressure")
    _T_ok(T0, 150, 350, "surface temperature")
    _finite(gamma=gamma, z=z)
    _need(0 < gamma <= 0.0098, "the lapse rate must be positive and at most 9.8 K/km")
    _need(0 <= z <= 20000, "height must be between 0 and 20 km")
    T = T0 - gamma * z
    _need(T > 150, "the temperature at that height would fall below 150 K")
    return {"p": _r(p0 * (T / T0) ** (G0 / (RD * gamma))), "T": _r(T)}


def _ref_lapse_pressure(p0, T0, gamma, z):
    p = _ivp(lambda zz, y: [-y[0] * G0 / (RD * (T0 - gamma * zz))], z, [p0])[0] if z > 0 else p0
    return {"p": p, "T": T0 - gamma * z}


proc("pressure_with_lapse_rate", "Pressure at height with a constant lapse rate", "atmospheric thermodynamics",
     "Pressure at height z in a dry atmosphere whose temperature falls linearly with height (troposphere model).",
     [_PRES("p0", "pressure at z = 0", ("surface pressure",)), _TEMP("T0", "temperature at z = 0", ("surface temperature",)),
      I("gamma", "lapse_rate", "lapse_rate", "temperature lapse rate (positive when temperature falls with height)",
        ("lapse rate",), "K/km"),
      I("z", "length", "length", "height above the reference level", ("height",), "m")],
     [O("p", "pressure", "pressure at height z (Pa)"), O("T", "temperature", "temperature at height z (K)")],
     lapse_rate_pressure, _ref_lapse_pressure,
     lambda p0, T0, gamma, z: {"p": _r(p0 * ((T0 - gamma * z) / T0) ** (RD * gamma / G0)), "T": _r(T0 - gamma * z)},
     lambda rng: {"p0": rng.uniform(95000, 104000), "T0": rng.uniform(260, 310), "gamma": rng.uniform(0.003, 0.0098),
                  "z": rng.uniform(200, 10000)},
     [{"p0": 1013.25, "T0": 15, "gamma": 6.5, "z": 3000}, {"p0": 1000, "T0": 25, "gamma": 8, "z": 1500}],
     ["constant lapse rate", "polytropic atmosphere"],
     conditions=["0 < gamma <= 9.8 K/km", "dry air, constant gravity"],
     method="p = p0 (T/T0)^(g/(R_d gamma)), T = T0 - gamma z",
     latex=r"p = p_0\left(\frac{T_0-\gamma z}{T_0}\right)^{g/(R_d\gamma)}")


def std_pressure(z):
    _finite(z=z)
    _need(-500 <= z <= 11000, "the tropospheric standard atmosphere covers -500 m to 11 km")
    return {"p": _r(101325.0 * (1 - (0.0065 / 288.0) * z) ** (G0 / (RD * 0.0065)))}


proc("standard_atmosphere_pressure", "Pressure at a height in the standard atmosphere", "atmospheric thermodynamics",
     "Pressure at a height in the tropospheric standard atmosphere (T0 = 288 K, 6.5 K/km, 1013.25 hPa; as MetPy).",
     [I("z", "length", "length", "height above sea level", ("height", "altitude"), "m")],
     [O("p", "pressure", "standard-atmosphere pressure (Pa)")], std_pressure,
     lambda z: (lambda mc, u: {"p": float(mc.height_to_pressure_std(z * u.m).m_as("Pa"))})(*_metpy()),
     lambda z: {"p": _r(101325.0 * math.exp(-G0 * z / (RD * 288.0)))},
     lambda rng: {"z": rng.uniform(100, 11000)},
     [{"z": 1500}, {"z": 9000}], ["standard atmosphere pressure", "pressure in the standard atmosphere"],
     conditions=["-500 m <= z <= 11 km (troposphere)"], invalid=[{"z": 20000}],
     method="p = p0 (1 - gamma z/T0)^(g/(R_d gamma))", latex=r"p = p_0\left(1-\frac{\gamma z}{T_0}\right)^{g/(R_d\gamma)}")


def std_height(p):
    _finite(p=p)
    _need(22632.0 <= p <= 107500.0, "the tropospheric standard atmosphere covers about 1075 hPa to 226 hPa")
    return {"z": _r(288.0 / 0.0065 * (1 - (p / 101325.0) ** (RD * 0.0065 / G0)))}


proc("standard_atmosphere_height", "Height of a pressure level in the standard atmosphere", "atmospheric thermodynamics",
     "The height of a pressure level in the tropospheric standard atmosphere (pressure altitude; as MetPy).",
     [_PRES("p", "pressure")], [O("z", "length", "pressure altitude (m)")], std_height,
     lambda p: (lambda mc, u: {"z": float(mc.pressure_to_height_std(p * u.Pa).m_as("m"))})(*_metpy()),
     lambda p: {"z": _r(-RD * 288.0 / G0 * math.log(p / 101325.0))},
     lambda rng: {"p": rng.uniform(25000, 98000)},
     [{"p": 500}, {"p": 850}], ["standard atmosphere height", "pressure altitude"],
     conditions=["226 hPa <= p <= 1075 hPa"], invalid=[{"p": 100}],
     method="z = (T0/gamma)(1 - (p/p0)^(R_d gamma/g))", latex=r"z = \frac{T_0}{\gamma}\left[1-\left(\frac{p}{p_0}\right)^{R_d\gamma/g}\right]")


def geopotential(z):
    _finite(z=z)
    _need(-1000 <= z <= 200000, "height must be between -1 km and 200 km")
    phi = G0 * RE * z / (RE + z)
    return {"phi": _r(phi), "Z": _r(phi / G0)}


proc("geopotential_height", "Geopotential and geopotential height", "atmospheric thermodynamics",
     "Geopotential and geopotential height of a geometric height above sea level, with gravity falling off as "
     "1/(R_e + z)^2 (as MetPy).",
     [I("z", "length", "length", "geometric height above sea level", ("height", "altitude"), "km")],
     [O("phi", "geopotential", "geopotential (m^2/s^2)"), O("Z", "length", "geopotential height (m)")], geopotential,
     lambda z: (lambda mc, u: (lambda ph: {"phi": ph, "Z": ph / G0})(float(mc.height_to_geopotential(z * u.m).m_as("m^2/s^2"))))(*_metpy()),
     lambda z: {"phi": _r(G0 * z), "Z": _r(z)},
     lambda rng: {"z": rng.uniform(500, 100000)},
     [{"z": 10}, {"z": 50}], ["geopotential height", "geopotential"],
     conditions=["-1 km <= z <= 200 km"], method="Phi = g R_e z/(R_e + z)", latex=r"\Phi = \frac{g\,R_e\,z}{R_e+z},\quad Z = \Phi/g")


# --- Rossby waves -----------------------------------------------------------------------------------------------------
def rossby_wave(U, lat, Lx):
    _finite(U=U, Lx=Lx)
    _lat_ok(lat)
    _need(5 <= abs(lat) <= 85, "the beta-plane is used here for 5 to 85 degrees latitude")
    _need(U > 0, "a westerly mean wind U > 0 is required (it sets the stationary wavelength)")
    _need(1e5 <= Lx <= 4e7, "the zonal wavelength must be between 100 km and 40,000 km")
    beta = 2 * OMEGA * math.cos(math.radians(lat)) / RE
    k = 2 * math.pi / Lx
    return {"c": _r(U - beta / (k * k)), "beta": _r(beta), "Ls": _r(2 * math.pi * math.sqrt(U / beta))}


@lru_cache(maxsize=1)
def _rossby_derived():
    """Plane wave psi = exp(i(kx - wt)) put into the linearised barotropic vorticity equation
    (d/dt + U d/dx) lap(psi) + beta dpsi/dx = 0; beta = df/dy of f = 2 Omega sin(y/a). SymPy solves for w and for the
    wavenumber at which the phase speed vanishes."""
    import sympy as sp
    k, U, beta, w, x, t, Om, a, y, phi = sp.symbols("k U beta w x t Omega a y phi", real=True)
    psi = sp.exp(sp.I * (k * x - w * t))
    eq = sp.diff(sp.diff(psi, x, 2), t) + U * sp.diff(sp.diff(psi, x, 2), x) + beta * sp.diff(psi, x)
    wsol = sp.solve(sp.simplify(eq / psi), w)[0]
    c = sp.simplify(wsol / k)
    ks = [s for s in sp.solve(sp.Eq(c, 0), k) if s.could_extract_minus_sign() is False][0]
    b = sp.diff(2 * Om * sp.sin(y / a), y).subs(y, a * phi)
    return (sp.lambdify((k, U, beta), c), sp.lambdify((U, beta), 2 * sp.pi / ks), sp.lambdify((Om, a, phi), b))


def _ref_rossby(U, lat, Lx):
    cf, lf, bf = _rossby_derived()
    beta = float(bf(OMEGA, RE, math.radians(lat)))
    return {"c": float(cf(2 * math.pi / Lx, U, beta)), "beta": beta, "Ls": float(lf(U, beta))}


proc("rossby_wave_speed", "Phase speed of a barotropic Rossby wave", "atmospheric dynamics",
     "Zonal phase speed of a barotropic Rossby wave in a uniform westerly flow on a beta-plane, the beta parameter, "
     "and the stationary wavelength 2 pi sqrt(U/beta).",
     [I("U", "speed", "speed", "zonal mean (westerly) wind speed", ("mean wind", "zonal wind"), "m/s"), _LAT(),
      I("Lx", "length", "length", "zonal wavelength", ("wavelength",), "km")],
     [O("c", "speed", "zonal phase speed (m/s; negative = westward)"), O("beta", "beta_parameter", "beta = df/dy (1/(m s))"),
      O("Ls", "length", "stationary wavelength (m)")], rossby_wave, _ref_rossby,
     lambda U, lat, Lx: (lambda beta, k: {"c": _r(U + beta / (k * k)), "beta": _r(beta), "Ls": _r(2 * math.pi * math.sqrt(U / beta))})(
         2 * OMEGA * math.cos(math.radians(lat)) / RE, 2 * math.pi / Lx),
     lambda rng: {"U": rng.uniform(2, 40), "lat": rng.uniform(20, 70), "Lx": rng.uniform(2e6, 1.2e7)},
     [{"U": 15, "lat": 45, "Lx": 6000}, {"U": 10, "lat": 60, "Lx": 4000}], ["rossby wave", "rossby wave speed",
                                                                         "rossby wave phase speed"],
     conditions=["U > 0", "5 <= |latitude| <= 85", "no meridional wavenumber (l = 0)"],
     method="c = U - beta/k^2, beta = 2 Omega cos(phi)/a, L_s = 2 pi sqrt(U/beta)",
     latex=r"c = U - \frac{\beta}{k^2},\quad \beta = \frac{2\Omega\cos\varphi}{a},\quad L_s = 2\pi\sqrt{U/\beta}")


# =============================================================================================== radiation and climate
_FLUX = lambda sym, meaning, names, show="W/m^2", **kw: I(sym, "radiative_flux", "radiative_flux", meaning, names, show, **kw)  # noqa: E731
_ALB = lambda: I("albedo", "number", "fraction", "planetary (Bond) albedo, a fraction", ("albedo",), "")  # noqa: E731


def _planck_constants():
    from scipy import constants as sc
    return sc.h, sc.c, sc.k, sc.sigma


def blackbody_flux(T, eps=1.0):
    _finite(T=T, eps=eps)
    _need(1 <= T <= 1e5, "temperature must be between 1 K and 100,000 K")
    _need(0 < eps <= 1, "emissivity must be in (0, 1]")
    return {"F": _r(eps * SIGMA * T ** 4)}


def _ref_blackbody(T, eps=1.0):
    from scipy.integrate import quad
    h, c, k, _s = _planck_constants()
    # pi times Planck's spectral radiance integrated over all frequencies, written in x = h nu/(k T)
    f = lambda x: x ** 3 / math.expm1(x) if 0 < x < 700 else 0.0          # noqa: E731  (the tail beyond 700 is < 1e-290)
    v = sum(quad(f, a, b, epsabs=0, epsrel=1e-13, limit=400)[0] for a, b in ((0, 3), (3, 10), (10, 40), (40, 700)))
    return {"F": eps * 2 * math.pi * (k * T) ** 4 / (h ** 3 * c ** 2) * v}


proc("blackbody_flux", "Radiant flux emitted by a black or grey body (Stefan-Boltzmann)", "radiation balance",
     "Power emitted per unit area by a surface of temperature T and (grey) emissivity eps.",
     [I("T", "temperature", "temperature", "surface temperature", ("temperature",), "K"),
      I("eps", "number", "number", "emissivity (0 < eps <= 1; default 1, a black body)", ("emissivity",), "",
        required=False)],
     [O("F", "radiative_flux", "emitted flux (W/m^2)")], blackbody_flux, _ref_blackbody,
     lambda T, eps=1.0: {"F": _r(eps * SIGMA * T ** 4 / math.pi)},
     lambda rng: {"T": rng.uniform(150, 6000), "eps": rng.uniform(0.5, 1.0)},
     [{"T": 288, "eps": 1}, {"T": 255, "eps": 0.95}], ["stefan boltzmann", "blackbody flux", "black body flux",
                                                       "emitted flux", "grey body flux"],
     conditions=["0 < eps <= 1", "grey body: emissivity independent of wavelength"],
     method="F = eps sigma T^4 (sigma = 5.670374419e-8 W/(m^2 K^4))", latex=r"F = \varepsilon\,\sigma\,T^4")


def wien_peak(T):
    _finite(T=T)
    _need(1 <= T <= 1e5, "temperature must be between 1 K and 100,000 K")
    lam = WIEN_B / T
    return {"lambda_max": _r(lam), "lambda_um": _r(lam * 1e6)}


def _ref_wien(T):
    h, c, k, _s = _planck_constants()
    B = lambda lam: 2 * h * c * c / lam ** 5 / (__import__("cmath").exp(h * c / (lam * k * T)) - 1)   # noqa: E731
    dB = lambda lam: (B(lam + 1e-30j * lam)).imag / (1e-30 * lam)     # complex-step derivative of Planck's law  # noqa: E731
    guess = 2.9e-3 / T
    lam = _brentq(dB, 0.5 * guess, 2 * guess)
    return {"lambda_max": lam, "lambda_um": lam * 1e6}


proc("wien_peak_wavelength", "Wavelength of peak blackbody emission (Wien)", "radiation balance",
     "The wavelength at which a black body of temperature T emits most strongly per unit wavelength.",
     [I("T", "temperature", "temperature", "temperature of the body", ("temperature",), "K")],
     [O("lambda_max", "length", "peak wavelength (m)"), O("lambda_um", "number", "peak wavelength (micrometres)")],
     wien_peak, _ref_wien,
     lambda T: (lambda h, c, k: {"lambda_max": _r(h * c / (5 * k * T)), "lambda_um": _r(h * c / (5 * k * T) * 1e6)})(
         6.62607015e-34, 299792458.0, 1.380649e-23),
     lambda rng: {"T": rng.uniform(150, 8000)},
     [{"T": 5772}, {"T": 288}], ["wien peak wavelength", "wien's law", "peak wavelength", "wien displacement"],
     conditions=["1 K <= T <= 1e5 K"], method="lambda_max = b/T, b = 2.897771955e-3 m K",
     latex=r"\lambda_{\max} = \frac{b}{T},\quad b = 2.897771955\times10^{-3}\ \mathrm{m\,K}")


def equilibrium_temperature(S, albedo):
    _finite(S=S, albedo=albedo)
    _need(S > 0, "the solar flux must be positive")
    _need(0 <= albedo < 1, "albedo must be in [0, 1)")
    q = S * (1 - albedo) / 4
    return {"Te": _r((q / SIGMA) ** 0.25), "absorbed": _r(q)}


def _ref_equilibrium(S, albedo):
    from scipy.integrate import dblquad
    _h, _c, _k, sigma = _planck_constants()
    # sunlight absorbed over the day-side hemisphere of a unit sphere (cos of the solar zenith angle), emitted
    # from the whole sphere (4 pi)
    absorbed, _ = dblquad(lambda th, ph: S * (1 - albedo) * math.cos(th) * math.sin(th), 0, 2 * math.pi, 0, math.pi / 2,
                          epsabs=0, epsrel=1e-13)
    return {"Te": (absorbed / (4 * math.pi * sigma)) ** 0.25, "absorbed": absorbed / (4 * math.pi)}


proc("planetary_equilibrium_temperature", "Planetary equilibrium (effective radiating) temperature", "radiation balance",
     "The temperature at which a planet radiating as a black body balances the sunlight it absorbs.",
     [_FLUX("S", "solar flux at the planet's distance (solar constant)", ("solar constant", "solar flux")), _ALB()],
     [O("Te", "temperature", "equilibrium temperature (K)"), O("absorbed", "radiative_flux", "absorbed flux per unit "
                                                                   "surface area, S(1 - albedo)/4 (W/m^2)")],
     equilibrium_temperature, _ref_equilibrium,
     lambda S, albedo: {"Te": _r((S * (1 - albedo) / SIGMA) ** 0.25), "absorbed": _r(S * (1 - albedo))},
     lambda rng: {"S": rng.uniform(5, 3000), "albedo": rng.uniform(0, 0.9)},
     [{"S": 1361, "albedo": 0.3}, {"S": 589, "albedo": 0.25}], ["equilibrium temperature", "effective temperature",
                                                                "effective radiating temperature",
                                                                "planetary equilibrium temperature"],
     conditions=["0 <= albedo < 1", "fast rotation: absorbed sunlight spread over the whole sphere"],
     method="T_e = (S (1 - albedo)/(4 sigma))^(1/4)", latex=r"T_e = \left(\frac{S(1-\alpha)}{4\sigma}\right)^{1/4}")


def greenhouse_single_layer(S, albedo, eps):
    _finite(S=S, albedo=albedo, eps=eps)
    _need(S > 0, "the solar flux must be positive")
    _need(0 <= albedo < 1, "albedo must be in [0, 1)")
    _need(0 < eps <= 1, "the layer's emissivity must be in (0, 1]")
    q = S * (1 - albedo) / 4
    ts = (q / (SIGMA * (1 - eps / 2))) ** 0.25
    return {"Ts": _r(ts), "Ta": _r(ts / 2 ** 0.25)}


def _ref_greenhouse(S, albedo, eps):
    import numpy as np
    _h, _c, _k, sigma = _planck_constants()
    q = S * (1 - albedo) / 4
    # unknowns sigma*Ts^4, sigma*Ta^4. Surface: absorbs sunlight and the layer's downward emission, emits sigma Ts^4.
    # Layer: absorbs eps of the surface emission, emits eps sigma Ta^4 up and down.
    x = np.linalg.solve(np.array([[1.0, -eps], [eps, -2 * eps]]), np.array([q, 0.0]))
    return {"Ts": (x[0] / sigma) ** 0.25, "Ta": (x[1] / sigma) ** 0.25}


proc("greenhouse_single_layer", "Surface temperature in the single-layer greenhouse model", "radiation balance",
     "Surface and atmosphere temperatures when one isothermal layer, transparent to sunlight, absorbs and emits a "
     "fraction eps of thermal radiation.",
     [_FLUX("S", "solar constant", ("solar constant", "solar flux")), _ALB(),
      I("eps", "number", "number", "long-wave emissivity (absorptivity) of the atmospheric layer", ("emissivity",), "")],
     [O("Ts", "temperature", "surface temperature (K)"), O("Ta", "temperature", "atmosphere temperature (K)")],
     greenhouse_single_layer, _ref_greenhouse,
     lambda S, albedo, eps: (lambda ts: {"Ts": _r(ts), "Ta": _r(ts / 2 ** 0.25)})((S * (1 - albedo) / 4 / SIGMA * 2) ** 0.25),
     lambda rng: {"S": rng.uniform(500, 2500), "albedo": rng.uniform(0.05, 0.6), "eps": rng.uniform(0.3, 0.95)},
     [{"S": 1361, "albedo": 0.3, "eps": 0.78}, {"S": 1361, "albedo": 0.3, "eps": 1}],
     ["single layer greenhouse", "greenhouse model", "one layer greenhouse"],
     conditions=["the layer is transparent to sunlight", "0 < eps <= 1"],
     method="T_s = [S(1 - albedo)/(4 sigma (1 - eps/2))]^(1/4), T_a = T_s/2^(1/4)",
     latex=r"T_s = \left[\frac{S(1-\alpha)}{4\sigma(1-\varepsilon/2)}\right]^{1/4},\quad T_a = 2^{-1/4}T_s")


def greenhouse_n_layers(S, albedo, N):
    _finite(S=S, albedo=albedo)
    _need(S > 0 and 0 <= albedo < 1, "S must be positive and albedo in [0, 1)")
    _need(isinstance(N, int) and 1 <= N <= 100, "the number of layers must be a whole number from 1 to 100")
    te = (S * (1 - albedo) / (4 * SIGMA)) ** 0.25
    return {"Ts": _r((N + 1) ** 0.25 * te), "T_top": _r(te), "T_bottom_layer": _r(N ** 0.25 * te)}


def _ref_n_layers(S, albedo, N):
    import numpy as np
    _h, _c, _k, sigma = _planck_constants()
    q = S * (1 - albedo) / 4
    n = N + 1                                               # E_0 surface, E_1 .. E_N layers (sigma T^4)
    A = np.zeros((n, n))
    b = np.zeros(n)
    A[0, 0], A[0, 1], b[0] = 1.0, -1.0, q                   # surface: E0 = q + E1
    for i in range(1, n):                                   # layer i: 2 E_i = E_{i-1} + E_{i+1}
        A[i, i - 1], A[i, i] = -1.0, 2.0
        if i + 1 < n:
            A[i, i + 1] = -1.0
    E = np.linalg.solve(A, b)
    return {"Ts": (E[0] / sigma) ** 0.25, "T_top": (E[-1] / sigma) ** 0.25, "T_bottom_layer": (E[1] / sigma) ** 0.25}


proc("greenhouse_n_layers", "Surface temperature with N fully absorbing layers", "radiation balance",
     "Surface and layer temperatures of the N-layer grey-atmosphere model (each layer absorbs all thermal radiation).",
     [_FLUX("S", "solar constant", ("solar constant", "solar flux")), _ALB(),
      I("N", "number", "number", "number of absorbing layers (whole number)", ("layers", "number of layers"), "",
        typ="integer", min=1, max=100)],
     [O("Ts", "temperature", "surface temperature (K)"), O("T_top", "temperature", "temperature of the top layer (K)"),
      O("T_bottom_layer", "temperature", "temperature of the lowest layer (K)")], greenhouse_n_layers, _ref_n_layers,
     lambda S, albedo, N: (lambda te: {"Ts": _r(N ** 0.25 * te), "T_top": _r(te), "T_bottom_layer": _r(N ** 0.25 * te)})(
         (S * (1 - albedo) / (4 * SIGMA)) ** 0.25),
     lambda rng: {"S": rng.uniform(500, 2500), "albedo": rng.uniform(0.05, 0.6), "N": rng.randint(1, 40)},
     [{"S": 1361, "albedo": 0.3, "N": 2}, {"S": 2601, "albedo": 0.75, "N": 10}], ["layer greenhouse", "n layer greenhouse",
                                                                              "multi layer greenhouse"],
     conditions=["1 <= N <= 100 whole layers", "layers transparent to sunlight, opaque to thermal radiation"],
     method="T_s = (N + 1)^(1/4) T_e, layer i from the top at i^(1/4) T_e",
     latex=r"T_s = (N+1)^{1/4}\,T_e,\quad T_e = \left(\frac{S(1-\alpha)}{4\sigma}\right)^{1/4}")


def daily_insolation(lat, decl, S0):
    _lat_ok(lat, 89.9)
    _finite(decl=decl, S0=S0)
    _need(-23.5 <= decl <= 23.5, "the solar declination must be between -23.5 and 23.5 degrees")
    _need(S0 > 0, "the solar constant must be positive")
    ph, de = math.radians(lat), math.radians(decl)
    x = -math.tan(ph) * math.tan(de)
    h0 = 0.0 if x >= 1 else (math.pi if x <= -1 else math.acos(x))
    q = S0 / math.pi * (h0 * math.sin(ph) * math.sin(de) + math.cos(ph) * math.cos(de) * math.sin(h0))
    q = 0.0 if abs(q) <= 1e-12 * S0 else q
    return {"Q": _r(q), "day_length_h": _r(24 * h0 / math.pi)}


def _ref_insolation(lat, decl, S0):
    from scipy.integrate import quad
    ph, de = math.radians(lat), math.radians(decl)
    mu = lambda h: math.sin(ph) * math.sin(de) + math.cos(ph) * math.cos(de) * math.cos(h)    # noqa: E731
    # sunrise hour angle found as the zero of cos(zenith) by bisection (no closed form used), then the daily mean
    if mu(0.0) <= 0:
        h0 = 0.0
    elif mu(math.pi) >= 0:
        h0 = math.pi
    else:
        h0 = _brentq(mu, 0.0, math.pi)
    v, _ = quad(lambda h: S0 * mu(h), -h0, h0, epsabs=0, epsrel=1e-13) if h0 > 0 else (0.0, 0)
    return {"Q": v / (2 * math.pi), "day_length_h": 24 * h0 / math.pi}


proc("daily_mean_insolation", "Daily mean insolation at the top of the atmosphere", "radiation balance",
     "The 24-hour mean solar flux on a horizontal surface at the top of the atmosphere for a latitude and solar "
     "declination (polar day and polar night included).",
     [_LAT(), I("decl", "number", "degrees", "solar declination (degrees)", ("declination", "solar declination"), "°"),
      _FLUX("S0", "solar constant (at the actual Earth-Sun distance)", ("solar constant",))],
     [O("Q", "radiative_flux", "daily mean insolation (W/m^2)"), O("day_length_h", "number", "length of the day (hours)")],
     daily_insolation, _ref_insolation,
     lambda lat, decl, S0: (lambda ph, de: (lambda x: (lambda h0: {"Q": _r(S0 / math.pi * (h0 * math.sin(ph) * math.sin(de) + math.cos(ph) * math.cos(de) * math.cos(h0))),
                                                                   "day_length_h": _r(24 * h0 / math.pi)})(
         0.0 if x >= 1 else (math.pi if x <= -1 else math.acos(x))))(-math.tan(ph) * math.tan(de)))(math.radians(lat), math.radians(decl)),
     lambda rng: {"lat": rng.uniform(-89, 89), "decl": rng.uniform(-23.44, 23.44), "S0": rng.uniform(1300, 1420)},
     [{"lat": 45, "decl": 23.44, "S0": 1361}, {"lat": 80, "decl": 20, "S0": 1361}],
     ["daily mean insolation", "daily insolation", "insolation"],
     conditions=["|latitude| < 89.9", "|declination| <= 23.5"], method="Q = (S0/pi)(h0 sin phi sin delta + cos phi cos delta sin h0)",
     latex=r"\bar Q = \frac{S_0}{\pi}\left(h_0\sin\varphi\sin\delta + \cos\varphi\cos\delta\sin h_0\right)")


def energy_balance_response(F, lam, C, t):
    _finite(F=F, lam=lam, C=C, t=t)
    _need(lam > 0 and C > 0, "the sensitivity and the heat capacity must be positive")
    _need(0 <= t <= 1e5, "time must be between 0 and 100,000 years")
    _need(F != 0, "a non-zero forcing is needed")
    tau = lam * C / YEAR
    return {"dT": _r(lam * F * (-math.expm1(-t / tau))), "dT_eq": _r(lam * F), "tau_years": _r(tau)}


def _ref_energy_balance(F, lam, C, t):
    # C dT/dt = F - T/lambda integrated over t years from T = 0
    T = _ivp(lambda tt, y: [(F - y[0] / lam) * YEAR / C], t, [0.0])[0] if t > 0 else 0.0
    eq = _ivp(lambda tt, y: [(F - y[0] / lam) * YEAR / C], 60 * lam * C / YEAR, [0.0])[0]
    return {"dT": T, "dT_eq": eq, "tau_years": lam * C / YEAR}


proc("energy_balance_response", "Warming of a one-box energy-balance model", "climate",
     "Temperature response after t years to a constant radiative forcing F in a one-box model C dT/dt = F - T/lambda, "
     "the equilibrium warming lambda F and the e-folding time lambda C.",
     [_FLUX("F", "radiative forcing applied at t = 0", ("forcing", "radiative forcing")),
      I("lam", "climate_sensitivity", "climate_sensitivity", "climate sensitivity parameter", ("sensitivity",
                                                                                              "climate sensitivity"), "K/(W/m^2)"),
      I("C", "heat_capacity_per_area", "heat_capacity_per_area", "effective heat capacity per unit area",
        ("heat capacity",), "J/(m^2*K)"),
      I("t", "number", "years", "time since the forcing began (years)", ("time",), "yr")],
     [O("dT", "temperature_difference", "warming after t (K)"), O("dT_eq", "temperature_difference", "equilibrium warming (K)"),
      O("tau_years", "number", "e-folding time (years)")], energy_balance_response, _ref_energy_balance,
     lambda F, lam, C, t: (lambda tau: {"dT": _r(lam * F * (-math.expm1(-t / tau))), "dT_eq": _r(lam * F), "tau_years": _r(tau)})(C / lam / YEAR),
     lambda rng: {"F": rng.uniform(0.5, 8), "lam": rng.uniform(0.3, 1.2), "C": rng.uniform(1e8, 1.5e9), "t": rng.uniform(1, 80)},
     [{"F": 3.7, "lam": 0.8, "C": 4.2e8, "t": 10}, {"F": 1.5, "lam": 0.5, "C": 8e8, "t": 30}],
     ["energy balance model", "one box energy balance", "energy balance response"],
     conditions=["lambda > 0, C > 0", "the forcing is switched on at t = 0 and then held constant"],
     method="dT(t) = lambda F (1 - exp(-t/(lambda C)))",
     latex=r"\Delta T(t) = \lambda F\left(1-e^{-t/\tau}\right),\quad \tau = \lambda C")


# =============================================================================================== seawater (EOS-80)
def _t68(t):
    return t * 1.00024


def _smow(t):
    T = _t68(t)
    a = (999.842594, 6.793952e-2, -9.095290e-3, 1.001685e-4, -1.120083e-6, 6.536332e-9)
    return a[0] + (a[1] + (a[2] + (a[3] + (a[4] + a[5] * T) * T) * T) * T) * T


def _dens0(s, t):
    T = _t68(t)
    b = (8.24493e-1, -4.0899e-3, 7.6438e-5, -8.2467e-7, 5.3875e-9)
    c = (-5.72466e-3, 1.0227e-4, -1.6546e-6)
    return (_smow(t) + (b[0] + (b[1] + (b[2] + (b[3] + b[4] * T) * T) * T) * T) * s
            + (c[0] + (c[1] + c[2] * T) * T) * s * math.sqrt(s) + 4.8314e-4 * s * s)


def _seck(s, t, p):
    P, T = p / 10.0, _t68(t)
    aw = 3.239908 + (1.43713e-3 + (1.16092e-4 - 5.77905e-7 * T) * T) * T
    bw = 8.50935e-5 + (-6.12293e-6 + 5.2787e-8 * T) * T
    kw = 19652.21 + (148.4206 + (-2.327105 + (1.360477e-2 - 5.155288e-5 * T) * T) * T) * T
    A = aw + (2.2838e-3 + (-1.0981e-5 - 1.6078e-6 * T) * T + 1.91075e-4 * math.sqrt(s)) * s
    B = bw + (-9.9348e-7 + (2.0816e-8 + 9.1697e-10 * T) * T) * s
    k0 = kw + (54.6746 + (-0.603459 + (1.09987e-2 - 6.1670e-5 * T) * T) * T
               + (7.944e-2 + (1.6483e-2 - 5.3009e-4 * T) * T) * math.sqrt(s)) * s
    return k0 + (A + B * P) * P


def _dens(s, t, p):
    return _dens0(s, t) / (1 - (p / 10.0) / _seck(s, t, p))


def _adtg(s, t, p):
    T = _t68(t)
    return (3.5803e-5 + (8.5258e-6 + (-6.836e-8 + 6.6228e-10 * T) * T) * T + (1.8932e-6 - 4.2393e-8 * T) * (s - 35)
            + ((1.8741e-8 + (-6.7795e-10 + (8.733e-12 - 5.4481e-14 * T) * T) * T) + (-1.1351e-10 + 2.7759e-12 * T) * (s - 35)) * p
            + (-4.6206e-13 + (1.8676e-14 - 2.1687e-16 * T) * T) * p * p)


def _ptmp(s, t, p, pr):
    r2 = math.sqrt(2.0)
    dp = pr - p
    dth = dp * _adtg(s, t, p)
    th = _t68(t) + 0.5 * dth
    q = dth
    dth = dp * _adtg(s, th / 1.00024, p + 0.5 * dp)
    th = th + (1 - 1 / r2) * (dth - q)
    q = (2 - r2) * dth + (-2 + 3 / r2) * q
    dth = dp * _adtg(s, th / 1.00024, p + 0.5 * dp)
    th = th + (1 + 1 / r2) * (dth - q)
    q = (2 + r2) * dth + (-2 - 3 / r2) * q
    dth = dp * _adtg(s, th / 1.00024, p + dp)
    return (th + (dth - 2 * q) / 6) / 1.00024


def _svel(s, t, p):
    P, T = p / 10.0, _t68(t)
    cw = ((((-2.3643e-12 * T + 3.8504e-10) * T - 9.7729e-9) * P
           + ((((1.0405e-12 * T - 2.5335e-10) * T + 2.5974e-8) * T - 1.7107e-6) * T + 3.1260e-5)) * P
          + ((((-6.1185e-10 * T + 1.3621e-7) * T - 8.1788e-6) * T + 6.8982e-4) * T + 0.153563)) * P \
        + ((((3.1464e-9 * T - 1.47800e-6) * T + 3.3420e-4) * T - 5.80852e-2) * T + 5.03711) * T + 1402.388
    a = ((((-3.389e-13 * T + 6.649e-12) * T + 1.100e-10) * P + (((7.988e-12 * T - 1.6002e-10) * T + 9.1041e-9) * T - 3.9064e-7)) * P
         + ((((-2.0122e-10 * T + 1.0507e-8) * T - 6.4885e-8) * T - 1.2580e-5) * T + 9.4742e-5)) * P \
        + (((-3.21e-8 * T + 2.006e-6) * T + 7.164e-5) * T - 1.262e-2) * T + 1.389
    b = -1.922e-2 - 4.42e-5 * T + (7.3637e-5 + 1.7945e-7 * T) * P
    d = 1.727e-3 - 7.9836e-6 * P
    return cw + a * s + b * s * math.sqrt(s) + d * s * s


def _sea_ok(S=None, T=None, p=None):
    if S is not None:
        _finite(S=S)
        _need(0 <= S <= 42, "practical salinity must be between 0 and 42")
    if T is not None:
        _finite(T=T)
        _need(271.15 <= T <= 313.15, "temperature must be between -2 and 40 °C")
    if p is not None:
        _finite(p=p)
        _need(0 <= p <= 1e8, "sea pressure must be between 0 and 10,000 dbar")


def _sw():
    import warnings
    warnings.simplefilter("ignore")
    import seawater
    return seawater


_SAL = lambda: I("S", "number", "number", "practical salinity (PSS-78)", ("salinity",), "")    # noqa: E731
_SEAP = lambda sym="p", meaning="sea pressure (0 at the surface)", names=("pressure", "sea pressure"): I(sym, "pressure", "pressure", meaning, names, "dbar")  # noqa: E731


def pure_water_density(T):
    _finite(T=T)
    _need(273.15 <= T <= 313.15, "temperature must be between 0 and 40 °C")
    return {"rho": _r(_smow(T - 273.15))}


proc("pure_water_density", "Density of pure water (SMOW, Bigg 1967)", "oceanography",
     "Density of standard mean ocean water (no salt) at atmospheric pressure, UNESCO 1983 polynomial.",
     [_TEMP("T", "water temperature")], [O("rho", "density", "density (kg/m^3)")], pure_water_density,
     lambda T: {"rho": float(_sw().smow(T - 273.15))},
     lambda T: {"rho": _r(_smow(T))},
     lambda rng: {"T": rng.uniform(273.15, 313.15)},
     [{"T": 4}, {"T": 25}], ["density of pure water", "pure water density"],
     conditions=["0 °C <= T <= 40 °C", "atmospheric pressure"], method="Bigg (1967) polynomial in T68 (UNESCO 1983 Eq. 14)",
     latex=r"\rho_w = \sum_{i=0}^{5} a_i\,T_{68}^{\,i}")


def seawater_density_surface(S, T):
    _sea_ok(S, T)
    rho = _dens0(S, T - 273.15)
    return {"rho": _r(rho), "sigma_t": _r(rho - 1000.0)}


proc("seawater_density_surface", "Density of seawater at the surface (UNESCO 1981)", "oceanography",
     "Density of seawater at atmospheric pressure from practical salinity and temperature (EOS-80), and sigma-t.",
     [_SAL(), _TEMP("T", "water temperature")],
     [O("rho", "density", "density (kg/m^3)"), O("sigma_t", "number", "sigma-t = rho - 1000 (kg/m^3)")],
     seawater_density_surface,
     lambda S, T: (lambda r: {"rho": r, "sigma_t": r - 1000.0})(float(_sw().dens0(S, T - 273.15))),
     lambda S, T: (lambda t: (lambda r: {"rho": _r(r), "sigma_t": _r(r - 1000)})(
         _dens0(S, t) - (-5.72466e-3 + (1.0227e-4 - 1.6546e-6 * _t68(t)) * _t68(t)) * S * math.sqrt(S)))(T - 273.15),
     lambda rng: {"S": rng.uniform(5, 40), "T": rng.uniform(271.5, 313)},
     [{"S": 35, "T": 15}, {"S": 30, "T": 2}], ["density of seawater at the surface", "surface seawater density",
                                              "seawater density at atmospheric pressure"],
     conditions=["0 <= S <= 42", "-2 °C <= T <= 40 °C"], method="UNESCO (1981) one-atmosphere equation of state (EOS-80)",
     latex=r"\rho(S,T,0) = \rho_w + B(T)\,S + C(T)\,S^{3/2} + d\,S^2")


def seawater_density(S, T, p):
    _sea_ok(S, T, p)
    rho = _dens(S, T - 273.15, p / 1e4)
    return {"rho": _r(rho), "sigma": _r(rho - 1000.0)}


proc("seawater_density", "In-situ density of seawater (EOS-80)", "oceanography",
     "In-situ density of seawater from practical salinity, temperature and sea pressure with the secant bulk modulus.",
     [_SAL(), _TEMP("T", "in-situ temperature"), _SEAP()],
     [O("rho", "density", "in-situ density (kg/m^3)"), O("sigma", "number", "rho - 1000 (kg/m^3)")], seawater_density,
     lambda S, T, p: (lambda r: {"rho": r, "sigma": r - 1000.0})(float(_sw().dens(S, T - 273.15, p / 1e4))),
     lambda S, T, p: (lambda r: {"rho": _r(r), "sigma": _r(r - 1000)})(
         _dens0(S, T - 273.15) / (1 - (p / 1e4) / _seck(S, T - 273.15, p / 1e4))),
     lambda rng: {"S": rng.uniform(5, 40), "T": rng.uniform(271.5, 303), "p": rng.uniform(5e5, 6e7)},
     [{"S": 35, "T": 2, "p": 4000}, {"S": 34.5, "T": 10, "p": 1000}], ["in situ density of seawater", "seawater density",
                                                                       "density of seawater"],
     conditions=["0 <= S <= 42", "-2 °C <= T <= 40 °C", "0 <= p <= 10,000 dbar"],
     method="rho = rho(S,T,0)/(1 - P/K(S,T,P)), P in bar (UNESCO 1983)",
     latex=r"\rho(S,T,p) = \frac{\rho(S,T,0)}{1-p/K(S,T,p)}")


def seawater_freezing_point(S, p=0.0):
    _sea_ok(S, None, p)
    _need(4 <= S <= 40, "the freezing-point formula holds for salinities 4 to 40")
    _need(p <= 5e6, "the freezing-point formula holds to 500 dbar")
    P = p / 1e4
    tf = (-0.0575 * S + 1.710523e-3 * S * math.sqrt(S) - 2.154996e-4 * S * S - 7.53e-4 * P) / 1.00024
    return {"Tf": _r(tf + 273.15), "Tf_C": _r(tf)}


proc("seawater_freezing_point", "Freezing point of seawater (UNESCO 1983)", "oceanography",
     "The freezing temperature of seawater from practical salinity and sea pressure (Millero 1978).",
     [_SAL(), I("p", "pressure", "pressure", "sea pressure (0 at the surface; default 0)", ("pressure", "sea pressure"),
                "dbar", required=False)],
     [O("Tf", "temperature", "freezing temperature (K)"), O("Tf_C", "number", "freezing temperature (°C, ITS-90)")],
     seawater_freezing_point,
     lambda S, p=0.0: (lambda c: {"Tf": c + 273.15, "Tf_C": c})(float(_sw().fp(S, p / 1e4))),
     lambda S, p=0.0: (lambda c: {"Tf": _r(c + 273.15), "Tf_C": _r(c)})(
         (-0.0575 * S + 1.710523e-3 * S * math.sqrt(S) - 2.154996e-4 * S * S) / 1.00024),
     lambda rng: {"S": rng.uniform(4, 40), "p": rng.uniform(5e5, 5e6)},
     [{"S": 35, "p": 0}, {"S": 34, "p": 300}], ["freezing point of seawater", "seawater freezing point"],
     conditions=["4 <= S <= 40", "0 <= p <= 500 dbar"], invalid=[{"S": 50, "p": 0}],
     method="t_f = -0.0575 S + 1.710523e-3 S^1.5 - 2.154996e-4 S^2 - 7.53e-4 p (p in dbar)",
     latex=r"t_f = -0.0575S + 1.710523\times10^{-3}S^{3/2} - 2.154996\times10^{-4}S^2 - 7.53\times10^{-4}p")


def sound_speed_seawater(S, T, p):
    _sea_ok(S, T, p)
    return {"c": _r(_svel(S, T - 273.15, p / 1e4))}


proc("sound_speed_seawater", "Speed of sound in seawater (Chen and Millero 1977)", "oceanography",
     "Speed of sound in seawater from practical salinity, temperature and sea pressure (UNESCO 1983 algorithm).",
     [_SAL(), _TEMP("T", "water temperature"), _SEAP()], [O("c", "speed", "speed of sound (m/s)")], sound_speed_seawater,
     lambda S, T, p: {"c": float(_sw().svel(S, T - 273.15, p / 1e4))},
     lambda S, T, p: {"c": _r(_svel(S, T - 273.15, p / 1e5))},
     lambda rng: {"S": rng.uniform(5, 40), "T": rng.uniform(271.5, 308), "p": rng.uniform(1e6, 8e7)},
     [{"S": 35, "T": 10, "p": 1000}, {"S": 35, "T": 2, "p": 5000}], ["speed of sound in seawater",
                                                                     "sound speed in seawater", "sound speed"],
     conditions=["0 <= S <= 40", "0 °C <= T <= 40 °C", "0 <= p <= 10,000 dbar"],
     method="Chen and Millero (1977) polynomial, P in bar (UNESCO 1983 Eq. 33)",
     latex=r"c(S,T,P) = C_w(T,P) + A(T,P)\,S + B(T,P)\,S^{3/2} + D(P)\,S^2")


def ocean_pressure(depth, lat):
    _finite(depth=depth)
    _lat_ok(lat)
    _need(0 <= depth <= 11000, "depth must be between 0 and 11,000 m")
    c1 = 5.92e-3 + math.sin(abs(math.radians(lat))) ** 2 * 5.25e-3
    pd = ((1 - c1) - math.sqrt((1 - c1) ** 2 - 8.84e-6 * depth)) / 4.42e-6
    return {"p": _r(pd * 1e4), "p_dbar": _r(pd)}


proc("ocean_pressure_from_depth", "Sea pressure at a depth (Saunders 1981)", "oceanography",
     "Sea pressure at a depth in the ocean, allowing for latitude (Saunders 1981).",
     [I("depth", "length", "length", "depth below the sea surface", ("depth",), "m"), _LAT()],
     [O("p", "pressure", "sea pressure (Pa)"), O("p_dbar", "number", "sea pressure (dbar)")], ocean_pressure,
     lambda depth, lat: (lambda pd: {"p": pd * 1e4, "p_dbar": pd})(float(_sw().pres(depth, lat))),
     lambda depth, lat: {"p": _r(1025 * 9.81 * depth), "p_dbar": _r(1025 * 9.81 * depth / 1e4)},
     lambda rng: {"depth": rng.uniform(10, 10000), "lat": rng.uniform(-80, 80)},
     [{"depth": 1000, "lat": 30}, {"depth": 5000, "lat": 60}], ["pressure at depth in the ocean", "ocean pressure",
                                                                "sea pressure from depth"],
     conditions=["0 <= depth <= 11,000 m"], method="Saunders (1981): p = [(1 - c1) - sqrt((1 - c1)^2 - 8.84e-6 z)]/4.42e-6",
     latex=r"p = \frac{(1-c_1)-\sqrt{(1-c_1)^2-8.84\times10^{-6}z}}{4.42\times10^{-6}}")


def ocean_depth(p, lat):
    _finite(p=p)
    _lat_ok(lat)
    _need(0 <= p <= 1.2e8, "sea pressure must be between 0 and 12,000 dbar")
    P = p / 1e4
    x = math.sin(math.radians(abs(lat))) ** 2
    bot = 9.780318 * (1.0 + (5.2788e-3 + 2.36e-5 * x) * x) + 2.184e-6 * 0.5 * P
    top = (((-1.82e-15 * P + 2.279e-10) * P - 2.2512e-5) * P + 9.72659) * P
    return {"depth": _r(top / bot)}


proc("ocean_depth_from_pressure", "Depth from sea pressure (Fofonoff and Millard 1983)", "oceanography",
     "Depth in the ocean from sea pressure and latitude (UNESCO 1983 standard algorithm).",
     [_SEAP(), _LAT()], [O("depth", "length", "depth (m)")], ocean_depth,
     lambda p, lat: {"depth": float(_sw().dpth(p / 1e4, lat))},
     lambda p, lat: {"depth": _r(p / (1025 * 9.81))},
     lambda rng: {"p": rng.uniform(1e5, 1e8), "lat": rng.uniform(-80, 80)},
     [{"p": 1000, "lat": 30}, {"p": 5000, "lat": 45}], ["depth from pressure", "depth from sea pressure"],
     conditions=["0 <= p <= 12,000 dbar"], method="z = (9.72659 p - 2.2512e-5 p^2 + 2.279e-10 p^3 - 1.82e-15 p^4)/(g(phi) + 1.092e-6 p)",
     latex=r"z = \frac{\sum c_i p^i}{g(\varphi) + \tfrac12\gamma' p}")


def seawater_adiabatic_gradient(S, T, p):
    _sea_ok(S, T, p)
    return {"gamma": _r(_adtg(S, T - 273.15, p / 1e4)), "gamma_per_km_dbar": _r(1000 * _adtg(S, T - 273.15, p / 1e4))}


proc("seawater_adiabatic_gradient", "Adiabatic temperature gradient of seawater (Bryden 1973)", "oceanography",
     "How fast the temperature of seawater rises under adiabatic compression, per decibar.",
     [_SAL(), _TEMP("T", "in-situ temperature"), _SEAP()],
     [O("gamma", "number", "adiabatic gradient (K per dbar)"), O("gamma_per_km_dbar", "number", "adiabatic gradient (K per 1000 dbar)")],
     seawater_adiabatic_gradient,
     lambda S, T, p: (lambda g: {"gamma": g, "gamma_per_km_dbar": 1000 * g})(float(_sw().adtg(S, T - 273.15, p / 1e4))),
     lambda S, T, p: (lambda g: {"gamma": _r(g), "gamma_per_km_dbar": _r(1000 * g)})(_adtg(-S, T - 273.15, p / 1e4)),
     lambda rng: {"S": rng.uniform(30, 40), "T": rng.uniform(272, 303), "p": rng.uniform(0, 6e7)},
     [{"S": 35, "T": 10, "p": 1000}, {"S": 34.7, "T": 2, "p": 4000}], ["adiabatic temperature gradient of seawater",
                                                                       "adiabatic lapse rate of seawater"],
     conditions=["0 <= S <= 42", "-2 °C <= T <= 40 °C"], method="Bryden (1973) polynomial (UNESCO 1983 Eq. 30)",
     latex=r"\Gamma(S,T,p) = \left(\frac{\partial T}{\partial p}\right)_{\eta}")


def seawater_potential_temperature(S, T, p, pr=0.0):
    _sea_ok(S, T, p)
    _sea_ok(None, None, pr)
    t = T - 273.15
    th = _ptmp(S, t, p / 1e4, pr / 1e4)
    return {"theta": _r(th + 273.15), "theta_C": _r(th), "dtheta": _r(th - t)}


proc("seawater_potential_temperature", "Potential temperature of seawater (Fofonoff 1977)", "oceanography",
     "The temperature a seawater parcel would have if moved adiabatically to a reference pressure (default the "
     "surface), by a fourth-order Runge-Kutta integration of the adiabatic gradient.",
     [_SAL(), _TEMP("T", "in-situ temperature"), _SEAP(),
      I("pr", "pressure", "pressure", "reference sea pressure (default 0, the surface)", ("reference pressure",), "dbar",
        required=False)],
     [O("theta", "temperature", "potential temperature (K)"), O("theta_C", "number", "potential temperature (°C)"),
      O("dtheta", "temperature_difference", "theta - T (K)")], seawater_potential_temperature,
     lambda S, T, p, pr=0.0: (lambda th: {"theta": th + 273.15, "theta_C": th, "dtheta": th - (T - 273.15)})(
         float(_sw().ptmp(S, T - 273.15, p / 1e4, pr / 1e4))),
     lambda S, T, p, pr=0.0: (lambda t: (lambda th: {"theta": _r(th + 273.15), "theta_C": _r(th), "dtheta": _r(th - t)})(
         t + (pr - p) / 1e4 * _adtg(S, t, p / 1e4)))(T - 273.15),
     lambda rng: {"S": rng.uniform(33, 37), "T": rng.uniform(273.5, 300), "p": rng.uniform(5e6, 6e7), "pr": 0.0},
     [{"S": 35, "T": 3, "p": 5000, "pr": 0}, {"S": 34.7, "T": 10, "p": 2000, "pr": 0}],
     ["potential temperature of seawater", "seawater potential temperature", "ocean potential temperature"],
     conditions=["0 <= S <= 42", "-2 °C <= T <= 40 °C"],
     method="Fofonoff (1977) fourth-order Runge-Kutta integration of the adiabatic gradient from p to p_r",
     latex=r"\theta = T + \int_{p}^{p_r}\Gamma(S,\theta',p')\,dp'")


def gravity_latitude(lat, z=0.0):
    _lat_ok(lat)
    _finite(z=z)
    _need(-11000 <= z <= 100000, "height must be between -11 km and 100 km")
    x = math.sin(math.radians(abs(lat))) ** 2
    g = 9.780318 * (1.0 + (5.2788e-3 + 2.36e-5 * x) * x) / (1 + z / 6371000.0) ** 2
    return {"g": _r(g)}


proc("gravity_at_latitude", "Gravitational acceleration at a latitude and height", "geophysics",
     "Normal gravity at sea level for a latitude (International Gravity Formula 1967, as UNESCO 1983) reduced "
     "for height with the inverse-square law.",
     [_LAT(), I("z", "length", "length", "height above sea level (default 0)", ("height", "altitude"), "m", required=False)],
     [O("g", "acceleration", "gravitational acceleration (m/s^2)")], gravity_latitude,
     lambda lat, z=0.0: {"g": float(_sw().g(lat, z))},
     lambda lat, z=0.0: {"g": _r(9.780318 * (1.0 + (5.2788e-3 + 2.36e-5 * math.sin(math.radians(lat)) ** 2) * math.sin(math.radians(lat)) ** 2))},
     lambda rng: {"lat": rng.uniform(-90, 90), "z": rng.uniform(100, 20000)},
     [{"lat": 45, "z": 0}, {"lat": 0, "z": 4000}], ["gravity at latitude", "normal gravity", "gravitational acceleration at"],
     conditions=["-90 <= latitude <= 90", "spherical inverse-square reduction for height (R = 6371 km)"],
     method="g = 9.780318 (1 + 5.2788e-3 sin^2 phi + 2.36e-5 sin^4 phi)/(1 + z/R)^2",
     latex=r"g = \frac{9.780318\,(1+5.2788\times10^{-3}\sin^2\varphi+2.36\times10^{-5}\sin^4\varphi)}{(1+z/R)^2}")


# =============================================================================================== ocean dynamics
def shallow_water_wave(h, distance=None):
    _finite(h=h)
    _need(0 < h <= 11000, "the depth must be between 0 and 11,000 m")
    c = math.sqrt(G0 * h)
    out = {"c": _r(c)}
    if distance is not None:
        _finite(distance=distance)
        _need(distance > 0, "the distance must be positive")
        out["travel_time"] = _r(distance / c)
        out["travel_time_h"] = _r(distance / c / 3600)
    return out


def _ref_shallow(h, distance=None):
    import numpy as np
    # characteristic speeds of the linearised shallow-water equations (eta_t + h u_x = 0, u_t + g eta_x = 0):
    # eigenvalues of the flux Jacobian
    c = float(np.max(np.linalg.eigvals(np.array([[0.0, h], [G0, 0.0]])).real))
    out = {"c": c}
    if distance is not None:
        out["travel_time"] = distance / c
        out["travel_time_h"] = distance / c / 3600
    return out


proc("shallow_water_wave", "Speed of a shallow-water (long) wave and tsunami travel time", "ocean dynamics",
     "Speed sqrt(g h) of a wave much longer than the water depth (tsunami, tide), and its travel time over a distance.",
     [I("h", "length", "length", "water depth", ("depth", "water depth"), "m"),
      I("distance", "length", "length", "distance travelled (optional)", ("distance",), "km", required=False)],
     [O("c", "speed", "wave speed (m/s)"), O("travel_time", "time", "travel time (s)"),
      O("travel_time_h", "number", "travel time (hours)")], shallow_water_wave, _ref_shallow,
     lambda h, distance=None: (lambda c: {"c": _r(c), **({"travel_time": _r(distance / c), "travel_time_h": _r(distance / c / 3600)}
                                                       if distance is not None else {})})(math.sqrt(2 * G0 * h)),
     lambda rng: {"h": rng.uniform(1, 8000), "distance": rng.uniform(1e4, 1e7)},
     [{"h": 4000, "distance": 5000}, {"h": 10, "distance": 1}], ["shallow water wave", "tsunami", "long wave speed"],
     conditions=["wavelength much longer than the depth", "standard gravity"], method="c = sqrt(g h), t = d/c",
     latex=r"c = \sqrt{g h},\quad t = d/c")


def ekman_depth(Az, lat):
    _finite(Az=Az)
    _lat_ok(lat)
    _need(5 <= abs(lat) <= 89, "Ekman layers are used here for 5 to 89 degrees latitude")
    _need(1e-5 <= Az <= 10, "the vertical eddy viscosity must be between 1e-5 and 10 m^2/s")
    f = abs(2 * OMEGA * math.sin(math.radians(lat)))
    d = math.sqrt(2 * Az / f)
    return {"D_E": _r(math.pi * d), "delta": _r(d)}


def _ref_ekman_depth(Az, lat):
    import numpy as np
    mc, u = _metpy()
    f = float(mc.coriolis_parameter(lat * u.deg).m_as("1/s"))
    # Ekman's equation i f W = Az W'' for W = u + i v: characteristic roots of Az s^2 - i f = 0; the decaying one
    r = np.roots([Az, 0.0, -1j * f])
    s = r[np.argmax(r.real)]
    return {"D_E": math.pi / s.real, "delta": 1 / s.real}


proc("ekman_depth", "Ekman layer depth", "ocean dynamics",
     "Depth of frictional influence of the wind on the ocean (the current reverses direction at D_E = pi delta).",
     [I("Az", "kinematic_viscosity", "kinematic_viscosity", "vertical eddy viscosity", ("eddy viscosity",), "m^2/s"), _LAT()],
     [O("D_E", "length", "Ekman depth pi delta (m)"), O("delta", "length", "Ekman e-folding depth sqrt(2 Az/|f|) (m)")],
     ekman_depth, _ref_ekman_depth,
     lambda Az, lat: (lambda f: {"D_E": _r(math.pi * math.sqrt(Az / f)), "delta": _r(math.sqrt(Az / f))})(
         abs(2 * OMEGA * math.sin(math.radians(lat)))),
     lambda rng: {"Az": rng.uniform(1e-3, 0.5), "lat": rng.choice([-1, 1]) * rng.uniform(10, 85)},
     [{"Az": 0.05, "lat": 45}, {"Az": 0.01, "lat": -30}], ["ekman depth", "ekman layer depth"],
     conditions=["5 <= |latitude| <= 89", "constant eddy viscosity", "deep ocean"],
     method="D_E = pi sqrt(2 Az/|f|)", latex=r"D_E = \pi\sqrt{\frac{2A_z}{|f|}}")


def ekman_transport(tau, lat, rho=1025.0):
    _finite(tau=tau, rho=rho)
    _lat_ok(lat)
    _need(5 <= abs(lat) <= 89, "Ekman transport is used here for 5 to 89 degrees latitude")
    _need(tau > 0, "the wind stress must be positive")
    _need(900 <= rho <= 1100, "the water density must be between 900 and 1100 kg/m^3")
    f = abs(2 * OMEGA * math.sin(math.radians(lat)))
    return {"M": _r(tau / (rho * f)), "mass_transport": _r(tau / f)}


def _ref_ekman_transport(tau, lat, rho=1025.0):
    import cmath
    from scipy.integrate import quad
    f = 2 * OMEGA * math.sin(math.radians(lat))
    Az = 0.05                                                  # any eddy viscosity: the transport does not depend on it
    s = cmath.sqrt(1j * f / Az)
    s = s if s.real > 0 else -s
    A = tau / (rho * Az * s)                                    # surface condition rho Az W'(0) = tau
    depth = 80 / s.real                                         # exp(-80): the spiral has vanished
    cuts = [-depth * i / 16 for i in range(16, -1, -1)]
    re = sum(quad(lambda z: (A * cmath.exp(s * z)).real, a, b, epsabs=0, epsrel=1e-13, limit=200)[0] for a, b in zip(cuts, cuts[1:]))
    im = sum(quad(lambda z: (A * cmath.exp(s * z)).imag, a, b, epsabs=0, epsrel=1e-13, limit=200)[0] for a, b in zip(cuts, cuts[1:]))
    M = math.hypot(re, im)
    return {"M": M, "mass_transport": M * rho}


proc("ekman_transport", "Ekman transport", "ocean dynamics",
     "Volume transport per unit width of the wind-driven Ekman layer, directed 90 degrees to the right of the wind in "
     "the northern hemisphere (left in the southern).",
     [I("tau", "shear_stress", "shear_stress", "wind stress on the sea surface", ("wind stress",), "Pa"), _LAT(),
      I("rho", "density", "density", "seawater density (default 1025 kg/m^3)", ("density",), "kg/m^3", required=False)],
     [O("M", "volume_transport_per_width", "Ekman volume transport per unit width (m^2/s)"),
      O("mass_transport", "mass_transport_per_width", "Ekman mass transport per unit width (kg/(m s))")],
     ekman_transport, _ref_ekman_transport,
     lambda tau, lat, rho=1025.0: (lambda f: {"M": _r(tau / f), "mass_transport": _r(tau * rho / f)})(
         abs(2 * OMEGA * math.sin(math.radians(lat)))),
     lambda rng: {"tau": rng.uniform(0.01, 0.5), "lat": rng.choice([-1, 1]) * rng.uniform(10, 85), "rho": rng.uniform(1020, 1030)},
     [{"tau": 0.1, "lat": 45, "rho": 1025}, {"tau": 0.2, "lat": -30, "rho": 1027}], ["ekman transport"],
     conditions=["5 <= |latitude| <= 89", "steady state, deep ocean"], method="M = tau/(rho |f|)",
     latex=r"M_E = \frac{\tau}{\rho\,|f|}")


# =============================================================================================== seismology
def _christoffel(lam, mu, rho, n=(1.0, 2.0, 3.0)):
    """Phase speeds of plane waves along n in an isotropic medium: eigenvalues of the Christoffel matrix
    Gamma_ik = C_ijkl n_j n_l / rho of the full stiffness tensor (ascending: S, S, P)."""
    import numpy as np
    n = np.array(n, float)
    n /= np.linalg.norm(n)
    d = np.eye(3)
    C = lam * np.einsum("ij,kl->ijkl", d, d) + mu * (np.einsum("ik,jl->ijkl", d, d) + np.einsum("il,jk->ijkl", d, d))
    G = np.einsum("ijkl,j,l->ik", C, n, n) / rho
    return np.sqrt(np.linalg.eigvalsh(G))


_MOD = lambda sym, kind, meaning, names: I(sym, kind, "modulus", meaning, names, "GPa")   # noqa: E731
_RHO = lambda meaning="density of the rock", names=("density",), show="kg/m^3", **kw: I("rho", "density", "density", meaning, names, show, **kw)  # noqa: E731


def velocities_from_moduli(K, mu, rho):
    _finite(K=K, mu=mu, rho=rho)
    _need(K > 0 and mu > 0 and rho > 0, "bulk modulus, shear modulus and density must be positive")
    vp, vs = math.sqrt((K + 4 * mu / 3) / rho), math.sqrt(mu / rho)
    return {"Vp": _r(vp), "Vs": _r(vs), "Vp_Vs": _r(vp / vs)}


def _ref_vel_moduli(K, mu, rho):
    v = _christoffel(K - 2 * mu / 3, mu, rho)
    return {"Vp": float(v[2]), "Vs": float(v[0]), "Vp_Vs": float(v[2] / v[0])}


proc("seismic_velocities_from_moduli", "P- and S-wave speeds from bulk and shear moduli", "seismology",
     "Body-wave speeds of an isotropic elastic rock from its bulk modulus, shear modulus and density.",
     [_MOD("K", "bulk_modulus", "bulk modulus (incompressibility)", ("bulk modulus",)),
      _MOD("mu", "shear_modulus", "shear modulus (rigidity)", ("shear modulus", "rigidity")), _RHO()],
     [O("Vp", "speed", "P-wave speed (m/s)"), O("Vs", "speed", "S-wave speed (m/s)"), O("Vp_Vs", "number", "Vp/Vs")],
     velocities_from_moduli, _ref_vel_moduli,
     lambda K, mu, rho: (lambda vp, vs: {"Vp": _r(vp), "Vs": _r(vs), "Vp_Vs": _r(vp / vs)})(math.sqrt((K + mu) / rho), math.sqrt(mu / rho)),
     lambda rng: {"K": rng.uniform(1e10, 2e11), "mu": rng.uniform(5e9, 1e11), "rho": rng.uniform(2000, 5000)},
     [{"K": 50, "mu": 30, "rho": 2700}, {"K": 130, "mu": 70, "rho": 3300}],
     ["seismic velocities from moduli", "wave speeds from moduli", "p and s wave speeds from moduli"],
     conditions=["isotropic, linear elastic", "K > 0, mu > 0"], method="Vp = sqrt((K + 4 mu/3)/rho), Vs = sqrt(mu/rho)",
     latex=r"V_P = \sqrt{\frac{K+\tfrac43\mu}{\rho}},\quad V_S = \sqrt{\frac{\mu}{\rho}}")


def velocities_from_young(E, nu, rho):
    _finite(E=E, nu=nu, rho=rho)
    _need(E > 0 and rho > 0, "Young's modulus and density must be positive")
    _need(-1 < nu < 0.5, "Poisson's ratio must be between -1 and 0.5")
    lam, mu = E * nu / ((1 + nu) * (1 - 2 * nu)), E / (2 * (1 + nu))
    vp, vs = math.sqrt((lam + 2 * mu) / rho), math.sqrt(mu / rho)
    return {"Vp": _r(vp), "Vs": _r(vs), "Vp_Vs": _r(vp / vs)}


def _ref_vel_young(E, nu, rho):
    lam, mu = E * nu / ((1 + nu) * (1 - 2 * nu)), E / (2 * (1 + nu))
    v = _christoffel(lam, mu, rho)
    return {"Vp": float(v[2]), "Vs": float(v[0]), "Vp_Vs": float(v[2] / v[0])}


proc("seismic_velocities_from_young", "P- and S-wave speeds from Young's modulus and Poisson's ratio", "seismology",
     "Body-wave speeds of an isotropic elastic rock from Young's modulus, Poisson's ratio and density.",
     [_MOD("E", "youngs_modulus", "Young's modulus", ("young's modulus", "youngs modulus")),
      I("nu", "number", "number", "Poisson's ratio", ("poisson's ratio", "poisson ratio"), ""), _RHO()],
     [O("Vp", "speed", "P-wave speed (m/s)"), O("Vs", "speed", "S-wave speed (m/s)"), O("Vp_Vs", "number", "Vp/Vs")],
     velocities_from_young, _ref_vel_young,
     lambda E, nu, rho: (lambda vs: {"Vp": _r(math.sqrt(E / rho)), "Vs": _r(vs), "Vp_Vs": _r(math.sqrt(E / rho) / vs)})(
         math.sqrt(E / (2 * (1 + nu)) / rho)),
     lambda rng: {"E": rng.uniform(1e10, 2e11), "nu": rng.uniform(0.05, 0.45), "rho": rng.uniform(2000, 5000)},
     [{"E": 70, "nu": 0.25, "rho": 2700}, {"E": 160, "nu": 0.28, "rho": 3300}],
     ["seismic velocities from young's modulus", "wave speeds from young's modulus"],
     conditions=["isotropic, linear elastic", "-1 < nu < 0.5"], invalid=[{"E": 70, "nu": 0.6, "rho": 2700}],
     method="Lamé constants from E and nu, then Vp = sqrt((lambda + 2 mu)/rho), Vs = sqrt(mu/rho)",
     latex=r"V_P = \sqrt{\frac{E(1-\nu)}{\rho(1+\nu)(1-2\nu)}},\quad V_S = \sqrt{\frac{E}{2\rho(1+\nu)}}")


def elastic_from_velocities(Vp, Vs, rho):
    _finite(Vp=Vp, Vs=Vs, rho=rho)
    _need(Vs > 0 and rho > 0, "Vs and density must be positive")
    _need(Vp * Vp > 4 * Vs * Vs / 3 * 1.0000001, "Vp must exceed sqrt(4/3) Vs (a positive bulk modulus)")
    mu = rho * Vs * Vs
    lam = rho * (Vp * Vp - 2 * Vs * Vs)
    K = lam + 2 * mu / 3
    nu = (Vp * Vp - 2 * Vs * Vs) / (2 * (Vp * Vp - Vs * Vs))
    return {"mu": _r(mu), "lam": _r(lam), "K": _r(K), "E": _r(2 * mu * (1 + nu)), "nu": _r(nu)}


def _ref_elastic(Vp, Vs, rho):
    import numpy as np
    n = np.array([1.0, 2.0, 3.0]) / math.sqrt(14.0)
    t = np.array([2.0, -1.0, 0.0]) / math.sqrt(5.0)                      # perpendicular to n
    d = np.eye(3)
    G = lambda lam, mu: (lam * np.einsum("ij,kl->ijkl", d, d) + mu * (np.einsum("ik,jl->ijkl", d, d) + np.einsum("il,jk->ijkl", d, d)))  # noqa: E731
    Gl = np.einsum("ijkl,j,l->ik", G(1.0, 0.0), n, n)
    Gm = np.einsum("ijkl,j,l->ik", G(0.0, 1.0), n, n)
    A = np.array([[n @ Gl @ n, n @ Gm @ n], [t @ Gl @ t, t @ Gm @ t]])   # P along n, S polarised along t
    lam, mu = np.linalg.solve(A, rho * np.array([Vp * Vp, Vs * Vs]))
    ratio = Vp / Vs
    nu = _brentq(lambda x: (lambda v: v[2] / v[0])(_christoffel(x / ((1 + x) * (1 - 2 * x)), 1 / (2 * (1 + x)), 1.0)) - ratio,
                 -0.999, 0.4999999)
    return {"mu": float(mu), "lam": float(lam), "nu": nu}


proc("elastic_constants_from_velocities", "Elastic moduli and Poisson's ratio from Vp, Vs and density", "seismology",
     "Shear modulus, Lamé's lambda, bulk modulus, Young's modulus and Poisson's ratio of an isotropic rock from its "
     "seismic velocities and density.",
     [I("Vp", "speed", "speed", "P-wave speed", ("p wave speed", "p-wave speed", "vp"), "km/s"),
      I("Vs", "speed", "speed", "S-wave speed", ("s wave speed", "s-wave speed", "vs"), "km/s"), _RHO()],
     [O("mu", "shear_modulus", "shear modulus (Pa)"), O("lam", "lame_lambda", "Lamé's first parameter (Pa)"),
      O("K", "bulk_modulus", "bulk modulus (Pa)"), O("E", "youngs_modulus", "Young's modulus (Pa)"),
      O("nu", "number", "Poisson's ratio")], elastic_from_velocities, _ref_elastic,
     lambda Vp, Vs, rho: (lambda mu, lam, nu: {"mu": _r(mu), "lam": _r(lam), "K": _r(lam + 2 * mu / 3), "E": _r(2 * mu * (1 + nu)), "nu": _r(nu)})(
         rho * Vs * Vs, rho * (Vp * Vp - 2 * Vs * Vs), (Vp * Vp - 2 * Vs * Vs) / (Vp * Vp - Vs * Vs)),
     lambda rng: (lambda vs: {"Vs": vs, "Vp": vs * rng.uniform(1.4, 2.5), "rho": rng.uniform(2000, 5000)})(rng.uniform(1000, 5000)),
     [{"Vp": 6, "Vs": 3.46, "rho": 2700}, {"Vp": 8.1, "Vs": 4.5, "rho": 3350}],
     ["elastic constants from velocities", "elastic moduli from seismic velocities", "poisson's ratio from velocities"],
     conditions=["isotropic, linear elastic", "Vp > sqrt(4/3) Vs"], invalid=[{"Vp": 3, "Vs": 3, "rho": 2700}],
     method="mu = rho Vs^2, lambda = rho (Vp^2 - 2 Vs^2), K = lambda + 2 mu/3, nu = (Vp^2 - 2Vs^2)/(2(Vp^2 - Vs^2))",
     latex=r"\mu = \rho V_S^2,\quad \lambda = \rho(V_P^2-2V_S^2),\quad \nu = \frac{V_P^2-2V_S^2}{2(V_P^2-V_S^2)}")


def _coord_ok(**kw):
    for k, v in kw.items():
        _finite(**{k: v})
        lim = 90 if k.startswith("lat") else 360
        _need(-lim <= v <= lim, f"{k} must be between -{lim} and {lim} degrees")


def epicentral_distance(lat1, lon1, lat2, lon2):
    _coord_ok(lat1=lat1, lon1=lon1, lat2=lat2, lon2=lon2)
    p1, p2, dl = math.radians(lat1), math.radians(lat2), math.radians(lon2 - lon1)
    num = math.hypot(math.cos(p2) * math.sin(dl), math.cos(p1) * math.sin(p2) - math.sin(p1) * math.cos(p2) * math.cos(dl))
    den = math.sin(p1) * math.sin(p2) + math.cos(p1) * math.cos(p2) * math.cos(dl)
    d = math.degrees(math.atan2(num, den))
    return {"delta": _r(d), "distance_km": _r(d * 2 * math.pi * R_EARTH_KM / 360)}


_LATN = lambda sym, meaning, names: I(sym, "number", "degrees", meaning, names, "°")   # noqa: E731
_PTS = [_LATN("lat1", "latitude of the first point (epicentre)", ("epicentre latitude", "epicenter latitude", "latitude 1")),
        _LATN("lon1", "longitude of the first point (epicentre)", ("epicentre longitude", "epicenter longitude", "longitude 1")),
        _LATN("lat2", "latitude of the second point (station)", ("station latitude", "latitude 2")),
        _LATN("lon2", "longitude of the second point (station)", ("station longitude", "longitude 2"))]


def _rand_pts(rng, lo=1.0, hi=150.0):
    while True:
        x = {"lat1": rng.uniform(-80, 80), "lon1": rng.uniform(-180, 180), "lat2": rng.uniform(-80, 80), "lon2": rng.uniform(-180, 180)}
        d = epicentral_distance(**x)["delta"]
        if lo <= d <= hi:
            return x


proc("epicentral_distance", "Epicentral distance on a spherical Earth", "seismology",
     "Great-circle angular distance between an epicentre and a station, and its length on a sphere of radius 6371 km.",
     _PTS, [O("delta", "number", "epicentral distance (degrees)"), O("distance_km", "number", "great-circle distance (km)")],
     epicentral_distance,
     lambda lat1, lon1, lat2, lon2: (lambda d: {"delta": d, "distance_km": __import__("obspy.geodetics", fromlist=["x"]).degrees2kilometers(d)})(
         float(__import__("obspy.geodetics", fromlist=["x"]).locations2degrees(lat1, lon1, lat2, lon2))),
     lambda lat1, lon1, lat2, lon2: (lambda d: {"delta": _r(d), "distance_km": _r(d * 111.19492664455873)})(math.hypot(lat2 - lat1, lon2 - lon1)),
     lambda rng: _rand_pts(rng),
     [{"lat1": 35.7, "lon1": 139.7, "lat2": 51.5, "lon2": -0.1}, {"lat1": 0, "lon1": 0, "lat2": 30, "lon2": 40}],
     ["epicentral distance", "great circle distance", "angular distance between"],
     conditions=["spherical Earth of radius 6371 km"], method="great-circle angle by atan2 (Vincenty's formula on the sphere)",
     latex=r"\Delta = \operatorname{atan2}\!\left(\sqrt{(\cos\varphi_2\sin\Delta\lambda)^2+(\cos\varphi_1\sin\varphi_2-\sin\varphi_1\cos\varphi_2\cos\Delta\lambda)^2},\ \sin\varphi_1\sin\varphi_2+\cos\varphi_1\cos\varphi_2\cos\Delta\lambda\right)")


def vincenty_inverse(lat1, lon1, lat2, lon2):
    _coord_ok(lat1=lat1, lon1=lon1, lat2=lat2, lon2=lon2)
    sph = epicentral_distance(lat1, lon1, lat2, lon2)["delta"]
    _need(0.01 <= sph <= 120, "the points must be 0.01 to 120 degrees apart (Vincenty's iteration converges slowly towards antipodes)")
    _need(not (abs(lat1) < 1e-9 and abs(lat2) < 1e-9), "two points on the equator are not supported")
    a, f = 6378137.0, 1 / 298.257223563
    b = a * (1 - f)
    norm = lambda x: (x + 180.0) % 360.0 - 180.0                           # noqa: E731
    L = math.radians(norm(lon2) - norm(lon1))
    U1, U2 = math.atan((1 - f) * math.tan(math.radians(lat1))), math.atan((1 - f) * math.tan(math.radians(lat2)))
    sU1, cU1, sU2, cU2 = math.sin(U1), math.cos(U1), math.sin(U2), math.cos(U2)
    lam = L
    for _ in range(500):
        sl, cl = math.sin(lam), math.cos(lam)
        ss = math.hypot(cU2 * sl, cU1 * sU2 - sU1 * cU2 * cl)
        cs = sU1 * sU2 + cU1 * cU2 * cl
        sig = math.atan2(ss, cs)
        sa = cU1 * cU2 * sl / ss
        c2a = 1 - sa * sa
        c2sm = cs - 2 * sU1 * sU2 / c2a
        C = f / 16 * c2a * (4 + f * (4 - 3 * c2a))
        new = L + (1 - C) * f * sa * (sig + C * ss * (c2sm + C * cs * (-1 + 2 * c2sm * c2sm)))
        if abs(new - lam) < 1e-15:
            lam = new
            break
        lam = new
    else:
        raise ValueError("Vincenty's iteration did not converge")
    sl, cl = math.sin(lam), math.cos(lam)
    ss = math.hypot(cU2 * sl, cU1 * sU2 - sU1 * cU2 * cl)
    cs = sU1 * sU2 + cU1 * cU2 * cl
    sig = math.atan2(ss, cs)
    sa = cU1 * cU2 * sl / ss
    c2a = 1 - sa * sa
    c2sm = cs - 2 * sU1 * sU2 / c2a
    u2 = c2a * (a * a - b * b) / (b * b)
    A = 1 + u2 / 16384 * (4096 + u2 * (-768 + u2 * (320 - 175 * u2)))
    B = u2 / 1024 * (256 + u2 * (-128 + u2 * (74 - 47 * u2)))
    ds = B * ss * (c2sm + B / 4 * (cs * (-1 + 2 * c2sm * c2sm) - B / 6 * c2sm * (-3 + 4 * ss * ss) * (-3 + 4 * c2sm * c2sm)))
    s = b * A * (sig - ds)
    a1 = math.degrees(math.atan2(cU2 * sl, cU1 * sU2 - sU1 * cU2 * cl)) % 360.0
    a2 = (math.degrees(math.atan2(cU1 * sl, -sU1 * cU2 + cU1 * sU2 * cl)) + 180.0) % 360.0
    return {"distance": _r(s), "azimuth": _r(a1), "back_azimuth": _r(a2)}


def _ref_vincenty(lat1, lon1, lat2, lon2):
    from obspy.geodetics.base import calc_vincenty_inverse
    d, a1, a2 = calc_vincenty_inverse(lat1, lon1, lat2, lon2)
    return {"distance": d, "azimuth": a1, "back_azimuth": a2}


proc("geodesic_distance_azimuth", "Geodesic distance and azimuths on the WGS84 ellipsoid (Vincenty)", "seismology",
     "Distance along the WGS84 ellipsoid between two points, the azimuth from the first to the second and the "
     "back-azimuth, by Vincenty's inverse method.",
     _PTS, [O("distance", "length", "geodesic distance (m)"), O("azimuth", "number", "azimuth at point 1 (degrees from north)"),
            O("back_azimuth", "number", "back-azimuth from point 2 (degrees from north)")],
     vincenty_inverse, _ref_vincenty,
     lambda lat1, lon1, lat2, lon2: (lambda d: {"distance": _r(d * 6371000 * math.pi / 180), "azimuth": 0.0, "back_azimuth": 0.0})(
         epicentral_distance(lat1, lon1, lat2, lon2)["delta"]),
     lambda rng: _rand_pts(rng, 1, 120),
     [{"lat1": 35.7, "lon1": 139.7, "lat2": 51.5, "lon2": -0.1}, {"lat1": -33.9, "lon1": 18.4, "lat2": 40.7, "lon2": -74.0}],
     ["geodesic distance", "vincenty", "distance and azimuth", "azimuth and distance"],
     conditions=["WGS84 ellipsoid (a = 6378137 m, f = 1/298.257223563)", "0.01 to 120 degrees apart"],
     method="Vincenty (1975) inverse iteration on the reduced latitudes", latex=r"s = bA(\sigma-\Delta\sigma)")


def moment_magnitude(M0):
    _finite(M0=M0)
    _need(1e6 <= M0 <= 1e25, "the seismic moment must be between 1e6 and 1e25 N m")
    return {"Mw": _r(math.log10(M0 * 1e7) / 1.5 - 10.7), "M0_dyne_cm": _r(M0 * 1e7)}


def _ref_mw(M0):
    import warnings
    warnings.simplefilter("ignore")
    from obspy.imaging.scripts.mopad import MomentTensor
    mt = MomentTensor([M0, -M0, 0.0, 0.0, 0.0, 0.0])                    # a pure double couple of scalar moment M0
    return {"Mw": float(mt._moment_magnitude), "M0_dyne_cm": float(mt.get_moment()) * 1e7}


proc("moment_magnitude", "Moment magnitude from seismic moment", "seismology",
     "Moment magnitude Mw of an earthquake from its scalar seismic moment (Hanks and Kanamori 1979, as ObsPy: "
     "Mw = log10(M0 in dyne cm)/1.5 - 10.7).",
     [I("M0", "seismic_moment", "seismic_moment", "scalar seismic moment", ("seismic moment", "moment"), "N*m")],
     [O("Mw", "number", "moment magnitude"), O("M0_dyne_cm", "number", "seismic moment (dyne cm)")], moment_magnitude, _ref_mw,
     lambda M0: {"Mw": _r(math.log10(M0) / 1.5 - 10.7), "M0_dyne_cm": _r(M0 * 1e7)},
     lambda rng: {"M0": 10 ** rng.uniform(13, 22.5)},
     [{"M0": 1.1e21}, {"M0": 3.5e17}], ["moment magnitude"],
     conditions=["1e6 <= M0 <= 1e25 N m"], invalid=[{"M0": -5}],
     method="Mw = (2/3) log10(M0 [dyne cm]) - 10.7", latex=r"M_w = \tfrac{2}{3}\log_{10}M_0[\mathrm{dyn\,cm}] - 10.7")


def local_magnitude(A, r):
    _finite(A=A, r=r)
    _need(A > 0, "the amplitude must be positive")
    _need(1e4 <= r <= 6e5, "the hypocentral distance must be between 10 and 600 km")
    a_mm, r_km = A * 1e3, r / 1e3
    return {"ML": _r(math.log10(a_mm) + math.log10(r_km / 100.0) + 0.00301 * (r_km - 100.0) + 3.0)}


def _ref_ml(A, r):
    import warnings
    warnings.simplefilter("ignore")
    from obspy.signal.invsim import WOODANDERSON, estimate_magnitude
    # ObsPy reads a peak-to-peak trace amplitude of an instrument; on the Wood-Anderson instrument itself in metres
    # the zero-to-peak Wood-Anderson amplitude in mm is A*1000
    return {"ML": float(estimate_magnitude(WOODANDERSON, 2 * A, 1.0, r / 1e3))}


proc("local_magnitude", "Local (Richter) magnitude from Wood-Anderson amplitude", "seismology",
     "Local magnitude from the zero-to-peak Wood-Anderson amplitude and the hypocentral distance, with the "
     "attenuation of Bakun and Joyner (1984), as ObsPy.",
     [I("A", "length", "length", "zero-to-peak Wood-Anderson amplitude", ("amplitude", "wood anderson amplitude"), "mm"),
      I("r", "length", "length", "hypocentral distance", ("distance", "hypocentral distance"), "km")],
     [O("ML", "number", "local magnitude")], local_magnitude, _ref_ml,
     lambda A, r: {"ML": _r(math.log10(A * 1e3) + 1.11 * math.log10(r / 1e5) + 0.00189 * (r / 1e3 - 100.0) + 3.0)},
     lambda rng: {"A": 10 ** rng.uniform(-4, 0), "r": rng.uniform(2e4, 4e5)},
     [{"A": 10, "r": 100}, {"A": 0.5, "r": 50}], ["local magnitude", "richter magnitude"],
     conditions=["10 km <= r <= 600 km", "calibration of central California (Bakun and Joyner 1984)"],
     invalid=[{"A": 10, "r": 2}],
     method="ML = log10 A + log10(r/100) + 0.00301 (r - 100) + 3.0 (A in mm, r in km)",
     latex=r"M_L = \log_{10}A + \log_{10}\frac{r}{100} + 0.00301\,(r-100) + 3.0")


def b_value(mags, Mc, dM=0.0):
    _finite(Mc=Mc, dM=dM)
    _need(0 <= dM <= 0.5, "the magnitude bin width must be between 0 and 0.5")
    sel = [m for m in mags if m >= Mc - 1e-9]
    _need(len(sel) >= 10, "at least 10 magnitudes at or above the completeness magnitude are needed")
    mean = sum(sel) / len(sel)
    den = mean - (Mc - dM / 2)
    _need(den > 0, "the mean magnitude must exceed Mc - dM/2")
    b = math.log10(math.e) / den
    return {"b": _r(b), "a": _r(math.log10(len(sel)) + b * Mc), "n": float(len(sel)), "mean": _r(mean)}


def _ref_b(mags, Mc, dM=0.0):
    import numpy as np
    from scipy import stats
    x = np.array([m for m in mags if m >= Mc - 1e-9])
    _loc, scale = stats.expon.fit(x - (Mc - dM / 2), floc=0)           # maximum-likelihood exponential fit
    b = math.log10(math.e) / scale
    return {"b": b, "a": math.log10(len(x)) + b * Mc, "n": float(len(x))}


def _rand_b(rng):
    b, Mc = rng.uniform(0.7, 1.4), round(rng.uniform(1.0, 3.0), 1)
    dM = rng.choice([0.0, 0.1])
    out = []
    for _ in range(rng.randint(30, 150)):
        m = Mc - dM / 2 + rng.expovariate(b * math.log(10))
        out.append(round(round(m / dM) * dM, 10) if dM else round(m, 6))
    return {"mags": out, "Mc": Mc, "dM": dM}


proc("gutenberg_richter_b_value", "Gutenberg-Richter b-value (maximum likelihood)", "seismology",
     "The b-value of the Gutenberg-Richter law log10 N = a - b M by Aki's maximum-likelihood estimator, with Utsu's "
     "correction for magnitudes binned at width dM; also the a-value of the cumulative count at Mc.",
     [I("mags", "number", "number", "list of event magnitudes", ("magnitudes",), "", typ="vector", min_length=10),
      I("Mc", "number", "number", "magnitude of completeness", ("completeness magnitude", "mc"), ""),
      I("dM", "number", "number", "magnitude bin width (0 for unbinned magnitudes; default 0)", ("bin width",), "",
        required=False)],
     [O("b", "number", "b-value"), O("a", "number", "a-value (log10 of the number of events >= Mc, plus b Mc)"),
      O("n", "number", "number of events used"), O("mean", "number", "mean magnitude of the events used")],
     b_value, _ref_b,
     lambda mags, Mc, dM=0.0: (lambda sel: (lambda b: {"b": _r(b), "a": _r(math.log10(len(sel)) + b * Mc), "n": float(len(sel)),
                                                       "mean": _r(sum(sel) / len(sel))})(1 / (sum(sel) / len(sel) - Mc + dM / 2)))(
         [m for m in mags if m >= Mc - 1e-9]),
     _rand_b,
     [{"mags": [2.1, 2.4, 2.0, 3.1, 2.6, 2.2, 2.9, 2.3, 2.0, 4.0, 2.5, 2.1], "Mc": 2.0, "dM": 0.1},
      {"mags": [3.0, 3.4, 3.1, 3.9, 3.2, 3.0, 3.6, 3.3, 3.1, 4.4, 3.5], "Mc": 3.0, "dM": 0.1}],
     ["b value", "gutenberg richter b value", "gutenberg richter"],
     conditions=["at least 10 events with M >= Mc", "magnitudes complete above Mc"],
     method="b = log10(e)/(mean(M) - (Mc - dM/2)) (Aki 1965, Utsu 1966)",
     latex=r"b = \frac{\log_{10}e}{\overline{M}-\left(M_c-\Delta M/2\right)}")


def nmo_velocity(x, t):
    _need(len(x) == len(t) and len(x) >= 2, "give as many offsets as times (at least two)")
    X = [v * v for v in x]
    Y = [v * v for v in t]
    n = len(X)
    mx, my = sum(X) / n, sum(Y) / n
    sxx = sum((a - mx) ** 2 for a in X)
    _need(sxx > 0, "at least two different offsets are needed")
    slope = sum((a - mx) * (b - my) for a, b in zip(X, Y)) / sxx
    icpt = my - slope * mx
    _need(slope > 0 and icpt > 0, "the picks do not fit a reflection hyperbola (t^2 must grow with x^2 from t0^2 > 0)")
    V, t0 = 1 / math.sqrt(slope), math.sqrt(icpt)
    return {"V_nmo": _r(V), "t0": _r(t0), "depth": _r(V * t0 / 2)}


def _ref_nmo(x, t):
    import numpy as np
    s, c = np.polyfit(np.array(x) ** 2, np.array(t) ** 2, 1)            # least squares t^2 = t0^2 + x^2/V^2
    V, t0 = 1 / np.sqrt(s), np.sqrt(c)
    return {"V_nmo": float(V), "t0": float(t0), "depth": float(V * t0 / 2)}


def _rand_nmo(rng):
    V, t0, n = rng.uniform(1500, 4500), rng.uniform(0.4, 3.0), rng.randint(3, 12)
    xs = sorted(rng.uniform(0, 4000) for _ in range(n))
    return {"x": xs, "t": [math.sqrt(t0 * t0 + (v / V) ** 2) * (1 + rng.uniform(-1e-3, 1e-3)) for v in xs]}


proc("nmo_velocity", "Normal-moveout velocity from reflection picks", "seismology",
     "Stacking (NMO) velocity, zero-offset time and reflector depth from reflection travel times at several offsets, "
     "by a least-squares fit of t^2 = t0^2 + x^2/V^2.",
     [I("x", "number", "number", "source-receiver offsets (list, in metres)", ("offsets",), "", typ="vector", min_length=2),
      I("t", "number", "number", "two-way reflection times (list, in seconds)", ("times", "travel times"), "", typ="vector",
        min_length=2)],
     [O("V_nmo", "speed", "NMO velocity (m/s)"), O("t0", "time", "zero-offset two-way time (s)"),
      O("depth", "length", "reflector depth V t0/2 for a constant velocity (m)")], nmo_velocity, _ref_nmo,
     lambda x, t: (lambda s, c: {"V_nmo": _r(1 / math.sqrt(abs(s))), "t0": _r(abs(c)), "depth": _r(abs(c) / math.sqrt(abs(s)) / 2)})(
         *(lambda X, Y: (lambda mx, my: ((lambda sl: (sl, my - sl * mx))(sum((a - mx) * (b - my) for a, b in zip(X, Y)) / sum((a - mx) ** 2 for a in X))))(
             sum(X) / len(X), sum(Y) / len(Y)))([v * v for v in x], list(t))),
     _rand_nmo,
     [{"x": [0, 500, 1000, 1500, 2000], "t": [1.0, 1.0198, 1.0770, 1.1662, 1.2806]},
      {"x": [200, 800, 1600], "t": [0.62, 0.71, 0.93]}], ["nmo velocity", "normal moveout velocity", "stacking velocity"],
     conditions=["flat reflector, constant velocity above it", "offsets in m, times in s"],
     method="least squares of t^2 against x^2: slope 1/V^2, intercept t0^2",
     latex=r"t^2 = t_0^2 + \frac{x^2}{V_{\mathrm{nmo}}^2}")


# =============================================================================================== geomagnetism, tectonics
def dipole_field(lat):
    _lat_ok(lat, 89.999, "magnetic latitude")
    ph = math.radians(lat)
    return {"inclination": _r(math.degrees(math.atan(2 * math.tan(ph)))), "F_ratio": _r(math.sqrt(1 + 3 * math.sin(ph) ** 2))}


def _dipole_vector(lat):
    import numpy as np
    ph = math.radians(lat)
    m = np.array([0.0, 0.0, -1.0])                                    # the geocentric dipole points to geographic south
    r = np.array([math.cos(ph), 0.0, math.sin(ph)])
    B = 3 * (m @ r) * r - m                                            # dipole field (unit moment, unit radius)
    north = np.array([-math.sin(ph), 0.0, math.cos(ph)])
    return float(B @ north), float(-(B @ r)), float(np.linalg.norm(B))


def _ref_dipole(lat):
    H, Z, F = _dipole_vector(lat)
    return {"inclination": math.degrees(math.atan2(Z, H)), "F_ratio": F / _dipole_vector(0.0)[2]}


proc("dipole_inclination", "Inclination and strength of a geocentric dipole field", "geomagnetism",
     "Magnetic inclination (dip) and field strength relative to the equator at a magnetic latitude for a geocentric "
     "axial dipole.",
     [_LAT("lat", "magnetic latitude (degrees)", ("magnetic latitude", "latitude"))],
     [O("inclination", "number", "inclination (degrees, positive downward)"), O("F_ratio", "number", "field strength / equatorial strength")],
     dipole_field, _ref_dipole,
     lambda lat: {"inclination": _r(math.degrees(math.atan(math.tan(math.radians(lat)) / 2))), "F_ratio": _r(math.sqrt(1 + 3 * math.sin(math.radians(lat)) ** 2))},
     lambda rng: {"lat": rng.uniform(-89, 89)},
     [{"lat": 45}, {"lat": -20}], ["dipole inclination", "magnetic inclination", "geomagnetic inclination"],
     conditions=["geocentric axial dipole"], method="tan I = 2 tan(latitude), F/F_eq = sqrt(1 + 3 sin^2 latitude)",
     latex=r"\tan I = 2\tan\lambda,\quad \frac{F}{F_{eq}} = \sqrt{1+3\sin^2\lambda}")


def paleolatitude(I_deg):
    _finite(I_deg=I_deg)
    _need(-89.999 <= I_deg <= 89.999, "the inclination must be strictly between -90 and 90 degrees")
    lat = math.degrees(math.atan(math.tan(math.radians(I_deg)) / 2))
    return {"paleolatitude": _r(lat), "pole_distance": _r(90 - lat)}


def _ref_paleolat(I_deg):
    f = lambda la: (lambda H, Z, F: math.degrees(math.atan2(Z, H)))(*_dipole_vector(la)) - I_deg    # noqa: E731
    la = _brentq(f, -89.9999999, 89.9999999)
    return {"paleolatitude": la, "pole_distance": 90 - la}


proc("paleolatitude", "Paleolatitude from magnetic inclination", "geomagnetism",
     "The magnetic (paleo)latitude at which a rock acquired its magnetisation, from the inclination of its remanence, "
     "for a geocentric axial dipole; and the angular distance to the paleopole.",
     [I("I_deg", "number", "degrees", "inclination of the remanent magnetisation (degrees)", ("inclination",), "°")],
     [O("paleolatitude", "number", "paleolatitude (degrees)"), O("pole_distance", "number", "distance to the pole (degrees)")],
     paleolatitude, _ref_paleolat,
     lambda I_deg: (lambda la: {"paleolatitude": _r(la), "pole_distance": _r(90 - la)})(math.degrees(math.atan(2 * math.tan(math.radians(I_deg))))),
     lambda rng: {"I_deg": rng.uniform(-88, 88)},
     [{"I_deg": 60}, {"I_deg": -30}], ["paleolatitude", "palaeolatitude"],
     conditions=["geocentric axial dipole", "|I| < 90"], invalid=[{"I_deg": 95}],
     method="latitude = atan(tan I/2)", latex=r"\lambda = \arctan\!\left(\tfrac12\tan I\right)")


def plate_velocity(pole_lat, pole_lon, omega, lat, lon):
    _coord_ok(pole_lat=pole_lat, pole_lon=pole_lon, lat=lat, lon=lon)
    _finite(omega=omega)
    _need(omega != 0 and abs(omega) <= 10, "the rotation rate must be non-zero and at most 10 degrees per Myr")
    d = epicentral_distance(lat, lon, pole_lat, pole_lon)["delta"]
    _need(0.5 <= d <= 179.5, "the site must not lie within 0.5 degrees of the pole or its antipode")
    p1, p2, dl = math.radians(lat), math.radians(pole_lat), math.radians(pole_lon - lon)
    az_pole = math.degrees(math.atan2(math.sin(dl) * math.cos(p2), math.cos(p1) * math.sin(p2) - math.sin(p1) * math.cos(p2) * math.cos(dl)))
    w = math.radians(abs(omega)) / 1e6                                  # rad/yr
    speed = w * R_EARTH_KM * 1e6 * math.sin(math.radians(d))            # mm/yr
    az = (az_pole + (90.0 if omega > 0 else -90.0)) % 360.0
    return {"speed": _r(speed), "azimuth": _r(az)}


def _ref_plate(pole_lat, pole_lon, omega, lat, lon):
    import numpy as np
    def unit(la, lo):
        la, lo = math.radians(la), math.radians(lo)
        return np.array([math.cos(la) * math.cos(lo), math.cos(la) * math.sin(lo), math.sin(la)])
    w = math.radians(omega) / 1e6 * unit(pole_lat, pole_lon)          # angular velocity vector, rad/yr
    r = R_EARTH_KM * 1e6 * unit(lat, lon)                               # position in mm
    v = np.cross(w, r)                                                  # mm/yr
    la, lo = math.radians(lat), math.radians(lon)
    east = np.array([-math.sin(lo), math.cos(lo), 0.0])
    north = np.array([-math.sin(la) * math.cos(lo), -math.sin(la) * math.sin(lo), math.cos(la)])
    return {"speed": float(np.linalg.norm(v)), "azimuth": math.degrees(math.atan2(v @ east, v @ north)) % 360.0}


def _rand_plate(rng):
    while True:
        x = {"pole_lat": rng.uniform(-85, 85), "pole_lon": rng.uniform(-180, 180), "omega": rng.choice([-1, 1]) * rng.uniform(0.1, 2.0),
             "lat": rng.uniform(-80, 80), "lon": rng.uniform(-180, 180)}
        d = epicentral_distance(x["lat"], x["lon"], x["pole_lat"], x["pole_lon"])["delta"]
        if 5 <= d <= 175:
            az = plate_velocity(**x)["azimuth"]
            if 1e-6 < az < 360 - 1e-6:
                return x


proc("plate_velocity", "Plate velocity from an Euler pole", "plate tectonics",
     "Speed and direction of a point on a rigid plate rotating about an Euler pole (sphere of radius 6371 km).",
     [_LATN("pole_lat", "latitude of the Euler pole", ("pole latitude",)), _LATN("pole_lon", "longitude of the Euler pole", ("pole longitude",)),
      I("omega", "number", "deg_per_myr", "rotation rate about the pole, counter-clockwise positive (degrees per Myr)",
        ("rotation rate", "angular velocity"), "°/Myr"),
      _LATN("lat", "latitude of the site", ("site latitude",)), _LATN("lon", "longitude of the site", ("site longitude",))],
     [O("speed", "number", "plate speed (mm/yr)"), O("azimuth", "number", "direction of motion (degrees from north)")],
     plate_velocity, _ref_plate,
     lambda pole_lat, pole_lon, omega, lat, lon: (lambda d: {"speed": _r(math.radians(abs(omega)) / 1e6 * R_EARTH_KM * 1e6 * math.cos(math.radians(d))),
                                                             "azimuth": 0.0})(epicentral_distance(lat, lon, pole_lat, pole_lon)["delta"]),
     _rand_plate,
     [{"pole_lat": 61.5, "pole_lon": -73.7, "omega": 0.75, "lat": 35, "lon": -120},
      {"pole_lat": 50, "pole_lon": -100, "omega": -0.3, "lat": 10, "lon": 20}], ["plate velocity", "euler pole"],
     conditions=["rigid plate on a sphere of radius 6371 km", "site 0.5 to 179.5 degrees from the pole"],
     method="v = omega R sin(Delta), direction perpendicular to the great circle to the pole",
     latex=r"v = \omega R \sin\Delta")


# =============================================================================================== gravity
def bouguer_slab(drho, h):
    _finite(drho=drho, h=h)
    _need(drho != 0 and abs(drho) <= 5000, "the density contrast must be non-zero and at most 5000 kg/m^3")
    _need(0 < h <= 50000, "the slab thickness must be between 0 and 50 km")
    g = 2 * math.pi * G_NEWTON * drho * h
    return {"dg_mGal": _r(g * 1e5), "dg": _r(g)}


def _ref_bouguer(drho, h):
    from scipy.integrate import quad
    # vertical attraction of a horizontal slab from depth h to 2h, summed over rings of radius r to infinity; the
    # depth integral of each ring is done exactly, the radial sum numerically
    ring = lambda r: 2 * math.pi * r * G_NEWTON * drho * (1 / math.hypot(r, h) - 1 / math.hypot(r, 2 * h))  # noqa: E731
    # r = h tan(u) maps [0, infinity) onto [0, pi/2)
    v, _ = quad(lambda u: ring(h * math.tan(u)) * h / math.cos(u) ** 2 if u < math.pi / 2 else 0.0, 0, math.pi / 2,
                epsabs=0, epsrel=1e-13, limit=500)
    return {"dg_mGal": v * 1e5, "dg": v}


proc("bouguer_slab", "Gravity anomaly of an infinite horizontal slab (Bouguer)", "geophysics",
     "Vertical gravity anomaly of an infinite horizontal slab of thickness h and density contrast drho.",
     [I("drho", "density", "density", "density contrast of the slab", ("density contrast",), "kg/m^3"),
      I("h", "length", "length", "slab thickness", ("thickness",), "m")],
     [O("dg_mGal", "number", "gravity anomaly (mGal)"), O("dg", "acceleration", "gravity anomaly (m/s^2)")],
     bouguer_slab, _ref_bouguer,
     lambda drho, h: {"dg_mGal": _r(math.pi * G_NEWTON * drho * h * 1e5), "dg": _r(math.pi * G_NEWTON * drho * h)},
     lambda rng: {"drho": rng.choice([-1, 1]) * rng.uniform(50, 1000), "h": rng.uniform(10, 5000)},
     [{"drho": 2670, "h": 100}, {"drho": -400, "h": 2000}], ["bouguer slab", "bouguer anomaly", "infinite slab"],
     conditions=["laterally infinite slab", "G = 6.6743e-11 N m^2/kg^2"], method="dg = 2 pi G drho h",
     latex=r"\Delta g = 2\pi G\,\Delta\rho\,h")


def buried_sphere(drho, R, z):
    _finite(drho=drho, R=R, z=z)
    _need(drho != 0 and abs(drho) <= 10000, "the density contrast must be non-zero and at most 10000 kg/m^3")
    _need(0 < R < z <= 1e6, "the sphere must lie wholly below the surface (0 < R < depth of its centre)")
    g = 4 / 3 * math.pi * G_NEWTON * drho * R ** 3 / (z * z)
    return {"dg_mGal": _r(g * 1e5), "dg": _r(g)}


def _ref_sphere(drho, R, z):
    from scipy.integrate import quad
    # the sphere cut into horizontal discs at depth s; each disc's on-axis attraction 2 pi G drho (1 - s/sqrt(s^2 + a^2))
    disc = lambda s: 2 * math.pi * G_NEWTON * drho * (1 - s / math.sqrt(s * s + max(R * R - (s - z) ** 2, 0.0)))  # noqa: E731
    v, _ = quad(disc, z - R, z + R, epsabs=0, epsrel=1e-13, limit=500)
    return {"dg_mGal": v * 1e5, "dg": v}


proc("buried_sphere_anomaly", "Gravity anomaly above a buried sphere", "geophysics",
     "Peak vertical gravity anomaly directly above a buried sphere of radius R, depth z to its centre and density "
     "contrast drho.",
     [I("drho", "density", "density", "density contrast of the sphere", ("density contrast",), "kg/m^3"),
      I("R", "length", "length", "radius of the sphere", ("radius",), "m"),
      I("z", "length", "length", "depth to the centre of the sphere", ("depth",), "m")],
     [O("dg_mGal", "number", "peak gravity anomaly (mGal)"), O("dg", "acceleration", "peak gravity anomaly (m/s^2)")],
     buried_sphere, _ref_sphere,
     lambda drho, R, z: {"dg_mGal": _r(4 / 3 * math.pi * G_NEWTON * drho * R ** 3 / z * 1e5), "dg": _r(4 / 3 * math.pi * G_NEWTON * drho * R ** 3 / z)},
     lambda rng: (lambda R: {"drho": rng.choice([-1, 1]) * rng.uniform(100, 2000), "R": R, "z": R * rng.uniform(1.2, 10)})(rng.uniform(10, 2000)),
     [{"drho": 500, "R": 100, "z": 200}, {"drho": -300, "R": 1000, "z": 3000}], ["buried sphere", "gravity anomaly of a sphere"],
     conditions=["0 < R < z", "observation point directly above the centre"], invalid=[{"drho": 500, "R": 300, "z": 200}],
     method="dg = (4/3) pi G drho R^3/z^2", latex=r"\Delta g = \frac{4\pi G\,\Delta\rho\,R^3}{3z^2}")


# =============================================================================================== heat flow and the lithosphere
def geotherm(T0, q0, k, A, z):
    _finite(T0=T0, q0=q0, k=k, A=A, z=z)
    _need(k > 0, "the thermal conductivity must be positive")
    _need(q0 > 0 and A >= 0, "surface heat flow must be positive and heat production non-negative")
    _need(0 <= z <= 2e5, "depth must be between 0 and 200 km")
    return {"T": _r(T0 + q0 * z / k - A * z * z / (2 * k)), "q": _r(q0 - A * z)}


def _ref_geotherm(T0, q0, k, A, z):
    # steady conduction with internal heating: dT/dz = q/k, dq/dz = -A, integrated from the surface
    T, q = _ivp(lambda zz, y: [y[1] / k, -A], z, [T0, q0]) if z > 0 else (T0, q0)
    return {"T": T, "q": q}


proc("geotherm_radiogenic", "Steady geotherm with radiogenic heat production", "geothermics",
     "Temperature and heat flow at depth in a conducting layer with uniform radiogenic heat production, from the "
     "surface temperature and surface heat flow.",
     [_TEMP("T0", "surface temperature", ("surface temperature",)),
      I("q0", "heat_flux", "heat_flux", "surface heat flow", ("surface heat flow", "heat flow"), "mW/m^2"),
      I("k", "thermal_conductivity", "thermal_conductivity", "thermal conductivity", ("thermal conductivity", "conductivity"), "W/(m*K)"),
      I("A", "volumetric_heat_production", "volumetric_heat_production", "radiogenic heat production per unit volume",
        ("heat production",), "µW/m^3"),
      I("z", "length", "length", "depth", ("depth",), "km")],
     [O("T", "temperature", "temperature at depth (K)"), O("q", "heat_flux", "heat flow at depth (W/m^2)")],
     geotherm, _ref_geotherm,
     lambda T0, q0, k, A, z: {"T": _r(T0 + q0 * z / k + A * z * z / (2 * k)), "q": _r(q0 + A * z)},
     lambda rng: {"T0": rng.uniform(273, 300), "q0": rng.uniform(0.04, 0.1), "k": rng.uniform(2, 3.5),
                  "A": rng.uniform(0.5e-6, 3e-6), "z": rng.uniform(1000, 30000)},
     [{"T0": 10, "q0": 65, "k": 2.5, "A": 1, "z": 10}, {"T0": 15, "q0": 80, "k": 3, "A": 2, "z": 20}],
     ["geotherm", "temperature at depth", "radiogenic heat production"],
     conditions=["steady one-dimensional conduction", "uniform heat production and conductivity"],
     method="T(z) = T0 + q0 z/k - A z^2/(2k), q(z) = q0 - A z",
     latex=r"T(z) = T_0 + \frac{q_0}{k}z - \frac{A}{2k}z^2")


def seafloor_depth(t, dr=2500.0, Tm=1573.15, Tw=273.15, alpha=3e-5, kappa=1e-6, rho_m=3300.0, rho_w=1000.0):
    _finite(t=t, dr=dr, Tm=Tm, Tw=Tw, alpha=alpha, kappa=kappa, rho_m=rho_m, rho_w=rho_w)
    _need(0 < t <= 200, "the seafloor age must be between 0 and 200 Myr")
    _need(Tm > Tw and alpha > 0 and kappa > 0 and rho_m > rho_w > 0, "need Tm > Tw, alpha > 0, kappa > 0, rho_m > rho_w")
    ts = t * 1e6 * YEAR
    w = 2 * rho_m * alpha * (Tm - Tw) / (rho_m - rho_w) * math.sqrt(kappa * ts / math.pi)
    return {"depth": _r(dr + w), "subsidence": _r(w)}


def _ref_seafloor(t, dr=2500.0, Tm=1573.15, Tw=273.15, alpha=3e-5, kappa=1e-6, rho_m=3300.0, rho_w=1000.0):
    from scipy.integrate import quad
    from scipy.special import erfc
    ts = t * 1e6 * YEAR
    L = 2 * math.sqrt(kappa * ts)
    # isostatic balance of the thermal contraction of the half-space-cooling lithosphere, integrated over depth
    v = sum(quad(lambda z: (Tm - Tw) * erfc(z / L), a * L, b * L, epsabs=0, epsrel=1e-13, limit=200)[0]
            for a, b in ((0, 0.5), (0.5, 1), (1, 2), (2, 4), (4, 8), (8, 28)))      # erfc(28) < 1e-300
    w = rho_m * alpha * v / (rho_m - rho_w)
    return {"depth": dr + w, "subsidence": w}


proc("seafloor_depth_age", "Depth of the ocean floor with age (half-space cooling)", "plate tectonics",
     "Seafloor depth as oceanic lithosphere cools and contracts with age (half-space cooling, isostatic compensation "
     "under water). Defaults: ridge depth 2500 m, mantle 1300 °C, water 0 °C, alpha 3e-5 1/K, kappa 1e-6 m^2/s, "
     "rho_m 3300 and rho_w 1000 kg/m^3.",
     [I("t", "number", "myr", "age of the seafloor (millions of years)", ("age",), "Myr"),
      I("dr", "length", "length", "depth at the ridge (default 2500 m)", ("ridge depth",), "m", required=False),
      I("Tm", "temperature", "temperature", "mantle temperature (default 1300 °C)", ("mantle temperature",), "°C", required=False),
      I("Tw", "temperature", "temperature", "seawater temperature (default 0 °C)", ("water temperature",), "°C", required=False),
      I("alpha", "expansion_coefficient", "expansion_coefficient", "volumetric thermal expansion coefficient (default 3e-5 1/K)",
        ("thermal expansion",), "1/K", required=False),
      I("kappa", "thermal_diffusivity", "thermal_diffusivity", "thermal diffusivity (default 1e-6 m^2/s)", ("thermal diffusivity",),
        "m^2/s", required=False),
      I("rho_m", "density", "density", "mantle density (default 3300 kg/m^3)", ("mantle density",), "kg/m^3", required=False),
      I("rho_w", "density", "density", "seawater density (default 1000 kg/m^3)", ("water density",), "kg/m^3", required=False)],
     [O("depth", "length", "seafloor depth (m)"), O("subsidence", "length", "subsidence below the ridge (m)")],
     seafloor_depth, _ref_seafloor,
     lambda t, dr=2500.0, Tm=1573.15, Tw=273.15, alpha=3e-5, kappa=1e-6, rho_m=3300.0, rho_w=1000.0: (lambda w: {"depth": _r(dr + w), "subsidence": _r(w)})(
         rho_m * alpha * (Tm - Tw) / (rho_m - rho_w) * math.sqrt(kappa * t * 1e6 * YEAR / math.pi)),
     lambda rng: {"t": rng.uniform(1, 180), "dr": rng.uniform(2000, 3000), "Tm": rng.uniform(1500, 1700), "Tw": rng.uniform(273, 278),
                  "alpha": rng.uniform(2e-5, 4e-5), "kappa": rng.uniform(5e-7, 1.5e-6), "rho_m": rng.uniform(3200, 3400),
                  "rho_w": rng.uniform(1000, 1030)},
     [{"t": 50}, {"t": 100, "dr": 2600, "Tm": 1350, "Tw": 2, "alpha": 3.2e-5, "kappa": 8e-7, "rho_m": 3300, "rho_w": 1025}],
     ["seafloor depth", "ocean floor depth", "depth of the seafloor", "half space cooling"],
     conditions=["0 < age <= 200 Myr (half-space cooling; old seafloor flattens in reality)"],
     method="d = d_r + 2 rho_m alpha (Tm - Tw) sqrt(kappa t/pi)/(rho_m - rho_w)",
     latex=r"d = d_r + \frac{2\rho_m\alpha(T_m-T_w)}{\rho_m-\rho_w}\sqrt{\frac{\kappa t}{\pi}}")


# =============================================================================================== geochronology
def _decay_N(tau):
    """N(tau)/N(0) of dN/dtau = -N (tau = decay constant times time), integrated numerically."""
    return _ivp(lambda s, y: [-y[0]], tau, [1.0])[0] if tau > 0 else 1.0


_YRS = lambda sym, meaning, names, show="yr", **kw: I(sym, "number", "years", meaning, names, show, **kw)   # noqa: E731


def decay_remaining(half_life, t):
    _finite(half_life=half_life, t=t)
    _need(half_life > 0, "the half-life must be positive")
    _need(0 <= t <= 60 * half_life, "time must be between 0 and 60 half-lives")
    lam = math.log(2) / half_life
    frac = 2.0 ** (-t / half_life)
    return {"fraction": _r(frac), "decayed": _r(-math.expm1(-lam * t)), "lambda": _r(lam)}


proc("radioactive_decay_remaining", "Fraction of a radioactive parent remaining", "geochronology",
     "Fraction of the parent nuclide left (and decayed) after a time t, and the decay constant ln 2/half-life.",
     [_YRS("half_life", "half-life (years)", ("half-life", "half life")), _YRS("t", "elapsed time (years)", ("time", "elapsed time"))],
     [O("fraction", "number", "fraction of the parent remaining"), O("decayed", "number", "fraction decayed"),
      O("lambda", "number", "decay constant (1/yr)")], decay_remaining,
     lambda half_life, t: (lambda N: {"fraction": N, "decayed": 1 - N, "lambda": math.log(2) / half_life})(_decay_N(math.log(2) / half_life * t)),
     lambda half_life, t: {"fraction": _r(math.exp(-t / half_life)), "decayed": _r(1 - math.exp(-t / half_life)), "lambda": _r(1 / half_life)},
     lambda rng: (lambda h: {"half_life": h, "t": h * rng.uniform(0.05, 8)})(10 ** rng.uniform(0, 10)),
     [{"half_life": 5730, "t": 10000}, {"half_life": 1.25e9, "t": 4.5e9}], ["fraction remaining", "radioactive decay",
                                                                       "remaining fraction"],
     conditions=["half-life > 0", "t <= 60 half-lives"], method="N/N0 = 2^(-t/t_half), lambda = ln 2/t_half",
     latex=r"\frac{N}{N_0} = 2^{-t/t_{1/2}} = e^{-\lambda t},\quad \lambda = \frac{\ln 2}{t_{1/2}}")


def radiometric_age(ratio, half_life):
    _finite(ratio=ratio, half_life=half_life)
    _need(half_life > 0, "the half-life must be positive")
    _need(0 < ratio <= 1e6, "the daughter/parent ratio must be positive")
    lam = math.log(2) / half_life
    return {"age": _r(math.log1p(ratio) / lam), "lambda": _r(lam)}


def _ref_age(ratio, half_life):
    lam = math.log(2) / half_life
    # radiogenic daughter D = 1 - N grows as the parent N decays; shoot for the time at which D/N equals the ratio
    tau = _shoot(lambda s: (lambda N: (1 - N) / N)(_decay_N(s)) - ratio, 0.0, 60.0, math.log1p(ratio))
    return {"age": tau / lam, "lambda": lam}


proc("radiometric_age", "Radiometric age from a daughter/parent ratio", "geochronology",
     "Age of a closed system from the ratio of radiogenic daughter to remaining parent atoms and the half-life "
     "(no initial daughter).",
     [I("ratio", "number", "number", "radiogenic daughter / parent atom ratio", ("daughter to parent ratio", "d/p ratio",
                                                                               "daughter/parent ratio"), ""),
      _YRS("half_life", "half-life of the parent (years)", ("half-life", "half life"))],
     [O("age", "number", "age (years)"), O("lambda", "number", "decay constant (1/yr)")], radiometric_age, _ref_age,
     lambda ratio, half_life: {"age": _r(math.log1p(1 / ratio) * half_life / math.log(2)), "lambda": _r(math.log(2) / half_life)},
     lambda rng: {"ratio": 10 ** rng.uniform(-2, 1.3), "half_life": 10 ** rng.uniform(3, 10.5)},
     [{"ratio": 0.5, "half_life": 1.25e9}, {"ratio": 3, "half_life": 4.468e9}], ["radiometric age", "radiometric dating",
                                                                             "absolute age"],
     conditions=["closed system", "no daughter at the start"], method="t = ln(1 + D/P)/lambda",
     latex=r"t = \frac{1}{\lambda}\ln\!\left(1+\frac{D^*}{P}\right)")


def k_ar_age(ratio, lam=5.543e-10, lam_e=0.581e-10):
    _finite(ratio=ratio, lam=lam, lam_e=lam_e)
    _need(0 < ratio <= 100, "the 40Ar*/40K ratio must be positive")
    _need(0 < lam_e < lam <= 1e-6, "need 0 < lambda_e < lambda")
    return {"age": _r(math.log1p(lam / lam_e * ratio) / lam)}


def _ref_k_ar(ratio, lam=5.543e-10, lam_e=0.581e-10):
    fe = lam_e / lam

    def r_at(s):                                                       # K and Ar* after unit-rate time s
        K, Ar = _ivp(lambda tt, y: [-y[0], fe * y[0]], s, [1.0, 0.0]) if s > 0 else (1.0, 0.0)
        return Ar / K
    tau = _shoot(lambda s: r_at(s) - ratio, 0.0, 40.0, math.log1p(ratio / fe))
    return {"age": tau / lam}


proc("potassium_argon_age", "Potassium-argon age (branched decay)", "geochronology",
     "K-Ar age from the radiogenic 40Ar*/40K ratio, allowing for the branching of 40K decay to 40Ar (electron capture) "
     "and 40Ca. Defaults: lambda = 5.543e-10 /yr, lambda_e = 0.581e-10 /yr (Steiger and Jäger 1977).",
     [I("ratio", "number", "number", "radiogenic 40Ar*/40K atom ratio", ("ar/k ratio", "argon potassium ratio", "40ar/40k"), ""),
      I("lam", "number", "per_year", "total decay constant of 40K (default 5.543e-10 /yr)", ("decay constant",), "1/yr", required=False),
      I("lam_e", "number", "per_year", "partial decay constant to 40Ar (default 0.581e-10 /yr)", ("electron capture constant",), "1/yr",
        required=False)],
     [O("age", "number", "K-Ar age (years)")], k_ar_age, _ref_k_ar,
     lambda ratio, lam=5.543e-10, lam_e=0.581e-10: {"age": _r(math.log1p(ratio) / lam)},
     lambda rng: {"ratio": 10 ** rng.uniform(-4, 0), "lam": 5.543e-10, "lam_e": 0.581e-10},
     [{"ratio": 0.0055}, {"ratio": 0.12, "lam": 5.543e-10, "lam_e": 0.581e-10}], ["potassium argon age", "k ar age", "k-ar age"],
     conditions=["closed system, no initial argon"], method="t = (1/lambda) ln(1 + (lambda/lambda_e) Ar*/K)",
     latex=r"t = \frac{1}{\lambda}\ln\!\left(1+\frac{\lambda}{\lambda_e}\frac{^{40}\mathrm{Ar}^*}{^{40}\mathrm{K}}\right)")


def isochron_age(x, y, lam=1.42e-11):
    _finite(lam=lam)
    _need(len(x) == len(y) and len(x) >= 3, "give at least three samples, as many x as y values")
    _need(0 < lam <= 1e-3, "the decay constant must be positive")
    n = len(x)
    mx, my = sum(x) / n, sum(y) / n
    sxx = sum((a - mx) ** 2 for a in x)
    _need(sxx > 0, "the parent/daughter ratios must differ between samples")
    slope = sum((a - mx) * (b - my) for a, b in zip(x, y)) / sxx
    _need(slope > 0, "the isochron slope must be positive")
    return {"age": _r(math.log1p(slope) / lam), "slope": _r(slope), "initial_ratio": _r(my - slope * mx)}


def _ref_isochron(x, y, lam=1.42e-11):
    from scipy import stats
    fit = stats.linregress(x, y)
    tau = _shoot(lambda s: (lambda N: (1 - N) / N)(_decay_N(s)) - fit.slope, 0.0, 30.0, math.log1p(fit.slope))
    return {"age": tau / lam, "slope": fit.slope, "initial_ratio": fit.intercept}


def _rand_iso(rng):
    lam, age = 1.42e-11, rng.uniform(1e8, 4.5e9)
    m, y0, n = math.expm1(lam * age), rng.uniform(0.700, 0.710), rng.randint(4, 10)
    xs = [rng.uniform(0.05, 6) for _ in range(n)]
    return {"x": xs, "y": [y0 + m * v + rng.uniform(-2e-5, 2e-5) for v in xs], "lam": lam}


proc("isochron_age", "Isochron age (e.g. Rb-Sr) from sample ratios", "geochronology",
     "Age and initial ratio from an isochron: a least-squares line through (parent/stable, daughter/stable) ratios of "
     "cogenetic samples (ordinary least squares, not a York fit). Default decay constant 1.42e-11 /yr (87Rb).",
     [I("x", "number", "number", "parent/stable isotope ratios (list, e.g. 87Rb/86Sr)", ("parent ratios", "x ratios"), "",
        typ="vector", min_length=3),
      I("y", "number", "number", "daughter/stable isotope ratios (list, e.g. 87Sr/86Sr)", ("daughter ratios", "y ratios"), "",
        typ="vector", min_length=3),
      I("lam", "number", "per_year", "decay constant of the parent (default 1.42e-11 /yr)", ("decay constant",), "1/yr", required=False)],
     [O("age", "number", "isochron age (years)"), O("slope", "number", "isochron slope e^(lambda t) - 1"),
      O("initial_ratio", "number", "initial daughter/stable ratio (intercept)")], isochron_age, _ref_isochron,
     lambda x, y, lam=1.42e-11: (lambda r: dict(r, age=_r(r["slope"] / lam)))(isochron_age(x, y, lam)),
     _rand_iso,
     [{"x": [0.5, 1.2, 2.8, 4.1], "y": [0.7121, 0.7221, 0.7449, 0.7634], "lam": 1.42e-11},
      {"x": [0.2, 0.9, 1.7, 3.3, 5.0], "y": [0.70452, 0.71354, 0.72384, 0.74444, 0.76632]}],
     ["isochron age", "isochron", "rb sr age", "rubidium strontium age"],
     conditions=["cogenetic samples, closed systems, same initial ratio"], method="t = ln(1 + slope)/lambda",
     latex=r"t = \frac{\ln(1+m)}{\lambda},\quad \frac{D}{S} = \left(\frac{D}{S}\right)_0 + m\,\frac{P}{S}")


def radiocarbon_age(F):
    _finite(F=F)
    _need(1e-5 <= F <= 1.0, "the fraction modern must be between 1e-5 and 1")
    return {"age": _r(-8033.0 * math.log(F))}


def _ref_radiocarbon(F):
    tau = _shoot(lambda s: _decay_N(s) - F, 0.0, 12.0, -math.log(F))
    return {"age": tau * 8033.0}


proc("radiocarbon_age", "Conventional radiocarbon age", "geochronology",
     "Conventional radiocarbon age (years BP) from the fraction modern carbon, with the Libby mean life of 8033 years "
     "(Stuiver and Polach 1977); not calibrated to calendar years.",
     [I("F", "number", "fraction", "fraction modern (14C activity relative to the modern standard; or % in a question)",
        ("fraction modern", "fraction of modern carbon"), "")],
     [O("age", "number", "conventional radiocarbon age (years BP)")], radiocarbon_age, _ref_radiocarbon,
     lambda F: {"age": _r(-5730.0 / math.log(2) * math.log(F))},
     lambda rng: {"F": 10 ** rng.uniform(-4, -0.001)},
     [{"F": 0.5}, {"F": 0.25}], ["radiocarbon age", "carbon 14 age", "radiocarbon dating"],
     conditions=["1e-5 <= F <= 1 (no bomb carbon)"], invalid=[{"F": 1.3}],
     method="t = -8033 ln F", latex=r"t = -8033\,\ln F")


# =============================================================================================== sediments
def stokes_settling(d, rho_s, rho_f, mu):
    _finite(d=d, rho_s=rho_s, rho_f=rho_f, mu=mu)
    _need(0 < d <= 1e-3, "grain diameter must be between 0 and 1 mm")
    _need(rho_s > rho_f > 0 and mu > 0, "need rho_s > rho_f > 0 and mu > 0")
    w = (rho_s - rho_f) * G0 * d * d / (18 * mu)
    re = rho_f * w * d / mu
    _need(re <= 1.0, "the grain Reynolds number exceeds 1: Stokes' law does not apply")
    return {"w": _r(w), "Re": _r(re)}


def _ref_stokes(d, rho_s, rho_f, mu):
    vol = math.pi * d ** 3 / 6
    m = rho_s * vol
    tau = m / (3 * math.pi * mu * d)                                   # drag time scale
    # the grain released from rest: weight minus buoyancy minus Stokes drag 3 pi mu d v, integrated to terminal speed
    v = _ivp(lambda s, y: [tau * ((rho_s - rho_f) * vol * G0 - 3 * math.pi * mu * d * y[0]) / m], 60.0, [0.0])[0]
    return {"w": v, "Re": rho_f * v * d / mu}


proc("stokes_settling_velocity", "Settling velocity of a small grain (Stokes)", "sedimentology",
     "Terminal settling velocity of a small sphere in a viscous fluid (Stokes' law), with the grain Reynolds number "
     "(refused when it exceeds 1).",
     [I("d", "length", "length", "grain diameter", ("diameter", "grain diameter"), "mm"),
      I("rho_s", "density", "density", "grain density", ("grain density", "particle density"), "kg/m^3"),
      I("rho_f", "density", "density", "fluid density", ("fluid density", "water density"), "kg/m^3"),
      I("mu", "dynamic_viscosity", "dynamic_viscosity", "dynamic viscosity of the fluid", ("viscosity", "dynamic viscosity"), "Pa*s")],
     [O("w", "speed", "settling velocity (m/s)"), O("Re", "number", "grain Reynolds number")], stokes_settling, _ref_stokes,
     lambda d, rho_s, rho_f, mu: (lambda w: {"w": _r(w), "Re": _r(rho_f * w * d / mu)})((rho_s - rho_f) * G0 * d * d / (9 * mu)),
     lambda rng: {"d": rng.uniform(1e-6, 7e-5), "rho_s": rng.uniform(2000, 3200), "rho_f": rng.uniform(998, 1030), "mu": rng.uniform(8e-4, 1.5e-3)},
     [{"d": 0.05, "rho_s": 2650, "rho_f": 1000, "mu": 0.001}, {"d": 0.01, "rho_s": 2650, "rho_f": 1025, "mu": 0.0012}],
     ["stokes settling velocity", "settling velocity", "stokes law"],
     conditions=["grain Reynolds number <= 1", "a smooth sphere, no neighbouring grains"], invalid=[{"d": 0.5, "rho_s": 2650, "rho_f": 1000, "mu": 0.001}],
     method="w = (rho_s - rho_f) g d^2/(18 mu)", latex=r"w = \frac{(\rho_s-\rho_f)\,g\,d^2}{18\,\mu}")


# =============================================================================================== definitional additions
# (owner, 2026-10-23: a capability that passes every gate ships; a weaker reference is documented). CO2 radiative
# forcing, S-P epicentral distance, Airy and Pratt isostasy, gradient wind, finite-depth wave dispersion, Darcy flow,
# stream power, Rossby number, solar elevation angle, half-space heat flow.
_UNITS.update({
    "area": {"m^2": (1, 0), "m2": (1, 0), "km^2": (1e6, 0), "km2": (1e6, 0), "cm^2": (1e-4, 0), "cm2": (1e-4, 0),
             "ha": (1e4, 0), "hectares": (1e4, 0)},
    "flow_rate": {"m^3/s": (1, 0), "m3/s": (1, 0), "L/s": (1e-3, 0), "l/s": (1e-3, 0), "m^3/day": (1 / 86400, 0),
                  "m3/day": (1 / 86400, 0), "m^3/d": (1 / 86400, 0)},
    "hydraulic_conductivity": {"m/s": (1, 0), "cm/s": (1e-2, 0), "m/day": (1 / 86400, 0), "m/d": (1 / 86400, 0)},
    "temperature_difference": {"K": (1, 0), "kelvin": (1, 0), "°C": (1, 0), "degC": (1, 0), "C": (1, 0)},
    "ppm": {"ppm": (1, 0), "ppmv": (1, 0)},
})


def _guard(fn):
    """A wrong variant that raises cannot be compared: return a failed marker instead."""
    def g(**kw):
        try:
            return fn(**kw)
        except Exception:
            return {"_failed": True}
    return g


# Outputs of the documented examples of these additions, computed by their references (SciPy, MetPy, NumPy) on the
# build machine and rounded to 12 significant digits.
EXAMPLE_OUTPUTS.update({
    "co2_radiative_forcing": [{"dF": 3.708337416}, {"dF": 2.16923832838, "dT_eq": 1.7353906627}],
    "sp_epicentral_distance": [{"distance_km": 84.0, "tp": 14.0, "ts": 24.0}, {"distance_km": 462.857142857, "tp": 57.8571428571, "ts": 102.857142857}],
    "airy_isostasy": [{"root": 16800.0}, {"root": 22500.0, "crust_thickness": 62500.0}],
    "pratt_isostasy": [{"rho": 2718.44660194, "drho": 81.5533980583}, {"rho": 2864.19753086, "drho": 35.8024691358}],
    "gradient_wind": [{"V": 15.400364026, "ratio": 0.770018201301, "f": 0.000103126079314}, {"V": 11.2209243737, "ratio": 1.12209243737, "f": 0.000103126079314}],
    "wave_dispersion": [{"L": 121.209844039, "c": 12.1209844039, "cg": 9.27161212056, "kh": 1.03674505268}, {"L": 56.1879655944, "c": 9.36466093241, "cg": 4.68233050676, "kh": 11.182439586}],
    "darcy_flow": [{"Q": 0.0001, "q": 2e-06, "gradient": 0.02}, {"Q": 2e-05, "q": 1e-06, "gradient": 0.02, "v": 3.33333333333e-06}],
    "stream_power": [{"Omega": 1961.33}, {"Omega": 2451.6625, "omega": 122.583125}],
    "rossby_number": [{"Ro": 0.0969686820883, "f": 0.000103126079314}, {"Ro": 0.0137134425335, "f": 7.292115e-05}],
    "solar_elevation_angle": [{"elevation": 73.44, "zenith": 16.56, "azimuth": 180.0}, {"elevation": 32.7977513311, "zenith": 57.2022486689, "azimuth": 237.26759279}],
    "half_space_heat_flow": [{"q": 0.060931991121, "q_mW": 60.931991121, "thickness": 92401.5465817}, {"q": 0.148601621702, "q_mW": 148.601621702, "thickness": 36960.6186327}],
})


# --- CO2 radiative forcing -------------------------------------------------------------------------------------------
def co2_forcing(C, C0, lam=None):
    _finite(C=C, C0=C0)
    _need(1 <= C <= 1e5 and 1 <= C0 <= 1e5, "CO2 concentrations must be between 1 and 100,000 ppm")
    dF = 5.35 * math.log(C / C0)
    out = {"dF": _r(dF), "doublings": _r(math.log2(C / C0))}
    if lam is not None:
        _finite(lam=lam)
        _need(lam > 0, "the climate sensitivity parameter must be positive")
        out["dT_eq"] = _r(lam * dF)
    return out


def _ref_co2_forcing(C, C0, lam=None):
    # dF/dC = 5.35/C integrated from C0 to C (scipy quadrature)
    from scipy.integrate import quad
    dF = quad(lambda c: 5.35 / c, C0, C, epsabs=0, epsrel=1e-13, limit=200)[0] if C != C0 else 0.0
    out = {"dF": dF}
    if lam is not None:
        out["dT_eq"] = lam * dF
    return out


proc("co2_radiative_forcing", "Radiative forcing of a change in CO2 (Myhre et al. 1998)", "climate and radiation",
     "The radiative forcing dF = 5.35 ln(C/C0) W/m^2 of a change in CO2 concentration from C0 to C, and with a climate "
     "sensitivity parameter the equilibrium warming lambda dF.",
     [I("C", "number", "ppm", "CO2 concentration (ppm)", ("co2", "concentration", "co2 concentration"), "ppm"),
      I("C0", "number", "ppm", "reference CO2 concentration (ppm)", ("reference concentration", "initial concentration", "co2 reference"), "ppm"),
      I("lam", "climate_sensitivity", "climate_sensitivity", "climate sensitivity parameter (K per W/m^2, optional)",
        ("sensitivity", "climate sensitivity", "lambda"), "K/(W/m^2)", required=False)],
     [O("dF", "radiative_flux", "radiative forcing (W/m^2)"), O("doublings", "number", "number of CO2 doublings log2(C/C0)"),
      O("dT_eq", "temperature_difference", "equilibrium warming lambda dF (K)")],
     co2_forcing, _ref_co2_forcing,
     _guard(lambda C, C0, lam=None: (lambda o: dict(o, dF=_r(5.35 * math.log10(C / C0)), **({"dT_eq": _r(lam * 5.35 * math.log10(C / C0))} if lam else {})))(co2_forcing(C, C0, lam))),
     lambda rng: (lambda C0: {"C0": C0, "C": C0 * rng.choice([rng.uniform(0.3, 0.9), rng.uniform(1.1, 6.0)]),
                              **({"lam": rng.uniform(0.3, 1.5)} if rng.random() < 0.5 else {})})(rng.uniform(180, 600)),
     [{"C": 560, "C0": 280}, {"C": 420, "C0": 280, "lam": 0.8}], ["carbon dioxide radiative forcing", "carbon dioxide forcing", "radiative forcing of carbon dioxide"],
     conditions=["1 <= C, C0 <= 100,000 ppm", "the simplified logarithmic expression of Myhre et al. (1998)"],
     invalid=[{"C": -5, "C0": 280}], method="dF = 5.35 ln(C/C0) W/m^2; dT_eq = lambda dF",
     latex=r"\Delta F = 5.35\,\ln\frac{C}{C_0}\ \mathrm{W\,m^{-2}}")


# --- S-P epicentral distance -----------------------------------------------------------------------------------------
def sp_distance(dt, vp, vs):
    _finite(dt=dt, vp=vp, vs=vs)
    _need(dt > 0, "the S-P interval must be positive")
    _need(vp > vs > 0, "need vp > vs > 0")
    d = dt * vp * vs / (vp - vs)
    return {"distance": _r(d), "distance_km": _r(d / 1e3), "tp": _r(d / vp), "ts": _r(d / vs)}


def _ref_sp_distance(dt, vp, vs):
    # the distance at which the S wave arrives dt after the P wave (scipy brentq on the arrival-time difference)
    d = _brentq(lambda x: x / vs - x / vp - dt, 0.0, dt * vp * vs / (vp - vs) * 2 + 1.0)
    return {"distance_km": d / 1e3, "tp": d / vp, "ts": d / vs}


proc("sp_epicentral_distance", "Distance to an earthquake from the S-P interval", "seismology",
     "The distance from a station to the hypocentre from the delay between the S and P arrivals, for straight rays at "
     "constant P and S speeds.",
     [I("dt", "time", "time", "S minus P arrival time", ("s-p interval", "s p interval", "s-p time", "sp time", "delay"), "s"),
      I("vp", "speed", "speed", "P-wave speed", ("p wave speed", "p-wave speed", "p velocity"), "km/s"),
      I("vs", "speed", "speed", "S-wave speed", ("s wave speed", "s-wave speed", "s velocity"), "km/s")],
     [O("distance", "length", "distance to the source (m)"), O("distance_km", "number", "distance (km)"),
      O("tp", "time", "P travel time (s)"), O("ts", "time", "S travel time (s)")],
     sp_distance, _ref_sp_distance,
     _guard(lambda dt, vp, vs: (lambda d: {"distance": _r(d), "distance_km": _r(d / 1e3), "tp": _r(d / vp), "ts": _r(d / vs)})(dt * (vp - vs))),
     lambda rng: (lambda vp: {"vp": vp, "vs": vp / rng.uniform(1.6, 1.9), "dt": rng.uniform(1, 200)})(rng.uniform(5000, 8500)),
     [{"dt": 10, "vp": 6, "vs": 3.5}, {"dt": 45, "vp": 8, "vs": 4.5}],
     ["s p epicentral distance", "s p interval distance", "distance from the s p interval", "distance from s p time"],
     conditions=["vp > vs > 0", "straight rays at constant speeds (a local, crustal distance)"], invalid=[{"dt": 10, "vp": 3, "vs": 3.5}],
     method="d = dt vp vs/(vp - vs)", latex=r"d = \frac{\Delta t\,v_p v_s}{v_p - v_s}")


# --- Airy and Pratt isostasy -----------------------------------------------------------------------------------------
def airy_root(h, rho_c, rho_m, tc=None):
    _finite(h=h, rho_c=rho_c, rho_m=rho_m)
    _need(h > 0, "the elevation must be positive")
    _need(rho_m > rho_c > 0, "need rho_m > rho_c > 0")
    r = h * rho_c / (rho_m - rho_c)
    out = {"root": _r(r), "root_km": _r(r / 1e3)}
    if tc is not None:
        _finite(tc=tc)
        _need(tc > 0, "the normal crustal thickness must be positive")
        out["crust_thickness"] = _r(tc + h + r)
    return out


def _ref_airy_root(h, rho_c, rho_m, tc=None):
    # equal pressure at a compensation depth below both columns (crust of normal thickness T, mantle below) by
    # quadrature of rho g over each column; the root that balances them by brentq
    from scipy.integrate import quad
    T = tc if tc is not None else 35e3
    Dc = T + 5 * h * rho_c / (rho_m - rho_c) + 1e3                  # compensation depth below every root considered

    def column(top, root):                                              # top: elevation; crust down to T + root
        rho = lambda z: rho_c if z < T + root else rho_m                # noqa: E731  z measured down from sea level
        return sum(quad(lambda z: rho(z) * G0, a, b, epsabs=0, epsrel=1e-13)[0] for a, b in ((-top, T + root), (T + root, Dc)))
    ref = column(0.0, 0.0)
    r = _brentq(lambda x: column(h, x) - ref, 0.0, 5 * h * rho_c / (rho_m - rho_c))
    out = {"root": r}
    if tc is not None:
        out["crust_thickness"] = tc + h + r
    return out


proc("airy_isostasy", "Mountain root by Airy isostasy", "geodynamics",
     "The crustal root below topography of height h compensated by Airy isostasy (crust of uniform density floating on "
     "denser mantle), and with a normal crustal thickness the total thickness under the mountain.",
     [I("h", "length", "length", "elevation of the topography above sea level", ("elevation", "height", "topography"), "km"),
      I("rho_c", "density", "density", "crustal density", ("crustal density", "crust density"), "kg/m^3"),
      I("rho_m", "density", "density", "mantle density", ("mantle density",), "kg/m^3"),
      I("tc", "length", "length", "normal crustal thickness (optional)", ("crustal thickness", "normal crustal thickness"), "km", required=False)],
     [O("root", "length", "root depth below the normal crust (m)"), O("root_km", "number", "root depth (km)"),
      O("crust_thickness", "length", "total crustal thickness under the topography (m)")],
     airy_root, _ref_airy_root,
     _guard(lambda h, rho_c, rho_m, tc=None: (lambda r: dict({"root": _r(r), "root_km": _r(r / 1e3)}, **({"crust_thickness": _r(tc + h + r)} if tc else {})))(
         h * rho_m / (rho_m - rho_c))),
     lambda rng: {"h": rng.uniform(100, 8000), "rho_c": rng.uniform(2600, 2900), "rho_m": rng.uniform(3200, 3400),
                  **({"tc": rng.uniform(25e3, 45e3)} if rng.random() < 0.5 else {})},
     [{"h": 3, "rho_c": 2800, "rho_m": 3300}, {"h": 5, "rho_c": 2700, "rho_m": 3300, "tc": 35}],
     ["airy isostasy", "airy root", "isostatic root"],
     conditions=["h > 0", "rho_m > rho_c", "local (Airy) compensation, uniform crustal density"], invalid=[{"h": 3, "rho_c": 3400, "rho_m": 3300}],
     method="r = h rho_c/(rho_m - rho_c)", latex=r"r = \frac{\rho_c}{\rho_m - \rho_c}\,h")


def pratt_density(h, rho0, D):
    _finite(h=h, rho0=rho0, D=D)
    _need(h > 0, "the elevation must be positive")
    _need(rho0 > 0 and D > 0, "the reference density and the compensation depth must be positive")
    rho = rho0 * D / (D + h)
    return {"rho": _r(rho), "drho": _r(rho0 * h / (D + h))}


def _ref_pratt_density(h, rho0, D):
    # the uniform density of a column from elevation h down to the compensation depth D whose weight (quadrature of
    # rho g) equals that of the reference column of density rho0 from sea level to D (brentq)
    from scipy.integrate import quad
    ref = quad(lambda z: rho0 * G0, 0.0, D, epsabs=0, epsrel=1e-13)[0]
    rho = _brentq(lambda r: quad(lambda z: r * G0, -h, D, epsabs=0, epsrel=1e-13)[0] - ref, 0.0, rho0)
    return {"rho": rho, "drho": rho0 - rho}


proc("pratt_isostasy", "Column density by Pratt isostasy", "geodynamics",
     "The density of a crustal column standing h above sea level under Pratt isostasy (columns of different density "
     "down to a common compensation depth D), given the density of a column whose top is at sea level.",
     [I("h", "length", "length", "elevation of the column above sea level", ("elevation", "height", "topography"), "km"),
      I("rho0", "density", "density", "density of a column whose top is at sea level", ("reference density",), "kg/m^3"),
      I("D", "length", "length", "depth of compensation below sea level", ("compensation depth", "depth of compensation"), "km")],
     [O("rho", "density", "density of the elevated column (kg/m^3)"), O("drho", "density", "density deficit rho0 - rho (kg/m^3)")],
     pratt_density, _ref_pratt_density,
     _guard(lambda h, rho0, D: {"rho": _r(rho0 * (D - h) / D), "drho": _r(rho0 * h / D)}),
     lambda rng: {"h": rng.uniform(100, 8000), "rho0": rng.uniform(2700, 3000), "D": rng.uniform(5e4, 1.5e5)},
     [{"h": 3, "rho0": 2800, "D": 100}, {"h": 1, "rho0": 2900, "D": 80}],
     ["pratt isostasy", "pratt density", "pratt compensation"],
     conditions=["h > 0", "rho0, D > 0", "a column on land (no water load)"], invalid=[{"h": -3, "rho0": 2800, "D": 100}],
     method="rho = rho0 D/(D + h)", latex=r"\rho = \rho_0\,\frac{D}{D + h}")


# --- gradient wind ---------------------------------------------------------------------------------------------------
def gradient_wind(lat, R, Vg, flow):
    _lat_ok(lat)
    _finite(R=R, Vg=Vg)
    _need(5 <= abs(lat) <= 85, "gradient balance is used here for 5 to 85 degrees latitude")
    _need(R > 0 and Vg > 0, "the radius of curvature and the geostrophic wind must be positive")
    _need(flow in ("cyclonic", "anticyclonic"), "the flow must be cyclonic or anticyclonic")
    f = abs(2 * OMEGA * math.sin(math.radians(lat)))
    a = f * R / 2
    if flow == "cyclonic":
        V = f * R * Vg / (a + math.sqrt(a * a + f * R * Vg))          # = -a + sqrt(a^2 + f R Vg), without cancellation
    else:
        disc = a * a - f * R * Vg
        _need(disc >= 0, "no regular anticyclonic gradient wind: the geostrophic wind exceeds f R/4")
        V = f * R * Vg / (a + math.sqrt(disc))                          # = a - sqrt(a^2 - f R Vg), the regular root
    return {"V": _r(V), "ratio": _r(V / Vg), "f": _r(f)}


def _ref_gradient_wind(lat, R, Vg, flow):
    # the gradient-wind balance V^2/R +- f V - f Vg = 0 re-solved by brentq (cyclonic root between 0 and Vg; regular
    # anticyclonic root between Vg and f R/2), f from MetPy
    mc, u = _metpy()
    f = abs(float(mc.coriolis_parameter(lat * u.deg).m_as("1/s")))
    if flow == "cyclonic":
        V = _brentq(lambda v: v * v / R + f * v - f * Vg, 0.0, Vg)
    else:
        V = _brentq(lambda v: -v * v / R + f * v - f * Vg, Vg, f * R / 2)
    return {"V": V, "ratio": V / Vg, "f": f}


proc("gradient_wind", "Gradient wind in curved flow", "atmospheric dynamics",
     "The gradient wind speed of curved, frictionless flow from the geostrophic wind, the radius of curvature of the "
     "trajectory and the latitude, for cyclonic or anticyclonic flow (the regular solution).",
     [_LAT(), I("R", "length", "length", "radius of curvature of the flow", ("radius", "radius of curvature"), "km"),
      I("Vg", "speed", "speed", "geostrophic wind speed", ("geostrophic wind", "geostrophic speed"), "m/s"),
      I("flow", "number", "number", "cyclonic or anticyclonic", ("flow type", "curvature"), typ="choice", choices=["cyclonic", "anticyclonic"])],
     [O("V", "speed", "gradient wind speed (m/s)"), O("ratio", "number", "V/Vg"), O("f", "coriolis_parameter", "|f| (1/s)")],
     gradient_wind, _ref_gradient_wind,
     _guard(lambda lat, R, Vg, flow: {"V": _r(Vg), "ratio": 1.0, "f": _r(abs(2 * OMEGA * math.sin(math.radians(lat))))}),
     lambda rng: (lambda lat, R, flow: (lambda f: {"lat": lat, "R": R, "flow": flow,
                                                   "Vg": rng.uniform(3, 40) if flow == "cyclonic" else f * R / 4 * rng.uniform(0.1, 0.95)})(
         abs(2 * OMEGA * math.sin(math.radians(lat)))))(rng.choice([-1, 1]) * rng.uniform(15, 75), rng.uniform(2e5, 2e6), rng.choice(["cyclonic", "anticyclonic"])),
     [{"lat": 45, "R": 500, "Vg": 20, "flow": "cyclonic"}, {"lat": 45, "R": 1000, "Vg": 10, "flow": "anticyclonic"}],
     ["gradient wind"],
     conditions=["5 <= |latitude| <= 85 degrees", "R, Vg > 0", "anticyclonic: Vg <= |f| R/4"],
     invalid=[{"lat": 45, "R": 100, "Vg": 30, "flow": "anticyclonic"}],
     method="cyclonic V = -fR/2 + sqrt(f^2R^2/4 + f R Vg); anticyclonic V = fR/2 - sqrt(f^2R^2/4 - f R Vg)",
     latex=r"\frac{V^2}{R} \pm fV = fV_g")


# --- finite-depth wave dispersion ------------------------------------------------------------------------------------
def wave_dispersion(T, h):
    _finite(T=T, h=h)
    _need(0.5 <= T <= 60, "the wave period must be between 0.5 and 60 s")
    _need(0 < h <= 11000, "the depth must be between 0 and 11,000 m")
    w = 2 * math.pi / T
    y = w * w * h / G0                                                  # solve x tanh x = y for x = k h
    x = math.sqrt(y) if y < 1 else y                                    # a start above the root for Newton
    x = max(x, math.sqrt(y))
    for _ in range(100):
        t = math.tanh(x)
        f = x * t - y
        dx = f / (t + x * (1 - t * t))
        x -= dx
        if abs(dx) <= 1e-15 * x:
            break
    k = x / h
    c = w / k
    n = 0.5 * (1 + (2 * x / math.sinh(2 * x) if x < 350 else 0.0))
    return {"L": _r(2 * math.pi / k), "c": _r(c), "cg": _r(n * c), "kh": _r(x), "L0": _r(G0 * T * T / (2 * math.pi))}


def _ref_wave_dispersion(T, h):
    # the dispersion relation omega^2 = g k tanh(k h) re-solved for k by brentq; the group speed as d(omega)/dk by a
    # complex-step derivative of omega(k) = sqrt(g k tanh(k h)) (numpy)
    import numpy as np
    w = 2 * math.pi / T
    k0, ks = w * w / G0, w / math.sqrt(G0 * h)                     # tanh x < 1 and tanh x < x bound k from below
    k = _brentq(lambda kk: G0 * kk * math.tanh(kk * h) - w * w, 0.999 * max(k0, ks), 1.5 * (k0 + ks))
    eps = 1e-30 * k
    cg = float(np.imag(np.sqrt(G0 * (k + 1j * eps) * np.tanh((k + 1j * eps) * h))) / eps)
    return {"L": 2 * math.pi / k, "c": w / k, "cg": cg, "kh": k * h}


proc("wave_dispersion", "Water-wave dispersion at finite depth", "oceanography",
     "Wavelength, phase speed and group speed of a linear surface gravity wave of period T in water of depth h, from "
     "the full dispersion relation omega^2 = g k tanh(k h) (deep and shallow limits included).",
     [I("T", "time", "time", "wave period", ("period", "wave period"), "s"),
      I("h", "length", "length", "water depth", ("depth", "water depth"), "m")],
     [O("L", "length", "wavelength (m)"), O("c", "speed", "phase speed (m/s)"), O("cg", "speed", "group speed (m/s)"),
      O("kh", "number", "relative depth k h"), O("L0", "length", "deep-water wavelength g T^2/(2 pi) (m)")],
     wave_dispersion, _ref_wave_dispersion,
     _guard(lambda T, h: (lambda L0: {"L": _r(L0), "c": _r(L0 / T), "cg": _r(L0 / T / 2), "kh": _r(2 * math.pi * h / L0), "L0": _r(L0)})(G0 * T * T / (2 * math.pi))),
     lambda rng: (lambda T: {"T": T, "h": G0 * T * T / (2 * math.pi) * rng.uniform(0.02, 0.6)})(rng.uniform(2, 20)),
     [{"T": 10, "h": 20}, {"T": 6, "h": 100}], ["finite depth wave", "wave dispersion", "water wave dispersion", "wavelength at depth"],
     conditions=["0.5 <= T <= 60 s", "0 < h <= 11,000 m", "linear (small-amplitude) waves"], invalid=[{"T": 10, "h": -5}],
     method="omega^2 = g k tanh(k h) solved by Newton's method; c = omega/k; c_g = c (1 + 2kh/sinh 2kh)/2",
     latex=r"\omega^2 = g k \tanh(kh),\quad c_g = \frac{c}{2}\left(1 + \frac{2kh}{\sinh 2kh}\right)")


# --- Darcy flow ------------------------------------------------------------------------------------------------------
def darcy_flow(K, A, dh, L, n=None):
    _finite(K=K, A=A, dh=dh, L=L)
    _need(K > 0 and A > 0 and L > 0, "K, A and L must be positive")
    _need(dh > 0, "the head difference must be positive (flow from high to low head)")
    q = K * dh / L
    out = {"Q": _r(q * A), "q": _r(q), "gradient": _r(dh / L)}
    if n is not None:
        _finite(n=n)
        _need(0 < n <= 1, "the porosity must be between 0 and 1")
        out["v"] = _r(q / n)
    return out


def _ref_darcy_flow(K, A, dh, L, n=None):
    # steady 1-D groundwater flow d/dx(K dh/dx) = 0 between fixed heads, solved by finite differences (numpy); the
    # discharge from the computed head gradient
    import numpy as np
    N = 11
    M = np.zeros((N, N))
    b = np.zeros(N)
    M[0, 0] = M[-1, -1] = 1.0
    b[0], b[-1] = dh, 0.0
    for i in range(1, N - 1):
        M[i, i - 1], M[i, i], M[i, i + 1] = K, -2 * K, K
    head = np.linalg.solve(M, b)
    dx = L / (N - 1)
    q = -K * (head[1] - head[0]) / dx
    out = {"Q": q * A, "q": q, "gradient": dh / L}
    if n is not None:
        out["v"] = q / n
    return out


proc("darcy_flow", "Groundwater discharge by Darcy's law", "hydrogeology",
     "The volumetric discharge Q = K A dh/L through a porous medium, the specific discharge (Darcy flux) and, with a "
     "porosity, the average linear (seepage) velocity.",
     [I("K", "hydraulic_conductivity", "hydraulic_conductivity", "hydraulic conductivity", ("hydraulic conductivity", "conductivity"), "m/s"),
      I("A", "area", "area", "cross-sectional area of flow", ("area", "cross-sectional area"), "m^2"),
      I("dh", "length", "length", "head difference", ("head difference", "head loss"), "m"),
      I("L", "length", "length", "flow length", ("length", "flow length", "distance"), "m"),
      I("n", "number", "fraction", "effective porosity (optional)", ("porosity", "effective porosity"), "", required=False)],
     [O("Q", "flow_rate", "discharge (m^3/s)"), O("q", "speed", "specific discharge (Darcy flux, m/s)"),
      O("gradient", "number", "hydraulic gradient dh/L"), O("v", "speed", "average linear velocity q/n (m/s)")],
     darcy_flow, _ref_darcy_flow,
     _guard(lambda K, A, dh, L, n=None: (lambda q: dict({"Q": _r(q * A), "q": _r(q), "gradient": _r(L / dh)}, **({"v": _r(q / n)} if n else {})))(K * L / dh)),
     lambda rng: (lambda L: {"K": 10 ** rng.uniform(-8, -2), "A": rng.uniform(1, 1000), "L": L, "dh": L * rng.uniform(0.001, 0.5),
                             **({"n": rng.uniform(0.05, 0.45)} if rng.random() < 0.5 else {})})(rng.uniform(1, 2000)),
     [{"K": 1e-4, "A": 50, "dh": 2, "L": 100}, {"K": 5e-5, "A": 20, "dh": 5, "L": 250, "n": 0.3}],
     ["darcy flow", "darcy's law", "darcy discharge", "groundwater discharge"],
     conditions=["K, A, L > 0", "dh > 0", "laminar flow in a saturated, homogeneous medium"], invalid=[{"K": 1e-4, "A": -50, "dh": 2, "L": 100}],
     method="Q = K A dh/L, q = Q/A, v = q/n", latex=r"Q = K A\,\frac{\Delta h}{L}")


# --- stream power ----------------------------------------------------------------------------------------------------
def stream_power(Q, S, w=None, rho=1000.0):
    _finite(Q=Q, S=S, rho=rho)
    _need(Q > 0, "the discharge must be positive")
    _need(0 < S <= 1, "the slope must be between 0 and 1 (m/m)")
    _need(rho > 0, "the water density must be positive")
    om = rho * G0 * Q * S
    out = {"Omega": _r(om)}
    if w is not None:
        _finite(w=w)
        _need(w > 0, "the channel width must be positive")
        out["omega"] = _r(om / w)
    return out


def _ref_stream_power(Q, S, w=None, rho=1000.0):
    # the rate of potential-energy loss of the flow per unit channel length: a water mass rho Q per second descending
    # S metres per metre of channel, integrated over a 1 m reach (scipy quadrature of the weight times the drop rate)
    from scipy.integrate import quad
    om = quad(lambda x: rho * Q * G0 * S, 0.0, 1.0, epsabs=0, epsrel=1e-13)[0]
    out = {"Omega": om}
    if w is not None:
        out["omega"] = om / w
    return out


proc("stream_power", "Stream power of a river", "geomorphology",
     "The total stream power per unit channel length Omega = rho g Q S and, with the channel width, the unit stream "
     "power omega = Omega/w.",
     [I("Q", "flow_rate", "flow_rate", "discharge", ("discharge", "flow rate"), "m^3/s"),
      I("S", "number", "fraction", "channel slope (m/m)", ("slope", "channel slope", "gradient"), ""),
      I("w", "length", "length", "channel width (optional)", ("width", "channel width"), "m", required=False),
      I("rho", "density", "density", "water density (default 1000 kg/m^3)", ("water density", "density"), "kg/m^3", required=False)],
     [O("Omega", "stream_power_per_length", "total stream power per unit length (W/m)"),
      O("omega", "unit_stream_power", "unit stream power (W/m^2)")],
     stream_power, _ref_stream_power,
     _guard(lambda Q, S, w=None, rho=1000.0: (lambda om: dict({"Omega": _r(om)}, **({"omega": _r(om / w)} if w else {})))(rho * Q * S)),
     lambda rng: {"Q": 10 ** rng.uniform(-1, 4), "S": 10 ** rng.uniform(-5, -1), **({"w": rng.uniform(1, 500)} if rng.random() < 0.6 else {}),
                  **({"rho": rng.uniform(998, 1100)} if rng.random() < 0.3 else {})},
     [{"Q": 100, "S": 0.002}, {"Q": 50, "S": 0.005, "w": 20}], ["stream power", "unit stream power", "river power"],
     conditions=["Q > 0", "0 < S <= 1", "steady uniform flow"], invalid=[{"Q": 100, "S": 1.5}],
     method="Omega = rho g Q S, omega = Omega/w", latex=r"\Omega = \rho g Q S,\quad \omega = \frac{\Omega}{w}")


# --- Rossby number ---------------------------------------------------------------------------------------------------
def rossby_number(U, L, lat):
    _lat_ok(lat)
    _finite(U=U, L=L)
    _need(abs(lat) >= 1, "the Rossby number is not defined at the equator (|latitude| >= 1 degree)")
    _need(U > 0 and L > 0, "the speed and the length scale must be positive")
    f = abs(2 * OMEGA * math.sin(math.radians(lat)))
    return {"Ro": _r(U / (f * L)), "f": _r(f)}


def _ref_rossby_number(U, L, lat):
    mc, u = _metpy()
    f = abs(float(mc.coriolis_parameter(lat * u.deg).m_as("1/s")))
    return {"Ro": U / (f * L), "f": f}


proc("rossby_number", "Rossby number", "atmospheric dynamics",
     "The Rossby number Ro = U/(|f| L) of a flow with speed scale U and length scale L at a latitude: the ratio of "
     "inertial to Coriolis accelerations.",
     [I("U", "speed", "speed", "velocity scale", ("speed", "velocity", "velocity scale"), "m/s"),
      I("L", "length", "length", "horizontal length scale", ("length scale", "length"), "km"), _LAT()],
     [O("Ro", "number", "Rossby number"), O("f", "coriolis_parameter", "|f| (1/s)")],
     rossby_number, _ref_rossby_number,
     _guard(lambda U, L, lat: (lambda f: {"Ro": _r(U / (f * L)), "f": _r(f)})(abs(OMEGA * math.sin(math.radians(lat))))),
     lambda rng: {"U": rng.uniform(0.1, 50), "L": 10 ** rng.uniform(3, 7), "lat": rng.choice([-1, 1]) * rng.uniform(2, 89)},
     [{"U": 10, "L": 1000, "lat": 45}, {"U": 0.1, "L": 100, "lat": 30}], ["rossby number"],
     conditions=["|latitude| >= 1 degree", "U, L > 0"], invalid=[{"U": 10, "L": 1000, "lat": 0}],
     method="Ro = U/(|f| L), f = 2 Omega sin(latitude)", latex=r"Ro = \frac{U}{|f|\,L}")


# --- solar elevation angle -------------------------------------------------------------------------------------------
def solar_elevation(lat, decl, H):
    _lat_ok(lat)
    _finite(decl=decl, H=H)
    _need(-23.5 <= decl <= 23.5, "the solar declination must be between -23.5 and 23.5 degrees")
    _need(-180 <= H <= 180, "the hour angle must be between -180 and 180 degrees (negative before solar noon)")
    ph, de, h = math.radians(lat), math.radians(decl), math.radians(H)
    up = math.sin(ph) * math.sin(de) + math.cos(ph) * math.cos(de) * math.cos(h)
    east = -math.cos(de) * math.sin(h)
    north = math.cos(ph) * math.sin(de) - math.sin(ph) * math.cos(de) * math.cos(h)
    el = math.degrees(math.atan2(up, math.hypot(east, north)))
    _need(math.hypot(east, north) > 1e-9, "the sun is at the zenith: the azimuth is undefined")
    az = math.degrees(math.atan2(east, north)) % 360.0
    return {"elevation": _r(el), "zenith": _r(90.0 - el), "azimuth": _r(az)}


def _ref_solar_elevation(lat, decl, H):
    # the sun's unit vector in the hour-angle (equatorial) frame rotated into the local east-north-up frame by two
    # rotation matrices (numpy)
    import numpy as np
    de, h, ph = np.radians(decl), np.radians(H), np.radians(lat)
    s_eq = np.array([np.cos(de) * np.cos(h), -np.cos(de) * np.sin(h), np.sin(de)])     # x to the meridian, z to the pole
    a = np.pi / 2 - ph                                                                   # tilt the pole down to the zenith
    Ry = np.array([[np.cos(a), 0, -np.sin(a)], [0, 1, 0], [np.sin(a), 0, np.cos(a)]])
    v = Ry @ s_eq                                                                        # x: south... (horizon frame)
    south, east, up = v[0], v[1], v[2]
    north = -south
    el = np.degrees(np.arctan2(up, np.hypot(east, north)))
    az = np.degrees(np.arctan2(east, north)) % 360.0
    return {"elevation": float(el), "zenith": float(90 - el), "azimuth": float(az)}


proc("solar_elevation_angle", "Solar elevation and azimuth", "climate and radiation",
     "The sun's elevation above the horizon, its zenith angle and azimuth (clockwise from north) for a latitude, a "
     "solar declination and an hour angle (15 degrees per hour from solar noon, negative in the morning).",
     [_LAT(), I("decl", "number", "degrees", "solar declination (degrees)", ("declination", "solar declination"), "°"),
      I("H", "number", "degrees", "hour angle (degrees, negative before solar noon)", ("hour angle",), "°")],
     [O("elevation", "number", "solar elevation angle (degrees)"), O("zenith", "number", "solar zenith angle (degrees)"),
      O("azimuth", "number", "solar azimuth, clockwise from north (degrees)")],
     solar_elevation, _ref_solar_elevation,
     _guard(lambda lat, decl, H: (lambda el: {"elevation": _r(el), "zenith": _r(90 - el), "azimuth": 0.0})(
         math.degrees(math.asin(max(-1.0, min(1.0, math.sin(math.radians(lat)) * math.sin(math.radians(decl)) + math.cos(math.radians(lat)) * math.cos(math.radians(decl)))))))),
     lambda rng: {"lat": rng.uniform(-80, 80), "decl": rng.uniform(-23.4, 23.4), "H": rng.choice([-1, 1]) * rng.uniform(5, 120)},
     [{"lat": 40, "decl": 23.44, "H": 0}, {"lat": 40, "decl": 0, "H": 45}],
     ["solar elevation angle", "solar elevation", "sun elevation", "solar zenith angle"],
     conditions=["|declination| <= 23.5 degrees", "-180 <= hour angle <= 180 degrees", "geometric (no refraction)"],
     invalid=[{"lat": 40, "decl": 30, "H": 0}],
     method="sin(elevation) = sin(phi) sin(delta) + cos(phi) cos(delta) cos(H); azimuth from the east and north components",
     latex=r"\sin\alpha = \sin\varphi\sin\delta + \cos\varphi\cos\delta\cos H")


# --- half-space heat flow --------------------------------------------------------------------------------------------
ERFINV_09 = 1.1630871536766743                                       # erfinv(0.9)


def half_space_heat_flow(t, dT=1300.0, k=3.3, kappa=1e-6):
    _finite(t=t, dT=dT, k=k, kappa=kappa)
    _need(0 < t <= 200, "the seafloor age must be between 0 and 200 Myr")
    _need(dT > 0 and k > 0 and kappa > 0, "the temperature difference, conductivity and diffusivity must be positive")
    ts = t * 1e6 * YEAR
    q = k * dT / math.sqrt(math.pi * kappa * ts)
    return {"q": _r(q), "q_mW": _r(q * 1e3), "thickness": _r(2 * ERFINV_09 * math.sqrt(kappa * ts))}


def _ref_half_space_heat_flow(t, dT=1300.0, k=3.3, kappa=1e-6):
    # the half-space cooling temperature T(z) = Ts + dT erf(z/(2 sqrt(kappa t))): the surface gradient by a
    # complex-step derivative of scipy's erf; the thermal thickness where erf reaches 0.9 (brentq)
    from scipy.special import erf, erfinv
    ts = t * 1e6 * YEAR
    s = 2 * math.sqrt(kappa * ts)
    hstep = 1e-20 * s
    grad = float((dT * erf(complex(0.0, hstep) / s)).imag / hstep)
    z90 = float(erfinv(0.9)) * s
    return {"q": k * grad, "q_mW": k * grad * 1e3, "thickness": z90}


proc("half_space_heat_flow", "Surface heat flow of cooling oceanic lithosphere (half-space model)", "geothermics",
     "The conductive surface heat flow q = k dT/sqrt(pi kappa t) of oceanic lithosphere of age t (half-space cooling) "
     "and its thermal thickness. Defaults: dT 1300 K, k 3.3 W/(m K), kappa 1e-6 m^2/s.",
     [I("t", "number", "myr", "age of the seafloor (millions of years)", ("age",), "Myr"),
      I("dT", "temperature_difference", "temperature_difference", "mantle minus surface temperature (default 1300 K)", ("temperature difference",), "K", required=False),
      I("k", "thermal_conductivity", "thermal_conductivity", "thermal conductivity (default 3.3 W/(m K))", ("thermal conductivity", "conductivity"), "W/(m*K)", required=False),
      I("kappa", "thermal_diffusivity", "thermal_diffusivity", "thermal diffusivity (default 1e-6 m^2/s)", ("thermal diffusivity", "diffusivity"), "m^2/s", required=False)],
     [O("q", "heat_flux", "surface heat flow (W/m^2)"), O("q_mW", "number", "surface heat flow (mW/m^2)"),
      O("thickness", "length", "thermal thickness, where (T - Ts)/dT = 0.9 (m)")],
     half_space_heat_flow, _ref_half_space_heat_flow,
     _guard(lambda t, dT=1300.0, k=3.3, kappa=1e-6: (lambda ts: {"q": _r(k * dT / math.sqrt(kappa * ts)), "q_mW": _r(k * dT / math.sqrt(kappa * ts) * 1e3),
                                                                 "thickness": _r(2 * ERFINV_09 * math.sqrt(kappa * ts))})(t * 1e6 * YEAR)),
     lambda rng: {"t": rng.uniform(1, 180), **({"dT": rng.uniform(1200, 1450), "k": rng.uniform(2.5, 4.0), "kappa": rng.uniform(6e-7, 1.4e-6)} if rng.random() < 0.6 else {})},
     [{"t": 50}, {"t": 10, "dT": 1350, "k": 3.1, "kappa": 8e-7}],
     ["half space heat flow", "heat flow from half space cooling", "seafloor heat flow", "oceanic heat flow"],
     conditions=["0 < t <= 200 Myr", "conductive half-space cooling"], invalid=[{"t": 500}],
     method="q = k dT/sqrt(pi kappa t); thermal thickness 2 erfinv(0.9) sqrt(kappa t) = 2.326 sqrt(kappa t)",
     latex=r"q = \frac{k\,\Delta T}{\sqrt{\pi\kappa t}}")
