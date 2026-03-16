//! AuroraOS super-prompt represented as executable Rust data.
//! This file intentionally provides real Rust code (not Markdown prose)
//! so the architecture can be consumed by tools.

#[derive(Debug, Clone)]
pub struct AuroraSpec {
    pub name: &'static str,
    pub language: &'static str,
    pub architecture: &'static str,
    pub boot_modes: &'static [&'static str],
    pub kernel_type: &'static str,
    pub key_features: &'static [&'static str],
    pub boot_flow: &'static [&'static str],
    pub phases: &'static [&'static str],
}

impl AuroraSpec {
    pub fn default_spec() -> Self {
        Self {
            name: "AuroraOS",
            language: "Rust (nightly)",
            architecture: "x86_64",
            boot_modes: &["UEFI", "BIOS"],
            kernel_type: "Hybrid microkernel",
            key_features: &[
                "Multitasking",
                "Virtual memory",
                "Filesystem",
                "Networking",
                "Device drivers",
                "GUI compositor",
                "SMP multicore",
                "Userland applications",
            ],
            boot_flow: &[
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
            phases: &[
                "Phase 1: Boot + kernel",
                "Phase 2: Memory + interrupts",
                "Phase 3: Scheduler + processes",
                "Phase 4: Filesystem",
                "Phase 5: Drivers",
                "Phase 6: Networking",
                "Phase 7: GUI",
            ],
        }
    }

    pub fn to_text(&self) -> String {
        let mut output = String::new();

        output.push_str("AuroraOS Specification\n");
        output.push_str("======================\n");
        output.push_str(&format!("Name: {}\n", self.name));
        output.push_str(&format!("Language: {}\n", self.language));
        output.push_str(&format!("Architecture: {}\n", self.architecture));
        output.push_str(&format!("Kernel type: {}\n\n", self.kernel_type));

        output.push_str("Boot modes:\n");
        for mode in self.boot_modes {
            output.push_str(&format!("  - {}\n", mode));
        }

        output.push_str("\nFeatures:\n");
        for feature in self.key_features {
            output.push_str(&format!("  - {}\n", feature));
        }

        output.push_str("\nBoot flow:\n");
        for (index, stage) in self.boot_flow.iter().enumerate() {
            output.push_str(&format!("  {}. {}\n", index + 1, stage));
        }

        output.push_str("\nDevelopment phases:\n");
        for phase in self.phases {
            output.push_str(&format!("  - {}\n", phase));
        }

        output
    }
}

fn main() {
    let spec = AuroraSpec::default_spec();
    println!("{}", spec.to_text());
}
