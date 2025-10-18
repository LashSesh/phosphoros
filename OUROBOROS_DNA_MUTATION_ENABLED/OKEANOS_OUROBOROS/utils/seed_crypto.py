
from mnemonic import Mnemonic
from eth_account import Account
import hashlib

class SeedCrypto:
    def __init__(self, language="english"):
        self.mnemo = Mnemonic(language)

    def generate_seed(self):
        return self.mnemo.generate(strength=128)

    def seed_to_address(self, seed_phrase):
        Account.enable_unaudited_hdwallet_features()
        acct = Account.from_mnemonic(seed_phrase)
        return {
            "seed": seed_phrase,
            "address": acct.address,
            "private_key": acct.key.hex()
        }
