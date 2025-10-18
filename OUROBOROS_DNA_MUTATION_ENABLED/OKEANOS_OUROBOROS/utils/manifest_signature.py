
import streamlit as st
import json
import numpy as np
import matplotlib.pyplot as plt
from collective_field import CollectiveField
from sklearn.decomposition import PCA

st.set_page_config(page_title="TRITON Ω – Manifestation", layout="wide")
st.title("TRITON Ω – Signatur-Manifestation")

field = CollectiveField()
vectors = field.load_vectors()

if vectors.shape[0] == 0:
    st.warning("Noch keine Vektoren im Feld gespeichert.")
else:
    st.subheader("Resonanzfeld-Projektion (PCA)")
    pca = PCA(n_components=2)
    projected = pca.fit_transform(vectors)

    fig, ax = plt.subplots(figsize=(8, 6))
    ax.scatter(projected[:, 0], projected[:, 1], alpha=0.6)
    ax.set_title("TRITON Signaturraum")
    st.pyplot(fig)

    st.markdown("---")
    st.subheader("Feldzusammenfassung")
    st.text(f"Gespeicherte Vektoren: {len(vectors)}")
    st.text(f"Dimensionsreduktion: PCA → 2D")
