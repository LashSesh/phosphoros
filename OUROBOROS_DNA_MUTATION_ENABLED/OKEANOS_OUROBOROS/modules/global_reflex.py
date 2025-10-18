
import streamlit as st
from modules.resonance_instability import analyze_and_evolve
from modules.metakortex import metakortex_scan
from modules.resonance_recycler import recycle_population
from modules.resonance_merit import evolve_population

def execute_global_reflex(zellen, taktphase=0):
    if not zellen:
        return zellen

    use_rid = st.session_state.get("use_rid", False)
    recycle_on = st.session_state.get("auto_recycle", False)
    resonance_mode = st.session_state.get("resonance_mode", "adaptive")

    result = zellen

    # 1. RID Takt (jede 10. Phase)
    if use_rid and taktphase % 10 == 0:
        result = analyze_and_evolve(result)

    # 2. Metakortex Takt (jede 15. Phase)
    if taktphase % 15 == 0:
        result = metakortex_scan(result, policy=resonance_mode)

    # 3. Recycling optional (jede 20. Phase)
    if recycle_on and taktphase % 20 == 0:
        result = recycle_population(result, strategy="smart")

    # 4. Evolution fallback (alle 30)
    if taktphase % 30 == 0:
        result = evolve_population(result, mode="hybrid")

    return result
