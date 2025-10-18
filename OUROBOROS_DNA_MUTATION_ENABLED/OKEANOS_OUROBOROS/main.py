
import streamlit as st
from gui.cell_manager_ui import display_cell_manager_ui
from gui.seed_generator_ui import display_seed_generator
from gui.memory_ui import display_memory_ui
from gui.echolot_ui import display_echolot_ui
from gui.puls_monitor_ui import display_puls_monitor
from gui.wiki_ui import display_wiki
from gui.theme_switcher import load_theme

st.set_page_config(page_title="OKEANOS", layout="wide")

with st.sidebar:
    st.title("🌀 OKEANOS")
    theme = load_theme()

    st.markdown("## 🧭 Navigation")
    page = st.radio("Modul wählen", [
        "Zellsteuerung",
        "Seed Generator",
        "Memory",
        "Echolot",
        "Puls Monitor",
        "Strukturwissen"
    ])

# Layout-Bereich
st.markdown(f"""
    <style>
        html, body {{
            background-color: {theme['bg']};
            color: {theme['text']};
        }}
        .block-container {{
            padding-top: 2rem;
        }}
    </style>
""", unsafe_allow_html=True)

# Page-Routing
if page == "Zellsteuerung":
    display_cell_manager_ui()
elif page == "Seed Generator":
    display_seed_generator()
elif page == "Memory":
    display_memory_ui()
elif page == "Echolot":
    display_echolot_ui()
elif page == "Puls Monitor":
    display_puls_monitor()
elif page == "Strukturwissen":
    display_wiki()
