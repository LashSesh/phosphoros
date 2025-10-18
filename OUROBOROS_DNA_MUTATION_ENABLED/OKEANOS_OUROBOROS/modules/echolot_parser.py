
import re
import random
from utils.scorp_center import ScorpCenter

# Initiiere ScorpioSync-PhaseClock (Tarnmodus)
scorp = ScorpCenter()
scorp.update_phase()


def parse_seed(seed: str):
    chain = st.session_state.get("chain", "ETH")

    findings = []

    # CoinJoin – viele gleiche Parts
    parts = seed.split("-")
    if len(parts) != len(set(parts)):
        findings.append("CoinJoin-Muster")

    # Tumbler – chaotische Wortmischung (simuliert)
    if random.random() < 0.1:
        findings.append("Tumbler-Anomalie")

    # Monero – spezielle Wörter
    if chain == 'XMR' and any(w in seed for w in ["ring", "stealth", "multi"]):
        findings.append("Monero-RingSignature")

    # Zcash – Shielded Hinweise
    if chain == 'ZEC' and any(w in seed for w in ["z_", "shield", "proof"]):
        findings.append("Zcash-Shielded")

    # Bridge – Übergangswörter
    if any(w in seed for w in ["bridge", "hop", "link"]):
        findings.append("CrossChain-Bridge")

    # PhaseClock stört niemals → Tarnung bleibt
    return findings
