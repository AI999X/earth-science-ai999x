"""earth-science-ai999x — offline earth science answers with their formula, steps and verification.

    import earth_science_ai999x as m
    m.ask("Lifting condensation level with pressure = 1000 hPa, temperature = 30 °C and dewpoint = 20 °C.").show()
    # p_lcl = 86421.3 Pa, T_lcl = 290.812 K
    m.solve("seawater_density", S=35, T="2 degC", p="40 MPa").show()       # 40 MPa = 4000 dbar
    # rho = 1046.02 kg/m^3 (EOS-80 in-situ density)
    m.capabilities()                   # every certified capability (74)

What it covers, from school to graduate level:
- atmospheric thermodynamics and moisture: potential and equivalent potential temperature, dry and moist adiabatic
  lapse rates, saturation vapour pressure, dewpoint, relative humidity, mixing ratio, specific humidity, virtual
  temperature, moist-air density, static energy, the lifting condensation level (Romps 2017);
- the hydrostatic atmosphere: barometric formula, scale height, hypsometric thickness, constant-lapse-rate and
  standard atmospheres, geopotential;
- meteorology and dynamics: wind components, wind chill, heat index, Coriolis parameter, inertial period, geostrophic
  wind, Brunt-Väisälä frequency, Rossby waves;
- radiation and climate: Stefan-Boltzmann and Wien laws, planetary equilibrium temperature, one-layer and N-layer
  greenhouse models, daily mean insolation, the transient response of a one-box energy-balance model;
- oceanography: EOS-80 seawater density, freezing point, speed of sound, adiabatic gradient, potential temperature,
  pressure and depth conversion, shallow-water (tsunami) waves, Ekman depth and transport;
- seismology and geophysics: P and S speeds and elastic constants, epicentral and WGS84 geodesic distance and azimuth,
  moment and local magnitude, Gutenberg-Richter b-value, NMO velocity, gravity at latitude, Bouguer slab and buried
  sphere anomalies, dipole inclination and paleolatitude, plate velocity about an Euler pole;
- geology: steady geotherms with radiogenic heating, seafloor depth with age, radioactive decay, radiometric, K-Ar,
  isochron and radiocarbon ages, Stokes settling of grains.

Questions take one documented form: the capability's name, then "with name = value unit, ..." (lists in brackets).
Inputs are SI in solve(); latitudes and angles are degrees, ages years. Every capability was certified against an
independent computation (MetPy, python-seawater, ObsPy, SciPy, NumPy, SymPy) at build time; those libraries are not
needed to use the package. A question that is not determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.0"
_P = Product("earth_science", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
