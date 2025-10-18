
import streamlit as st
from gui.cell_manager_ui import display_cell_manager_ui
from gui.seed_generator_ui import display_seed_generator
from gui.echolot_ui import display_echolot_ui
from gui.memory_ui import display_memory_ui
from gui.puls_monitor_ui import display_puls_monitor
from gui.reflex_control_ui import display_reflex_control
from gui.wiki_ui import display_wiki
from gui.theme_switcher import apply_theme

# Seite konfigurieren
st.set_page_config(page_title="OKEANOS GUI", layout="wide")

# Theme anwenden
apply_theme()

# Chain-Wahl sichtbar machen
if "chain" not in st.session_state:
    st.session_state["chain"] = "ETH"

chain = st.selectbox("Blockchain auswählen", ["ETH", "BTC", "XMR", "ZEC", "LTC", "DOGE"], key="chain")
st.caption(f"Aktive Chain: {chain}")

# Tabs anzeigen
tabs = st.tabs([
    "Zellsteuerung", "Seed-Generator", "Echolot", "MemoryField",
    "Pulsmonitor", "Reflexkontrolle", "Strukturwissen"
])

with tabs[0]:
    display_cell_manager_ui()

with tabs[1]:
    display_seed_generator()

with tabs[2]:
    display_echolot_ui()

with tabs[3]:
    display_memory_ui()

with tabs[4]:
    display_puls_monitor()

with tabs[5]:
    display_reflex_control()

with tabs[6]:
    display_wiki()
