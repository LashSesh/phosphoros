
import streamlit as st
import json
import os
import pandas as pd
from sklearn.manifold import TSNE
import matplotlib.pyplot as plt
from sentence_transformers import SentenceTransformer

MEMORY_GRAPH_PATH = "data/memory_graph.json"

@st.cache_data
def load_memory_graph():
    if not os.path.exists(MEMORY_GRAPH_PATH):
        return []
    with open(MEMORY_GRAPH_PATH, "r", encoding="utf-8") as f:
        return json.load(f)

@st.cache_data
def embed_seeds(seeds):
    model = SentenceTransformer("all-MiniLM-L6-v2")
    embeddings = model.encode(seeds, show_progress_bar=True)
    return embeddings

def plot_embeddings(embeddings, labels):
    tsne = TSNE(n_components=2, random_state=42, perplexity=5)
    reduced = tsne.fit_transform(embeddings)
    df = pd.DataFrame(reduced, columns=["x", "y"])
    df["label"] = labels

    fig, ax = plt.subplots()
    scatter = ax.scatter(df["x"], df["y"], c=pd.factorize(df["label"])[0], cmap="Spectral", alpha=0.7)
    ax.set_title("TRITON X – Seed Embedding Map")
    st.pyplot(fig)

st.set_page_config(page_title="TRITON Embedding", layout="wide")
st.title("🧭 TRITON X – Seed Embedding Map")

memory = load_memory_graph()
if not memory:
    st.warning("Keine Treffer im Memory Graph gefunden.")
else:
    seeds = [entry["mutated_seed"] for entry in memory]
    labels = [entry["cluster_id"] for entry in memory]
    embeddings = embed_seeds(seeds)
    plot_embeddings(embeddings, labels)
