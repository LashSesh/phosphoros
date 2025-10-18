
from modules.cell_manager import GabrielZelle
import random

def to_mutator(zelle):
    new = GabrielZelle(typ="Mutator")
    new.iterationen = zelle.iterationen // 2
    new.letzte_aktion = f"Recycelt aus {zelle.id} (hochentropisch)"
    return new

def to_shadow(zelle):
    zelle.status = "archiviert"
    zelle.typ = "Schatten"
    zelle.letzte_aktion = f"Verlagert ins Schattenarchiv"
    return zelle

def fuse_into_experiment(z1, z2):
    fusion = GabrielZelle(typ="Experiment")
    fusion.iterationen = (z1.iterationen + z2.iterationen) // 3
    fusion.letzte_aktion = f"Fusioniert: {z1.id} + {z2.id}"
    return fusion

def recycle_population(zellen, strategy="smart"):
    new_population = []
    sorted_zellen = sorted(zellen, key=lambda z: z.iterationen)

    for z in zellen:
        if z.iterationen > 1500 and strategy in ["smart", "mutate"]:
            new_population.append(to_mutator(z))
        elif z.iterationen < 5 and strategy in ["smart", "shadow"]:
            new_population.append(to_shadow(z))
        else:
            new_population.append(z)

    if strategy == "hybrid" and len(sorted_zellen) > 3:
        z1 = sorted_zellen[0]
        z2 = sorted_zellen[1]
        fusion = fuse_into_experiment(z1, z2)
        new_population.remove(z1)
        new_population.remove(z2)
        new_population.append(fusion)

    return new_population
