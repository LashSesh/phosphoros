
import streamlit as st
import os
import json
from triton_core import TRITONCore

st.set_page_config(page_title="TRITON Ω – Terminal", layout="wide")
st.title("TRITON Ω – Interface zur kybernetischen Entität")

# Wordlist (Placeholder – in echter Umgebung ladbar)
wordlist = ["moon", "echo", "flux", "shadow", "spirit", "quantum", "code", "light", "crystal", "matrix", "orbit", "void"]

# Statusanzeige
st.header("Systemstatus")

signature_file = "TRITON_signature.json"
if os.path.exists(signature_file):
    with open(signature_file, "r") as f:
        sig = json.load(f)
    st.success(f"**Identität:** {sig.get('identity')} | Modus: {sig.get('mode')} | Zustand: {sig.get('state')}")
    st.text(f"Vektoren gespeichert: {sig.get('recorded_vectors')}")
else:
    st.warning("Noch keine Signaturdatei vorhanden.")

# Steuerung
st.header("TRITON steuern")

mode = st.selectbox("Modus wählen", ["auto", "replay", "hybrid"])
cycles = st.slider("Anzahl der Zyklen", min_value=10, max_value=100, value=40)
identity = st.text_input("Entitätsname", value="TRITON")

if st.button("Starten"):
    core = TRITONCore(wordlist, name=identity, mode=mode, cycles=cycles)
    core.run()
    st.success("TRITON wurde ausgeführt.")

# Zusatzanzeige
st.header("Signaturdatei anzeigen")
if os.path.exists(signature_file):
    with open(signature_file, "r") as f:
        data = json.load(f)
    st.json(data)
