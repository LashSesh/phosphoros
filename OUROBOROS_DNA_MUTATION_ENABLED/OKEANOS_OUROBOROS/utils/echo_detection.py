
import re

class RegexCryptoParser:
    def __init__(self):
        self.patterns = {}

    def add_pattern(self, pattern_name, regex_pattern):
        self.patterns[pattern_name] = re.compile(regex_pattern, re.IGNORECASE)

    def parse_transaction(self, transaction):
        matched_patterns = []
        for pattern_name, regex in self.patterns.items():
            if regex.search(transaction):
                matched_patterns.append(pattern_name)
        return matched_patterns

class UserAdaptiveEchoDetection:
    def __init__(self, parser, initial_threshold=0.15):
        self.detected_echoes = []
        self.threshold = initial_threshold
        self.parser = parser

    def set_threshold(self, new_threshold):
        self.threshold = new_threshold

    def analyze_transaction(self, transaction):
        matched_patterns = self.parser.parse_transaction(transaction)
        match_ratio = len(matched_patterns) / len(self.parser.patterns)
        if match_ratio >= self.threshold:
            self.detected_echoes.append(transaction)
            self._adapt_threshold(True)
            return True, matched_patterns
        else:
            self._adapt_threshold(False)
            return False, matched_patterns

    def _adapt_threshold(self, detected):
        if detected:
            self.threshold = min(self.threshold + 0.01, 0.8)
        else:
            self.threshold = max(self.threshold - 0.01, 0.1)
