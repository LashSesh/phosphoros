
import streamlit as st
import os

def display_wiki():
    st.subheader("🧠 Strukturwissen: Gabriel Cells & 5D")

    wiki_dir = "wiki"
    if not os.path.exists(wiki_dir):
        st.warning("📁 Kein wiki/ Ordner gefunden.")
        return

    files = [f for f in os.listdir(wiki_dir) if f.endswith(".md")]
    if not files:
        st.info("Noch keine Inhalte im Wiki.")
        return

    selected = st.selectbox("🗂️ Thema wählen", sorted(files))
    file_path = os.path.join(wiki_dir, selected)
    with open(file_path, "r", encoding="utf-8") as f:
        content = f.read()
        st.markdown(content, unsafe_allow_html=True)
