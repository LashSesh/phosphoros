
import streamlit as st

def display_reflex_control():
    st.title("🧭 Reflexkontrolle")

    st.markdown("""
    Dieses Panel steuert die automatische Selbstregulation von OKEANOS.
    Hier lassen sich RID, Recycling, Metakortex und Evolution gezielt koordinieren.
    """)

    st.checkbox("RID aktivieren", key="use_rid", help="Aktiviert resonanzbasierte Instabilitätsüberwachung (RID)")
    st.checkbox("Recycling aktivieren", key="auto_recycle", help="Ermöglicht automatisches Recyceln schwacher/extremer Zellen")
    
    st.selectbox("Metakortex-Strategie", ["balance", "purge", "recycle", "adapt"], key="resonance_mode", help="Wähle die Reaktion auf Detektion instabiler Zellen")

    st.info("Diese Einstellungen beeinflussen alle automatischen Reflexprozesse, die während des Ouroboros-Takts ausgeführt werden.")
