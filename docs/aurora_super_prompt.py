"""AuroraOS super-prompt represented as executable Python data.

This file intentionally provides real Python code (not Markdown prose)
so build or tooling scripts can import and reuse the specification.
"""

from dataclasses import dataclass
from typing import List


@dataclass(frozen=True)
class AuroraSpec:
    name: str
    language: str
    architecture: str
    boot_modes: List[str]
    kernel_type: str
    key_features: List[str]
    boot_flow: List[str]
    phases: List[str]

    @staticmethod
    def default() -> "AuroraSpec":
        return AuroraSpec(
            name="AuroraOS",
            language="Rust (nightly)",
            architecture="x86_64",
            boot_modes=["UEFI", "BIOS"],
            kernel_type="Hybrid microkernel",
            key_features=[
                "Multitasking",
                "Virtual memory",
                "Filesystem",
                "Networking",
                "Device drivers",
                "GUI compositor",
                "SMP multicore",
                "Userland applications",
            ],
            boot_flow=[
                "UEFI/BIOS",
                "Bootloader",
                "Kernel entry",
                "Memory manager",
                "Interrupt system",
                "Driver initialization",
                "Scheduler start",
                "Process system",
                "Launch init",
                "Launch shell/GUI",
            ],
            phases=[
                "Phase 1: Boot + kernel",
                "Phase 2: Memory + interrupts",
                "Phase 3: Scheduler + processes",
                "Phase 4: Filesystem",
                "Phase 5: Drivers",
                "Phase 6: Networking",
                "Phase 7: GUI",
            ],
        )

    def as_text(self) -> str:
        lines = [
            "AuroraOS Specification",
            "======================",
            f"Name: {self.name}",
            f"Language: {self.language}",
            f"Architecture: {self.architecture}",
            f"Kernel type: {self.kernel_type}",
            "",
            "Boot modes:",
            *[f"  - {mode}" for mode in self.boot_modes],
            "",
            "Features:",
            *[f"  - {feature}" for feature in self.key_features],
            "",
            "Boot flow:",
            *[f"  {idx + 1}. {stage}" for idx, stage in enumerate(self.boot_flow)],
            "",
            "Development phases:",
            *[f"  - {phase}" for phase in self.phases],
        ]
        return "\n".join(lines)


if __name__ == "__main__":
    print(AuroraSpec.default().as_text())
