
from web3 import Web3

class AddressChecker:
    def __init__(self, provider_url="https://cloudflare-eth.com"):
        self.w3 = Web3(Web3.HTTPProvider(provider_url))

    def check_balance(self, address):
        try:
            balance_wei = self.w3.eth.get_balance(address)
            balance_eth = self.w3.fromWei(balance_wei, 'ether')
            return float(balance_eth)
        except Exception as e:
            print(f"Fehler bei Balance-Check: {e}")
            return None

# Beispiel:
# checker = AddressChecker()
# bal = checker.check_balance("0x...")
# print(bal)
