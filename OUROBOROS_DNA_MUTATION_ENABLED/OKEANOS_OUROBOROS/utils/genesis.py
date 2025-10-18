from scorp_center import ScorpCenter
from ilu_node import ILUNode
from entropy_seed import EntropySeed
from phase_mask import PhaseMask

def main(silent=True):
    EntropySeed().initialize()
    center = ScorpCenter()
    ilu = ILUNode("ILU-GENESIS", 0.05)
    center.register_ilu(ilu)

    for _ in range(1000):
        center.update_phase()
        if ilu.is_resonant(center.phase_clock):
            packet = "init"
            masked = PhaseMask().apply(packet)
            ilu.emit_packet(masked)

if __name__ == "__main__":
    main()