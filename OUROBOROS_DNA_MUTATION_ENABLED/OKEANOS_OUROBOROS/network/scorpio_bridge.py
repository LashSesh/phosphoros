
import streamlit as st

def get_chain_provider():
    chain = st.session_state.get("chain", "ETH")
    if chain == "ETH":
        return "https://cloudflare-eth.com"
    elif chain == "BTC":
        return "https://api.blockcypher.com/v1/btc/main/addrs/"
    elif chain == "LTC":
        return "https://api.blockcypher.com/v1/ltc/main/addrs/"
    elif chain == "DOGE":
        return "https://sochain.com/api/v2/get_address_balance/DOGE/"
    else:
        return None  # XMR, ZEC – meist nicht öffentlich abrufbar

from web3 import Web3

class ScorpioBridge:
    def __init__(self, provider_url="https://cloudflare-eth.com"):
        self.w3 = Web3(Web3.HTTPProvider(provider_url))

    def check_seed(self, address):
        try:
            balance_wei = self.w3.eth.get_balance(address)
            balance_eth = self.w3.fromWei(balance_wei, 'ether')
            return float(balance_eth)
        except Exception as e:
            print(f"[ScorpioBridge] Fehler bei Balance-Check: {e}")
            return None
