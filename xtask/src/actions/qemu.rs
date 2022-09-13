use std::str::FromStr;

use super::errors::XError;


#[derive(Args)]
pub(crate) struct QemuArgs {
    #[clap(flatten)]
    build: BuildArgs,
    /// Number of hart (SMP for Symmetrical Multiple Processor).
    #[clap(long)]
    smp: Option<u8>,
    /// Port for gdb to connect. If set, qemu will block and wait gdb to connect.
    #[clap(long)]
    gdb: Option<u16>,
}

#[derive(Clone, Args)]
pub(crate) struct BuildArgs {
    #[clap(flatten)]
    pub arch: ArchArg,
    /// Build as debug mode.
    #[clap(long)]
    pub debug: bool,
    #[clap(long)]
    pub features: Option<String>,
}

#[derive(Copy, Clone, Args)]
pub(crate) struct ArchArg {
    /// Build architecture, `riscv64` or `x86_64`.
    #[clap(short, long)]
    pub arch: Arch,
}

/// 支持的 CPU 架构。
#[derive(Copy, Clone)]
pub(crate) enum Arch {
    Riscv64,
    X86_64,
    Aarch64,
}

impl FromStr for Arch {
    type Err = XError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "riscv64" => Ok(Self::Riscv64),
            "x86_64" => Ok(Self::X86_64),
            "aarch64" => Ok(Self::Aarch64),
            _ => Err(XError::EnumParse {
                _type: "Arch",
                value: s.into(),
            }),
        }
    }
}
