use std::env;
use std::str::FromStr;

use super::arch::*;
use super::constants::*;
use super::errors::XError;

#[derive(Args, Debug)]
pub(crate) struct QemuArgs {
    /// Build args.
    #[clap(flatten)]
    build: BuildArgs,
    /// Number of hart (SMP for Symmetrical Multiple Processor).
    #[clap(long)]
    smp: Option<u8>,
    /// Port for gdb to connect. If set, qemu will block and wait gdb to connect.
    #[clap(long)]
    gdb: Option<u16>,
}

#[derive(Clone, Args, Debug)]
pub(crate) struct BuildArgs {
    #[clap(flatten)]
    pub arch: ArchArg,
    /// Build as debug mode.
    #[clap(long)]
    pub debug: bool,
    #[clap(long)]
    pub features: Option<String>,
}

#[derive(Copy, Clone, Args, Debug)]
pub(crate) struct ArchArg {
    /// Build architecture, `riscv64` | `x86_64` | `aarch64`.
    #[clap(short, long)]
    pub arch: Option<Arch>,
}

impl BuildArgs {
    #[inline]
    fn arch(&self) -> Option<Arch> {
        self.arch.arch
    }
}

impl QemuArgs {
    pub fn qemu(&self) {
        let arch = match self.build.arch() {
            Some(arch) => arch.name().to_owned(),
            None => env::var("ARCH").unwrap(),
        };
        let smp = match self.smp {
            Some(smp) => smp.to_string(),
            None => env::var("SMP").unwrap(),
        };
        #[rustfmt::skip]
        cmd_lib::run_cmd!(
            qemu-system-$arch
            -machine virt
            -smp $smp
            -bios default
            -kernel zcore.bin
            -initrd riscv64.img
            -append LOG=warn
            -serial mon:stdio
            -display none
            -no-reboot
            -nographic
        );
    }
}
