
import streamlit as st
from statistics import mean, stdev
from modules.resonance_merit import evolve_population, evaluate_fitness

def analyze_and_evolve(zellen, mode="hybrid", threshold_score=0.5, entropy_check=True):
    if not zellen:
        return zellen

    fitness_scores = [evaluate_fitness(z) for z in zellen]
    avg_score = mean(fitness_scores)
    std_dev = stdev(fitness_scores) if len(fitness_scores) > 1 else 0

    st.session_state.rid_metrics = {
        "average_score": round(avg_score, 4),
        "score_deviation": round(std_dev, 4),
        "zellen_count": len(zellen)
    }

    instability_triggered = False

    if avg_score < threshold_score:
        instability_triggered = True
        st.warning("RID: Niedriger Score-Durchschnitt – evolutionärer Impuls ausgelöst.")
    elif entropy_check and std_dev > 0.15:
        instability_triggered = True
        st.warning("RID: Resonanzinstabilität – Standardabweichung zu hoch.")

    if instability_triggered:
        return evolve_population(zellen, mode=mode)
    return zellen
