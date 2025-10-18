
import streamlit as st
import pandas as pd
import os
import json
import matplotlib.pyplot as plt
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.decomposition import PCA

MEMORY_GRAPH_PATH = "data/memory_graph.json"

def load_seeds_from_memory():
    if not os.path.exists(MEMORY_GRAPH_PATH):
        return [], []
    with open(MEMORY_GRAPH_PATH, "r", encoding="utf-8") as f:
        data = json.load(f)
        df = pd.DataFrame(data)
        seeds = df.get("mutated_seed", [])
        labels = df.get("cluster_id", [])
        return list(seeds), list(labels)

def display_embedding_map():
    st.subheader("🌌 Seed Embedding Map (TF-IDF + PCA)")
    seeds, labels = load_seeds_from_memory()

    if not seeds or not labels:
        st.warning("Keine Seeds oder Labels im Speicher vorhanden.")
        return

    with st.spinner("Berechne TF-IDF Embeddings..."):
        vectorizer = TfidfVectorizer()
        tfidf_matrix = vectorizer.fit_transform(seeds)
        pca = PCA(n_components=2)
        reduced = pca.fit_transform(tfidf_matrix.toarray())

    df = pd.DataFrame(reduced, columns=["x", "y"])
    df["label"] = labels

    fig, ax = plt.subplots(figsize=(10, 6))
    scatter = ax.scatter(df["x"], df["y"], c=pd.factorize(df["label"])[0], cmap="tab10", alpha=0.75)
    ax.set_title("Seed-Cluster im Vektorraum (leichtgewichtige Embedding Map)")
    ax.set_xlabel("x")
    ax.set_ylabel("y")
    st.pyplot(fig)
