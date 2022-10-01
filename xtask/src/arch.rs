use std::str::FromStr;

use super::constants::*;
use crate::errors::XError;

/// 支持的 CPU 架构。
#[derive(Copy, Clone, Debug)]
pub(crate) enum Arch {
    Riscv64,
    X86_64,
    Aarch64,
}

impl FromStr for Arch {
    type Err = XError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            ARCH_RISCV64 => Ok(Self::Riscv64),
            ARCH_X86_64 => Ok(Self::X86_64),
            ARCH_AARCH64 => Ok(Self::Aarch64),
            _ => Err(XError::EnumParse {
                _type: "Arch",
                value: s.into(),
            }),
        }
    }
}

impl Arch {
    /// Returns the name of Arch.
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Riscv64 => ARCH_RISCV64,
            Self::X86_64 => ARCH_X86_64,
            Self::Aarch64 => ARCH_AARCH64,
        }
    }
}
