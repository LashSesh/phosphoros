from vector_tunnel import VectorTunnel

class ILUNode:
    def __init__(self, id, base_phase):
        self.id = id
        self.phase_offset = base_phase
        self.protocol_stack = ["HTTP", "WS", "TLS", "DNS"]

    def is_resonant(self, global_phase):
        return abs(global_phase - self.phase_offset) < 0.05

    def emit_packet(self, packet):
        tunnel = VectorTunnel(self.protocol_stack)
        tunnel.transport(packet)