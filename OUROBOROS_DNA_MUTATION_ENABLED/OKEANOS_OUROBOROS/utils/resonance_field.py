
import streamlit as st
import json
import numpy as np
import matplotlib.pyplot as plt
from seed_embedder import SeedEmbedder
from sklearn.decomposition import PCA

st.set_page_config(page_title="TRITON Resonanzfeld", layout="wide")
st.title("TRITON X.3 – Geometrische Resonanzprojektion")

hitlog_file = "resonance_hits.json"
if not st.button("Visualisierung starten"):
    st.stop()

if not hitlog_file or not open(hitlog_file).read().strip():
    st.warning("Noch keine Treffer vorhanden.")
    st.stop()

with open(hitlog_file, "r") as f:
    hits = json.load(f)

embedder = SeedEmbedder()
vectors = []
labels = []

for hit in hits:
    try:
        vec = embedder.embed_seed(hit["seed"])
        vectors.append(vec)
        labels.append(hit["address"][:6])
    except:
        continue

if not vectors:
    st.error("Keine gültigen Seeds im Log gefunden.")
    st.stop()

vectors = np.stack(vectors)
pca = PCA(n_components=2)
projected = pca.fit_transform(vectors)

fig, ax = plt.subplots(figsize=(8, 6))
ax.scatter(projected[:, 0], projected[:, 1], alpha=0.7)

for i, label in enumerate(labels):
    ax.annotate(label, (projected[i, 0], projected[i, 1]), fontsize=7, alpha=0.6)

ax.set_title("Seed-Resonanzfeld (PCA Projektion)")
st.pyplot(fig)
