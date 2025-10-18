
from statistics import mean, stdev
from modules.resonance_instability import analyze_and_evolve
from modules.resonance_recycler import recycle_population
from modules.resonance_merit import evolve_population

def metakortex_scan(zellen, policy="balance"):
    if not zellen:
        return zellen

    iter_counts = [z.iterationen for z in zellen]
    avg_iter = mean(iter_counts)
    std_iter = stdev(iter_counts) if len(iter_counts) > 1 else 0
    extreme_high = [z for z in zellen if z.iterationen > avg_iter + std_iter * 2]
    extreme_low = [z for z in zellen if z.iterationen < avg_iter - std_iter * 1.5]

    if policy == "purge":
        return evolve_population(zellen, mode="merge")
    elif policy == "recycle":
        return recycle_population(zellen, strategy="smart")
    elif policy == "balance":
        if len(extreme_high) > 3 or len(extreme_low) > 3:
            return recycle_population(zellen, strategy="hybrid")
    elif policy == "adapt":
        return analyze_and_evolve(zellen)

    return zellen
