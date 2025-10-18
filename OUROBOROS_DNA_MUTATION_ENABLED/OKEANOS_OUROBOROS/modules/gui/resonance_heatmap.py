
import streamlit as st
import json
import os
import pandas as pd
import numpy as np
import seaborn as sns
import matplotlib.pyplot as plt

MEMORY_GRAPH_PATH = "data/memory_graph.json"

@st.cache_data
def load_memory_graph():
    if not os.path.exists(MEMORY_GRAPH_PATH):
        return []
    with open(MEMORY_GRAPH_PATH, "r", encoding="utf-8") as f:
        return json.load(f)

def build_heatmap(df):
    df["cluster_id"] = df["cluster_id"].astype(str)
    df["bucket"] = df["index"] // 100  # Gruppierung der Indizes in Bereiche
    pivot = df.groupby(["cluster_id", "bucket"]).size().unstack(fill_value=0)

    st.subheader("🔥 Resonanz-Heatmap (Cluster x Indexraum)")
    fig, ax = plt.subplots(figsize=(12, 6))
    sns.heatmap(pivot, cmap="plasma", ax=ax, cbar=True)
    ax.set_xlabel("Index-Bereich (x100)")
    ax.set_ylabel("Cluster-ID")
    st.pyplot(fig)

st.set_page_config(page_title="TRITON Resonanz-Heatmap", layout="wide")
st.title("🌀 TRITON X – Resonanz-Heatmap")

memory = load_memory_graph()
if not memory:
    st.warning("Keine Treffer im Memory Graph gefunden.")
else:
    df = pd.DataFrame(memory)
    build_heatmap(df)
