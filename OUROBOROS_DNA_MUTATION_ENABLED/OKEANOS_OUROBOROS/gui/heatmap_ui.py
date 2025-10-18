import streamlit as st
import pandas as pd
import os
import json
import matplotlib.pyplot as plt
import seaborn as sns

MEMORY_GRAPH_PATH = "data/memory_graph.json"


def load_memory_data():
    if not os.path.exists(MEMORY_GRAPH_PATH):
        return pd.DataFrame()
    with open(MEMORY_GRAPH_PATH, "r", encoding="utf-8") as f:
        data = json.load(f)
        return pd.DataFrame(data)


def display_heatmap():
    st.subheader("Resonanz-Heatmap")
    df = load_memory_data()

    if df.empty:
        st.warning("Keine Daten im Memory Graph gefunden.")
        return

    if "cluster_id" not in df.columns or "index" not in df.columns:
        st.error("Memory-Daten sind unvollständig.")
        return

    df["cluster_id"] = df["cluster_id"].astype(str)
    df["bucket"] = df["index"] // 100
    pivot = df.groupby(["cluster_id", "bucket"]).size().unstack(fill_value=0)

    fig, ax = plt.subplots(figsize=(12, 6))
    sns.heatmap(pivot, cmap="plasma", ax=ax, cbar=True)
    ax.set_title("Cluster x Index Heatmap")
    ax.set_xlabel("Index-Bereich (x100)")
    ax.set_ylabel("Cluster-ID")
    st.pyplot(fig)