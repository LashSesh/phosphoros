
import streamlit as st
from modules.cell_manager import CellManager
from modules.ouroboros_engine import OuroborosEngine

def display_cell_manager_ui():
    st.title("🧬 Zellsteuerung")

    if "cell_manager" not in st.session_state:
        st.session_state.cell_manager = CellManager()

    manager = st.session_state.cell_manager

    if "ouroboros" not in st.session_state:
        st.session_state.ouroboros = OuroborosEngine(manager)

    col1, col2 = st.columns(2)

    with col1:
        st.subheader("Zellen hinzufügen")
        typ = st.selectbox("Zelltyp wählen", ["Navigator", "Mutator", "Watcher", "Collector", "Supervisor"])
        anzahl = st.number_input("Anzahl", min_value=1, max_value=50, value=1, step=1)
        if st.button("Zellen erzeugen"):
            manager.batch_add_zellen(typ, anzahl)
            st.success(f"{anzahl} Zellen vom Typ {typ} erzeugt.")

    with col2:
        st.subheader("Ouroboros")
        if st.button("🌀 Ouroboros starten"):
            st.session_state.ouroboros.start()
            st.success("Ouroboros gestartet.")
        if st.button("⛔ Ouroboros stoppen"):
            st.session_state.ouroboros.stop()
            st.warning("Ouroboros gestoppt.")

    st.markdown("---")
    st.subheader("Aktive Zellen")

    if manager.zellen:
        for z in manager.zellen:
            with st.expander(f"🧪 [{z.typ}] {z.id}"):
                st.write(f"Iterationen: {z.iterationen}")
                st.write(f"Status: {z.status}")
                st.write(f"Letzte Aktion: {z.letzte_aktion}")
                if st.button(f"Löschen: {z.id}", key=f"del_{z.id}"):
                    manager.remove_zelle(z.id)
                    st.experimental_rerun()
    else:
        st.info("Noch keine Zellen erzeugt.")
