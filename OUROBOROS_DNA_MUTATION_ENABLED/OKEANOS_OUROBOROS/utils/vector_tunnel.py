class VectorTunnel:
    def __init__(self, proto_stack):
        self.stack = proto_stack

    def transport(self, packet):
        encoded = f"<{self.stack[0]}>{packet}</{self.stack[-1]}>"
        return encoded  # Silent emission