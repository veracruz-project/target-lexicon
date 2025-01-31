// This file defines all the identifier enums and target-aware logic.

use crate::triple::{Endianness, PointerWidth, Triple};
use alloc::boxed::Box;
use alloc::string::String;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::str::FromStr;

/// The "architecture" field, which in some cases also specifies a specific
/// subarchitecture.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Architecture {
    Unknown,
    Arm(ArmArchitecture),
    AmdGcn,
    Aarch64(Aarch64Architecture),
    Asmjs,
    Hexagon,
    X86_32(X86_32Architecture),
    Mips32(Mips32Architecture),
    Mips64(Mips64Architecture),
    Msp430,
    Nvptx64,
    Powerpc,
    Powerpc64,
    Powerpc64le,
    Riscv32(Riscv32Architecture),
    Riscv64(Riscv64Architecture),
    S390x,
    Sparc,
    Sparc64,
    Sparcv9,
    Wasm32,
    Wasm64,
    X86_64,
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum ArmArchitecture {
    Arm, // Generic arm
    Armeb,
    Armv4,
    Armv4t,
    Armv5t,
    Armv5te,
    Armv5tej,
    Armv6,
    Armv6j,
    Armv6k,
    Armv6z,
    Armv6kz,
    Armv6t2,
    Armv6m,
    Armv7,
    Armv7a,
    Armv7ve,
    Armv7m,
    Armv7r,
    Armv7s,
    Armv8,
    Armv8a,
    Armv8_1a,
    Armv8_2a,
    Armv8_3a,
    Armv8_4a,
    Armv8_5a,
    Armv8mBase,
    Armv8mMain,
    Armv8r,

    Armebv7r,

    Thumbeb,
    Thumbv6m,
    Thumbv7a,
    Thumbv7em,
    Thumbv7m,
    Thumbv7neon,
    Thumbv8mBase,
    Thumbv8mMain,
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Aarch64Architecture {
    Aarch64,
    Aarch64be,
}

// #[non_exhaustive]
// #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
// #[allow(missing_docs)]
// pub enum ArmFpu {
//     Vfp,
//     Vfpv2,
//     Vfpv3,
//     Vfpv3Fp16,
//     Vfpv3Xd,
//     Vfpv3XdFp16,
//     Neon,
//     NeonVfpv3,
//     NeonVfpv4,
//     Vfpv4,
//     Vfpv4D16,
//     Fpv4SpD16,
//     Fpv5SpD16,
//     Fpv5D16,
//     FpArmv8,
//     NeonFpArmv8,
//     CryptoNeonFpArmv8,
// }

impl ArmArchitecture {
    /// Test if this architecture uses the Thumb instruction set.
    pub fn is_thumb(self) -> bool {
        match self {
            Self::Arm
            | Self::Armeb
            | Self::Armv4
            | Self::Armv4t
            | Self::Armv5t
            | Self::Armv5te
            | Self::Armv5tej
            | Self::Armv6
            | Self::Armv6j
            | Self::Armv6k
            | Self::Armv6z
            | Self::Armv6kz
            | Self::Armv6t2
            | Self::Armv6m
            | Self::Armv7
            | Self::Armv7a
            | Self::Armv7ve
            | Self::Armv7m
            | Self::Armv7r
            | Self::Armv7s
            | Self::Armv8
            | Self::Armv8a
            | Self::Armv8_1a
            | Self::Armv8_2a
            | Self::Armv8_3a
            | Self::Armv8_4a
            | Self::Armv8_5a
            | Self::Armv8mBase
            | Self::Armv8mMain
            | Self::Armv8r
            | Self::Armebv7r => false,
            Self::Thumbeb
            | Self::Thumbv6m
            | Self::Thumbv7a
            | Self::Thumbv7em
            | Self::Thumbv7m
            | Self::Thumbv7neon
            | Self::Thumbv8mBase
            | Self::Thumbv8mMain => true,
        }
    }

    // pub fn has_fpu(self) -> Result<&'static [ArmFpu], ()> {

    // }

    /// Return the pointer bit width of this target's architecture.
    pub fn pointer_width(self) -> PointerWidth {
        match self {
            Self::Arm
            | Self::Armeb
            | Self::Armv4
            | Self::Armv4t
            | Self::Armv5t
            | Self::Armv5te
            | Self::Armv5tej
            | Self::Armv6
            | Self::Armv6j
            | Self::Armv6k
            | Self::Armv6z
            | Self::Armv6kz
            | Self::Armv6t2
            | Self::Armv6m
            | Self::Armv7
            | Self::Armv7a
            | Self::Armv7ve
            | Self::Armv7m
            | Self::Armv7r
            | Self::Armv7s
            | Self::Armv8
            | Self::Armv8a
            | Self::Armv8_1a
            | Self::Armv8_2a
            | Self::Armv8_3a
            | Self::Armv8_4a
            | Self::Armv8_5a
            | Self::Armv8mBase
            | Self::Armv8mMain
            | Self::Armv8r
            | Self::Armebv7r
            | Self::Thumbeb
            | Self::Thumbv6m
            | Self::Thumbv7a
            | Self::Thumbv7em
            | Self::Thumbv7m
            | Self::Thumbv7neon
            | Self::Thumbv8mBase
            | Self::Thumbv8mMain => PointerWidth::U32,
        }
    }

    /// Return the endianness of this architecture.
    pub fn endianness(self) -> Endianness {
        match self {
            Self::Arm
            | Self::Armv4
            | Self::Armv4t
            | Self::Armv5t
            | Self::Armv5te
            | Self::Armv5tej
            | Self::Armv6
            | Self::Armv6j
            | Self::Armv6k
            | Self::Armv6z
            | Self::Armv6kz
            | Self::Armv6t2
            | Self::Armv6m
            | Self::Armv7
            | Self::Armv7a
            | Self::Armv7ve
            | Self::Armv7m
            | Self::Armv7r
            | Self::Armv7s
            | Self::Armv8
            | Self::Armv8a
            | Self::Armv8_1a
            | Self::Armv8_2a
            | Self::Armv8_3a
            | Self::Armv8_4a
            | Self::Armv8_5a
            | Self::Armv8mBase
            | Self::Armv8mMain
            | Self::Armv8r
            | Self::Thumbv6m
            | Self::Thumbv7a
            | Self::Thumbv7em
            | Self::Thumbv7m
            | Self::Thumbv7neon
            | Self::Thumbv8mBase
            | Self::Thumbv8mMain => Endianness::Little,
            Self::Armeb | Self::Armebv7r | Self::Thumbeb => Endianness::Big,
        }
    }
}

impl Aarch64Architecture {
    /// Test if this architecture uses the Thumb instruction set.
    pub fn is_thumb(self) -> bool {
        match self {
            Self::Aarch64 | Self::Aarch64be => false,
        }
    }

    // pub fn has_fpu(self) -> Result<&'static [ArmFpu], ()> {

    // }

    /// Return the pointer bit width of this target's architecture.
    pub fn pointer_width(self) -> PointerWidth {
        match self {
            Self::Aarch64 | Self::Aarch64be => PointerWidth::U64,
        }
    }

    /// Return the endianness of this architecture.
    pub fn endianness(self) -> Endianness {
        match self {
            Self::Aarch64 => Endianness::Little,
            Self::Aarch64be => Endianness::Big,
        }
    }
}

/// An enum for all 32-bit RISC-V architectures.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Riscv32Architecture {
    Riscv32,
    Riscv32i,
    Riscv32imac,
    Riscv32imc,
}

/// An enum for all 64-bit RISC-V architectures.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Riscv64Architecture {
    Riscv64,
    Riscv64gc,
    Riscv64imac,
}

/// An enum for all 32-bit x86 architectures.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum X86_32Architecture {
    I386,
    I586,
    I686,
}

/// An enum for all 32-bit MIPS architectures (not just "MIPS32").
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Mips32Architecture {
    Mips,
    Mipsel,
    Mipsisa32r6,
    Mipsisa32r6el,
}

/// An enum for all 64-bit MIPS architectures (not just "MIPS64").
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Mips64Architecture {
    Mips64,
    Mips64el,
    Mipsisa64r6,
    Mipsisa64r6el,
}

/// A string for a `Vendor::Custom` that can either be used in `const`
/// contexts or hold dynamic strings.
#[derive(Clone, Debug, Eq)]
pub enum CustomVendor {
    /// An owned `String`. This supports the general case.
    Owned(Box<String>),
    /// A static `str`, so that `CustomVendor` can be constructed in `const`
    /// contexts.
    Static(&'static str),
}

impl CustomVendor {
    /// Extracts a string slice.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Owned(s) => s,
            Self::Static(s) => s,
        }
    }
}

impl PartialEq for CustomVendor {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Hash for CustomVendor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

/// The "vendor" field, which in practice is little more than an arbitrary
/// modifier.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Vendor {
    Unknown,
    Amd,
    Apple,
    Experimental,
    Fortanix,
    Nvidia,
    Pc,
    Rumprun,
    Sun,
    Uwp,
    Wrs,

    /// A custom vendor. "Custom" in this context means that the vendor is
    /// not specifically recognized by upstream Autotools, LLVM, Rust, or other
    /// relevant authorities on triple naming. It's useful for people building
    /// and using locally patched toolchains.
    ///
    /// Outside of such patched environments, users of `target-lexicon` should
    /// treat `Custom` the same as `Unknown` and ignore the string.
    Custom(CustomVendor),
}

/// The "operating system" field, which sometimes implies an environment, and
/// sometimes isn't an actual operating system.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum OperatingSystem {
    Unknown,
    AmdHsa,
    Bitrig,
    Cloudabi,
    Cuda,
    Darwin,
    Dragonfly,
    Emscripten,
    Freebsd,
    Fuchsia,
    Haiku,
    Hermit,
    Illumos,
    Ios,
    L4re,
    Linux,
    MacOSX { major: u16, minor: u16, patch: u16 },
    Nebulet,
    Netbsd,
    None_,
    Openbsd,
    OpTee,
    Psp,
    Redox,
    Solaris,
    Uefi,
    VxWorks,
    Wasi,
    Windows,
}

/// The "environment" field, which specifies an ABI environment on top of the
/// operating system. In many configurations, this field is omitted, and the
/// environment is implied by the operating system.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Environment {
    Unknown,
    AmdGiz,
    Android,
    Androideabi,
    Eabi,
    Eabihf,
    Gnu,
    Gnuabi64,
    Gnueabi,
    Gnueabihf,
    Gnuspe,
    Gnux32,
    Macabi,
    Musl,
    Musleabi,
    Musleabihf,
    Muslabi64,
    Msvc,
    Kernel,
    Uclibc,
    Sgx,
    Softfloat,
    Spe,
    TrustZone
}

/// The "binary format" field, which is usually omitted, and the binary format
/// is implied by the other fields.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum BinaryFormat {
    Unknown,
    Elf,
    Coff,
    Macho,
    Wasm,
}

impl Architecture {
    /// Return the endianness of this architecture.
    pub fn endianness(self) -> Result<Endianness, ()> {
        match self {
            Self::Unknown => Err(()),
            Self::Arm(arm) => Ok(arm.endianness()),
            Self::Aarch64(aarch) => Ok(aarch.endianness()),
            Self::AmdGcn
            | Self::Asmjs
            | Self::Hexagon
            | Self::X86_32(_)
            | Self::Mips64(Mips64Architecture::Mips64el)
            | Self::Mips32(Mips32Architecture::Mipsel)
            | Self::Mips32(Mips32Architecture::Mipsisa32r6el)
            | Self::Mips64(Mips64Architecture::Mipsisa64r6el)
            | Self::Msp430
            | Self::Nvptx64
            | Self::Powerpc64le
            | Self::Riscv32(_)
            | Self::Riscv64(_)
            | Self::Wasm32
            | Self::Wasm64
            | Self::X86_64 => Ok(Endianness::Little),
            Self::Mips32(Mips32Architecture::Mips)
            | Self::Mips64(Mips64Architecture::Mips64)
            | Self::Mips32(Mips32Architecture::Mipsisa32r6)
            | Self::Mips64(Mips64Architecture::Mipsisa64r6)
            | Self::Powerpc
            | Self::Powerpc64
            | Self::S390x
            | Self::Sparc
            | Self::Sparc64
            | Self::Sparcv9 => Ok(Endianness::Big),
        }
    }

    /// Return the pointer bit width of this target's architecture.
    pub fn pointer_width(self) -> Result<PointerWidth, ()> {
        match self {
            Self::Unknown => Err(()),
            Self::Msp430 => Ok(PointerWidth::U16),
            Self::Arm(arm) => Ok(arm.pointer_width()),
            Self::Aarch64(aarch) => Ok(aarch.pointer_width()),
            Self::Asmjs
            | Self::Hexagon
            | Self::X86_32(_)
            | Self::Riscv32(_)
            | Self::Sparc
            | Self::Wasm32
            | Self::Mips32(_)
            | Self::Powerpc => Ok(PointerWidth::U32),
            Self::AmdGcn
            | Self::Powerpc64le
            | Self::Riscv64(_)
            | Self::X86_64
            | Self::Mips64(_)
            | Self::Nvptx64
            | Self::Powerpc64
            | Self::S390x
            | Self::Sparc64
            | Self::Sparcv9
            | Self::Wasm64 => Ok(PointerWidth::U64),
        }
    }
}

/// Return the binary format implied by this target triple, ignoring its
/// `binary_format` field.
pub(crate) fn default_binary_format(triple: &Triple) -> BinaryFormat {
    match triple.operating_system {
        OperatingSystem::None_ => match triple.environment {
            Environment::Eabi | Environment::Eabihf => BinaryFormat::Elf,
            _ => BinaryFormat::Unknown,
        },
        OperatingSystem::Darwin | OperatingSystem::Ios | OperatingSystem::MacOSX { .. } => {
            BinaryFormat::Macho
        }
        OperatingSystem::Windows => BinaryFormat::Coff,
        OperatingSystem::Nebulet
        | OperatingSystem::Emscripten
        | OperatingSystem::VxWorks
        | OperatingSystem::Wasi
        | OperatingSystem::Unknown => match triple.architecture {
            Architecture::Wasm32 | Architecture::Wasm64 => BinaryFormat::Wasm,
            _ => BinaryFormat::Unknown,
        },
        _ => BinaryFormat::Elf,
    }
}

impl fmt::Display for ArmArchitecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Self::Arm => "arm",
            Self::Armeb => "armeb",
            Self::Armv4 => "armv4",
            Self::Armv4t => "armv4t",
            Self::Armv5t => "armv5t",
            Self::Armv5te => "armv5te",
            Self::Armv5tej => "armv5tej",
            Self::Armv6 => "armv6",
            Self::Armv6j => "armv6j",
            Self::Armv6k => "armv6k",
            Self::Armv6z => "armv6z",
            Self::Armv6kz => "armv6kz",
            Self::Armv6t2 => "armv6t2",
            Self::Armv6m => "armv6m",
            Self::Armv7 => "armv7",
            Self::Armv7a => "armv7a",
            Self::Armv7ve => "armv7ve",
            Self::Armv7m => "armv7m",
            Self::Armv7r => "armv7r",
            Self::Armv7s => "armv7s",
            Self::Armv8 => "armv8",
            Self::Armv8a => "armv8a",
            Self::Armv8_1a => "armv8.1a",
            Self::Armv8_2a => "armv8.2a",
            Self::Armv8_3a => "armv8.3a",
            Self::Armv8_4a => "armv8.4a",
            Self::Armv8_5a => "armv8.5a",
            Self::Armv8mBase => "armv8m.base",
            Self::Armv8mMain => "armv8m.main",
            Self::Armv8r => "armv8r",
            Self::Thumbeb => "thumbeb",
            Self::Thumbv6m => "thumbv6m",
            Self::Thumbv7a => "thumbv7a",
            Self::Thumbv7em => "thumbv7em",
            Self::Thumbv7m => "thumbv7m",
            Self::Thumbv7neon => "thumbv7neon",
            Self::Thumbv8mBase => "thumbv8m.base",
            Self::Thumbv8mMain => "thumbv8m.main",
            Self::Armebv7r => "armebv7r",
        };
        f.write_str(s)
    }
}

impl fmt::Display for Aarch64Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Self::Aarch64 => "aarch64",
            Self::Aarch64be => "aarch64be",
        };
        f.write_str(s)
    }
}

impl fmt::Display for Riscv32Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Self::Riscv32 => "riscv32",
            Self::Riscv32i => "riscv32i",
            Self::Riscv32imac => "riscv32imac",
            Self::Riscv32imc => "riscv32imc",
        };
        f.write_str(s)
    }
}

impl fmt::Display for Riscv64Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Self::Riscv64 => "riscv64",
            Self::Riscv64gc => "riscv64gc",
            Self::Riscv64imac => "riscv64imac",
        };
        f.write_str(s)
    }
}

impl fmt::Display for X86_32Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Self::I386 => "i386",
            Self::I586 => "i586",
            Self::I686 => "i686",
        };
        f.write_str(s)
    }
}

impl fmt::Display for Mips32Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Self::Mips => "mips",
            Self::Mipsel => "mipsel",
            Self::Mipsisa32r6 => "mipsisa32r6",
            Self::Mipsisa32r6el => "mipsisa32r6el",
        };
        f.write_str(s)
    }
}

impl fmt::Display for Mips64Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Self::Mips64 => "mips64",
            Self::Mips64el => "mips64el",
            Self::Mipsisa64r6 => "mipsisa64r6",
            Self::Mipsisa64r6el => "mipsisa64r6el",
        };
        f.write_str(s)
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Arm(arm) => arm.fmt(f),
            Self::Aarch64(aarch) => aarch.fmt(f),
            Self::Unknown => f.write_str("unknown"),
            Self::AmdGcn => f.write_str("amdgcn"),
            Self::Asmjs => f.write_str("asmjs"),
            Self::Hexagon => f.write_str("hexagon"),
            Self::X86_32(x86_32) => x86_32.fmt(f),
            Self::Mips32(mips32) => mips32.fmt(f),
            Self::Mips64(mips64) => mips64.fmt(f),
            Self::Msp430 => f.write_str("msp430"),
            Self::Nvptx64 => f.write_str("nvptx64"),
            Self::Powerpc => f.write_str("powerpc"),
            Self::Powerpc64 => f.write_str("powerpc64"),
            Self::Powerpc64le => f.write_str("powerpc64le"),
            Self::Riscv32(riscv32) => riscv32.fmt(f),
            Self::Riscv64(riscv64) => riscv64.fmt(f),
            Self::S390x => f.write_str("s390x"),
            Self::Sparc => f.write_str("sparc"),
            Self::Sparc64 => f.write_str("sparc64"),
            Self::Sparcv9 => f.write_str("sparcv9"),
            Self::Wasm32 => f.write_str("wasm32"),
            Self::Wasm64 => f.write_str("wasm64"),
            Self::X86_64 => f.write_str("x86_64"),
        }
    }
}

impl FromStr for ArmArchitecture {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "arm" => Self::Arm,
            "armeb" => Self::Armeb,
            "armv4" => Self::Armv4,
            "armv4t" => Self::Armv4t,
            "armv5t" => Self::Armv5t,
            "armv5te" => Self::Armv5te,
            "armv5tej" => Self::Armv5tej,
            "armv6" => Self::Armv6,
            "armv6j" => Self::Armv6j,
            "armv6k" => Self::Armv6k,
            "armv6z" => Self::Armv6z,
            "armv6kz" => Self::Armv6kz,
            "armv6t2" => Self::Armv6t2,
            "armv6m" => Self::Armv6m,
            "armv7" => Self::Armv7,
            "armv7a" => Self::Armv7a,
            "armv7ve" => Self::Armv7ve,
            "armv7m" => Self::Armv7m,
            "armv7r" => Self::Armv7r,
            "armv7s" => Self::Armv7s,
            "armv8" => Self::Armv8,
            "armv8a" => Self::Armv8a,
            "armv8.1a" => Self::Armv8_1a,
            "armv8.2a" => Self::Armv8_2a,
            "armv8.3a" => Self::Armv8_3a,
            "armv8.4a" => Self::Armv8_4a,
            "armv8.5a" => Self::Armv8_5a,
            "armv8m.base" => Self::Armv8mBase,
            "armv8m.main" => Self::Armv8mMain,
            "armv8r" => Self::Armv8r,
            "thumbeb" => Self::Thumbeb,
            "thumbv6m" => Self::Thumbv6m,
            "thumbv7a" => Self::Thumbv7a,
            "thumbv7em" => Self::Thumbv7em,
            "thumbv7m" => Self::Thumbv7m,
            "thumbv7neon" => Self::Thumbv7neon,
            "thumbv8m.base" => Self::Thumbv8mBase,
            "thumbv8m.main" => Self::Thumbv8mMain,
            "armebv7r" => Self::Armebv7r,
            _ => return Err(()),
        })
    }
}

impl FromStr for Aarch64Architecture {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "aarch64" => Self::Aarch64,
            "arm64" => Self::Aarch64,
            "aarch64be" => Self::Aarch64be,
            _ => return Err(()),
        })
    }
}

impl FromStr for Riscv32Architecture {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "riscv32" => Self::Riscv32,
            "riscv32i" => Self::Riscv32i,
            "riscv32imac" => Self::Riscv32imac,
            "riscv32imc" => Self::Riscv32imc,
            _ => return Err(()),
        })
    }
}

impl FromStr for Riscv64Architecture {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "riscv64" => Self::Riscv64,
            "riscv64gc" => Self::Riscv64gc,
            "riscv64imac" => Self::Riscv64imac,
            _ => return Err(()),
        })
    }
}

impl FromStr for X86_32Architecture {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "i386" => Self::I386,
            "i586" => Self::I586,
            "i686" => Self::I686,
            _ => return Err(()),
        })
    }
}

impl FromStr for Mips32Architecture {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "mips" => Self::Mips,
            "mipsel" => Self::Mipsel,
            "mipsisa32r6" => Self::Mipsisa32r6,
            "mipsisa32r6el" => Self::Mipsisa32r6el,
            _ => return Err(()),
        })
    }
}

impl FromStr for Mips64Architecture {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "mips64" => Self::Mips64,
            "mips64el" => Self::Mips64el,
            "mipsisa64r6" => Self::Mipsisa64r6,
            "mipsisa64r6el" => Self::Mipsisa64r6el,
            _ => return Err(()),
        })
    }
}

impl FromStr for Architecture {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "unknown" => Self::Unknown,
            "amdgcn" => Self::AmdGcn,
            "asmjs" => Self::Asmjs,
            "hexagon" => Self::Hexagon,
            "msp430" => Self::Msp430,
            "nvptx64" => Self::Nvptx64,
            "powerpc" => Self::Powerpc,
            "powerpc64" => Self::Powerpc64,
            "powerpc64le" => Self::Powerpc64le,
            "s390x" => Self::S390x,
            "sparc" => Self::Sparc,
            "sparc64" => Self::Sparc64,
            "sparcv9" => Self::Sparcv9,
            "wasm32" => Self::Wasm32,
            "wasm64" => Self::Wasm64,
            "x86_64" => Self::X86_64,
            _ => {
                if let Ok(arm) = ArmArchitecture::from_str(s) {
                    Self::Arm(arm)
                } else if let Ok(aarch64) = Aarch64Architecture::from_str(s) {
                    Self::Aarch64(aarch64)
                } else if let Ok(riscv32) = Riscv32Architecture::from_str(s) {
                    Self::Riscv32(riscv32)
                } else if let Ok(riscv64) = Riscv64Architecture::from_str(s) {
                    Self::Riscv64(riscv64)
                } else if let Ok(x86_32) = X86_32Architecture::from_str(s) {
                    Self::X86_32(x86_32)
                } else if let Ok(mips32) = Mips32Architecture::from_str(s) {
                    Self::Mips32(mips32)
                } else if let Ok(mips64) = Mips64Architecture::from_str(s) {
                    Self::Mips64(mips64)
                } else {
                    return Err(());
                }
            }
        })
    }
}

impl fmt::Display for Vendor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Self::Unknown => "unknown",
            Self::Amd => "amd",
            Self::Apple => "apple",
            Self::Experimental => "experimental",
            Self::Fortanix => "fortanix",
            Self::Nvidia => "nvidia",
            Self::Pc => "pc",
            Self::Rumprun => "rumprun",
            Self::Sun => "sun",
            Self::Uwp => "uwp",
            Self::Wrs => "wrs",
            Self::Custom(ref name) => name.as_str(),
        };
        f.write_str(s)
    }
}

impl FromStr for Vendor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "unknown" => Self::Unknown,
            "amd" => Self::Amd,
            "apple" => Self::Apple,
            "experimental" => Self::Experimental,
            "fortanix" => Self::Fortanix,
            "nvidia" => Self::Nvidia,
            "pc" => Self::Pc,
            "rumprun" => Self::Rumprun,
            "sun" => Self::Sun,
            "uwp" => Self::Uwp,
            "wrs" => Self::Wrs,
            custom => {
                use alloc::borrow::ToOwned;

                // A custom vendor. Since triple syntax is so loosely defined,
                // be as conservative as we can to avoid potential ambiguities.
                // We err on the side of being too strict here, as we can
                // always relax it if needed.

                // Don't allow empty string names.
                if custom.is_empty() {
                    return Err(());
                }

                // Don't allow any other recognized name as a custom vendor,
                // since vendors can be omitted in some contexts.
                if Architecture::from_str(custom).is_ok()
                    || OperatingSystem::from_str(custom).is_ok()
                    || Environment::from_str(custom).is_ok()
                    || BinaryFormat::from_str(custom).is_ok()
                {
                    return Err(());
                }

                // Require the first character to be an ascii lowercase.
                if !custom.chars().next().unwrap().is_ascii_lowercase() {
                    return Err(());
                }

                // Restrict the set of characters permitted in a custom vendor.
                let has_restricted = custom.chars().any(|c: char| {
                    !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '.')
                });

                if has_restricted {
                    return Err(());
                }

                Self::Custom(CustomVendor::Owned(Box::new(custom.to_owned())))
            }
        })
    }
}

impl fmt::Display for OperatingSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Self::Unknown => "unknown",
            Self::AmdHsa => "amdhsa",
            Self::Bitrig => "bitrig",
            Self::Cloudabi => "cloudabi",
            Self::Cuda => "cuda",
            Self::Darwin => "darwin",
            Self::Dragonfly => "dragonfly",
            Self::Emscripten => "emscripten",
            Self::Freebsd => "freebsd",
            Self::Fuchsia => "fuchsia",
            Self::Haiku => "haiku",
            Self::Hermit => "hermit",
            Self::Illumos => "illumos",
            Self::Ios => "ios",
            Self::L4re => "l4re",
            Self::Linux => "linux",
            Self::MacOSX {
                major,
                minor,
                patch,
            } => {
                return write!(f, "macosx{}.{}.{}", major, minor, patch);
            }
            Self::Nebulet => "nebulet",
            Self::Netbsd => "netbsd",
            Self::None_ => "none",
            Self::Openbsd => "openbsd",
            Self::OpTee => "optee",
	    Self::Psp => "psp",
            Self::Redox => "redox",
            Self::Solaris => "solaris",
            Self::Uefi => "uefi",
            Self::VxWorks => "vxworks",
            Self::Wasi => "wasi",
            Self::Windows => "windows",
        };
        f.write_str(s)
    }
}

impl FromStr for OperatingSystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        // TODO also parse version number for darwin and ios OSes
        if s.starts_with("macosx") {
            // Parse operating system names like `macosx10.7.0`.
            let s = &s["macosx".len()..];
            let mut parts = s.split('.').map(|num| num.parse::<u16>());

            macro_rules! get_part {
                () => {
                    if let Some(Ok(part)) = parts.next() {
                        part
                    } else {
                        return Err(());
                    }
                };
            }

            let major = get_part!();
            let minor = get_part!();
            let patch = get_part!();

            if parts.next().is_some() {
                return Err(());
            }

            return Ok(Self::MacOSX {
                major,
                minor,
                patch,
            });
        }

        Ok(match s {
            "unknown" => Self::Unknown,
            "amdhsa" => Self::AmdHsa,
            "bitrig" => Self::Bitrig,
            "cloudabi" => Self::Cloudabi,
            "cuda" => Self::Cuda,
            "darwin" => Self::Darwin,
            "dragonfly" => Self::Dragonfly,
            "emscripten" => Self::Emscripten,
            "freebsd" => Self::Freebsd,
            "fuchsia" => Self::Fuchsia,
            "haiku" => Self::Haiku,
            "hermit" => Self::Hermit,
            "illumos" => Self::Illumos,
            "ios" => Self::Ios,
            "l4re" => Self::L4re,
            "linux" => Self::Linux,
            "nebulet" => Self::Nebulet,
            "netbsd" => Self::Netbsd,
            "none" => Self::None_,
            "openbsd" => Self::Openbsd,
	    "optee" => Self::OpTee,
            "psp" => Self::Psp,
            "redox" => Self::Redox,
            "solaris" => Self::Solaris,
            "uefi" => Self::Uefi,
            "vxworks" => Self::VxWorks,
            "wasi" => Self::Wasi,
            "windows" => Self::Windows,
            _ => return Err(()),
        })
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Self::Unknown => "unknown",
            Self::AmdGiz => "amdgiz",
            Self::Android => "android",
            Self::Androideabi => "androideabi",
            Self::Eabi => "eabi",
            Self::Eabihf => "eabihf",
            Self::Gnu => "gnu",
            Self::Gnuabi64 => "gnuabi64",
            Self::Gnueabi => "gnueabi",
            Self::Gnueabihf => "gnueabihf",
            Self::Gnuspe => "gnuspe",
            Self::Gnux32 => "gnux32",
            Self::Macabi => "macabi",
            Self::Musl => "musl",
            Self::Musleabi => "musleabi",
            Self::Musleabihf => "musleabihf",
            Self::Muslabi64 => "muslabi64",
            Self::Msvc => "msvc",
            Self::Kernel => "kernel",
            Self::Uclibc => "uclibc",
            Self::Sgx => "sgx",
            Self::Softfloat => "softfloat",
            Self::Spe => "spe",
	    Self::TrustZone => "trustzone"
        };
        f.write_str(s)
    }
}

impl FromStr for Environment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "unknown" => Self::Unknown,
            "amdgiz" => Self::AmdGiz,
            "android" => Self::Android,
            "androideabi" => Self::Androideabi,
            "eabi" => Self::Eabi,
            "eabihf" => Self::Eabihf,
            "gnu" => Self::Gnu,
            "gnuabi64" => Self::Gnuabi64,
            "gnueabi" => Self::Gnueabi,
            "gnueabihf" => Self::Gnueabihf,
            "gnuspe" => Self::Gnuspe,
            "gnux32" => Self::Gnux32,
            "macabi" => Self::Macabi,
            "musl" => Self::Musl,
            "musleabi" => Self::Musleabi,
            "musleabihf" => Self::Musleabihf,
            "muslabi64" => Self::Muslabi64,
            "msvc" => Self::Msvc,
            "kernel" => Self::Kernel,
            "uclibc" => Self::Uclibc,
            "sgx" => Self::Sgx,
            "softfloat" => Self::Softfloat,
            "spe" => Self::Spe,
	    "trustzone" => Self::TrustZone,
            _ => return Err(()),
        })
    }
}

impl fmt::Display for BinaryFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Self::Unknown => "unknown",
            Self::Elf => "elf",
            Self::Coff => "coff",
            Self::Macho => "macho",
            Self::Wasm => "wasm",
        };
        f.write_str(s)
    }
}

impl FromStr for BinaryFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "unknown" => Self::Unknown,
            "elf" => Self::Elf,
            "coff" => Self::Coff,
            "macho" => Self::Macho,
            "wasm" => Self::Wasm,
            _ => return Err(()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn roundtrip_known_triples() {
        // This list is constructed from:
        //  - targets emitted by "rustup target list"
        //  - targets emitted by "rustc +nightly --print target-list"
        //  - targets contributors have added
        let targets = [
            "aarch64-apple-ios",
            "aarch64-fuchsia",
            "aarch64-linux-android",
            "aarch64-pc-windows-msvc",
            "aarch64-unknown-cloudabi",
            "aarch64-unknown-freebsd",
            "aarch64-unknown-hermit",
            "aarch64-unknown-linux-gnu",
            "aarch64-unknown-linux-musl",
            "aarch64-unknown-netbsd",
            "aarch64-unknown-none",
            "aarch64-unknown-none-softfloat",
            "aarch64-unknown-openbsd",
            "aarch64-unknown-redox",
	    "aarch64-unknown-optee-trustzone",
            "aarch64-uwp-windows-msvc",
            "aarch64-wrs-vxworks",
            "amdgcn-amd-amdhsa",
            "amdgcn-amd-amdhsa-amdgiz",
            "armebv7r-none-eabi",
            "armebv7r-none-eabihf",
            "arm-linux-androideabi",
            "arm-unknown-linux-gnueabi",
            "arm-unknown-linux-gnueabihf",
            "arm-unknown-linux-musleabi",
            "arm-unknown-linux-musleabihf",
            "armv4t-unknown-linux-gnueabi",
            "armv5te-unknown-linux-gnueabi",
            "armv5te-unknown-linux-musleabi",
            "armv6-unknown-freebsd",
            "armv6-unknown-netbsd-eabihf",
            "armv7a-none-eabi",
            "armv7a-none-eabihf",
            "armv7-apple-ios",
            "armv7-linux-androideabi",
            "armv7r-none-eabi",
            "armv7r-none-eabihf",
            "armv7s-apple-ios",
            "armv7-unknown-cloudabi-eabihf",
            "armv7-unknown-freebsd",
            "armv7-unknown-linux-gnueabi",
            "armv7-unknown-linux-gnueabihf",
            "armv7-unknown-linux-musleabi",
            "armv7-unknown-linux-musleabihf",
            "armv7-unknown-netbsd-eabihf",
            "armv7-wrs-vxworks-eabihf",
            "asmjs-unknown-emscripten",
            "hexagon-unknown-linux-musl",
            "i386-apple-ios",
            "i586-pc-windows-msvc",
            "i586-unknown-linux-gnu",
            "i586-unknown-linux-musl",
            "i686-apple-darwin",
            "i686-linux-android",
            "i686-apple-macosx10.7.0",
            "i686-pc-windows-gnu",
            "i686-pc-windows-msvc",
            "i686-unknown-cloudabi",
            "i686-unknown-dragonfly",
            "i686-unknown-freebsd",
            "i686-unknown-haiku",
            "i686-unknown-linux-gnu",
            "i686-unknown-linux-musl",
            "i686-unknown-netbsd",
            "i686-unknown-openbsd",
            "i686-unknown-uefi",
            "i686-uwp-windows-gnu",
            "i686-uwp-windows-msvc",
            "i686-wrs-vxworks",
            "mips64el-unknown-linux-gnuabi64",
            "mips64el-unknown-linux-muslabi64",
            "mips64-unknown-linux-gnuabi64",
            "mips64-unknown-linux-muslabi64",
            "mipsel-sony-psp",
            "mipsel-unknown-linux-gnu",
            "mipsel-unknown-linux-musl",
            "mipsel-unknown-linux-uclibc",
            "mipsisa32r6el-unknown-linux-gnu",
            "mipsisa32r6-unknown-linux-gnu",
            "mipsisa64r6el-unknown-linux-gnuabi64",
            "mipsisa64r6-unknown-linux-gnuabi64",
            "mips-unknown-linux-gnu",
            "mips-unknown-linux-musl",
            "mips-unknown-linux-uclibc",
            "msp430-none-elf",
            "nvptx64-nvidia-cuda",
            "powerpc64le-unknown-linux-gnu",
            "powerpc64le-unknown-linux-musl",
            "powerpc64-unknown-freebsd",
            "powerpc64-unknown-linux-gnu",
            "powerpc64-unknown-linux-musl",
            "powerpc64-wrs-vxworks",
            "powerpc-unknown-linux-gnu",
            "powerpc-unknown-linux-gnuspe",
            "powerpc-unknown-linux-musl",
            "powerpc-unknown-netbsd",
            "powerpc-wrs-vxworks",
            "powerpc-wrs-vxworks-spe",
            "riscv32imac-unknown-none-elf",
            "riscv32imc-unknown-none-elf",
            "riscv32i-unknown-none-elf",
            "riscv64gc-unknown-linux-gnu",
            "riscv64gc-unknown-none-elf",
            "riscv64imac-unknown-none-elf",
            "s390x-unknown-linux-gnu",
            "sparc64-unknown-linux-gnu",
            "sparc64-unknown-netbsd",
            "sparc64-unknown-openbsd",
            "sparc-unknown-linux-gnu",
            "sparcv9-sun-solaris",
            "thumbv6m-none-eabi",
            "thumbv7a-pc-windows-msvc",
            "thumbv7a-uwp-windows-msvc",
            "thumbv7em-none-eabi",
            "thumbv7em-none-eabihf",
            "thumbv7m-none-eabi",
            "thumbv7neon-linux-androideabi",
            "thumbv7neon-unknown-linux-gnueabihf",
            "thumbv7neon-unknown-linux-musleabihf",
            "thumbv8m.base-none-eabi",
            "thumbv8m.main-none-eabi",
            "thumbv8m.main-none-eabihf",
            "wasm32-experimental-emscripten",
            "wasm32-unknown-emscripten",
            "wasm32-unknown-unknown",
            "wasm64-unknown-unknown",
            "wasm32-wasi",
            "wasm64-wasi",
            "x86_64-apple-darwin",
            "x86_64-apple-ios",
            "x86_64-fortanix-unknown-sgx",
            "x86_64-fuchsia",
            "x86_64-linux-android",
            "x86_64-linux-kernel",
            "x86_64-apple-macosx10.7.0",
            "x86_64-pc-solaris",
            "x86_64-pc-windows-gnu",
            "x86_64-pc-windows-msvc",
            "x86_64-rumprun-netbsd",
            "x86_64-sun-solaris",
            "x86_64-unknown-bitrig",
            "x86_64-unknown-cloudabi",
            "x86_64-unknown-dragonfly",
            "x86_64-unknown-freebsd",
            "x86_64-unknown-haiku",
            "x86_64-unknown-hermit",
            "x86_64-unknown-hermit-kernel",
            "x86_64-unknown-illumos",
            "x86_64-unknown-l4re-uclibc",
            "x86_64-unknown-linux-gnu",
            "x86_64-unknown-linux-gnux32",
            "x86_64-unknown-linux-musl",
            "x86_64-unknown-netbsd",
            "x86_64-unknown-openbsd",
            "x86_64-unknown-redox",
            "x86_64-unknown-uefi",
            "x86_64-uwp-windows-gnu",
            "x86_64-uwp-windows-msvc",
            "x86_64-wrs-vxworks",
        ];

        for target in targets.iter() {
            let t = Triple::from_str(target).expect("can't parse target");
            assert_ne!(t.architecture, Architecture::Unknown);
            assert_eq!(t.to_string(), *target);
        }
    }

    #[test]
    fn thumbv7em_none_eabihf() {
        let t = Triple::from_str("thumbv7em-none-eabihf").expect("can't parse target");
        assert_eq!(
            t.architecture,
            Architecture::Arm(ArmArchitecture::Thumbv7em)
        );
        assert_eq!(t.vendor, Vendor::Unknown);
        assert_eq!(t.operating_system, OperatingSystem::None_);
        assert_eq!(t.environment, Environment::Eabihf);
        assert_eq!(t.binary_format, BinaryFormat::Elf);
    }

    #[test]
    fn custom_vendors() {
        // Test various invalid cases.
        assert!(Triple::from_str("x86_64--linux").is_err());
        assert!(Triple::from_str("x86_64-42-linux").is_err());
        assert!(Triple::from_str("x86_64-__customvendor__-linux").is_err());
        assert!(Triple::from_str("x86_64-^-linux").is_err());
        assert!(Triple::from_str("x86_64- -linux").is_err());
        assert!(Triple::from_str("x86_64-CustomVendor-linux").is_err());
        assert!(Triple::from_str("x86_64-linux-linux").is_err());
        assert!(Triple::from_str("x86_64-x86_64-linux").is_err());
        assert!(Triple::from_str("x86_64-elf-linux").is_err());
        assert!(Triple::from_str("x86_64-gnu-linux").is_err());
        assert!(Triple::from_str("x86_64-linux-customvendor").is_err());
        assert!(Triple::from_str("customvendor").is_err());
        assert!(Triple::from_str("customvendor-x86_64").is_err());
        assert!(Triple::from_str("x86_64-").is_err());
        assert!(Triple::from_str("x86_64--").is_err());

        // Test various Unicode things.
        assert!(
            Triple::from_str("x86_64-𝓬𝓾𝓼𝓽𝓸𝓶𝓿𝓮𝓷𝓭𝓸𝓻-linux").is_err(),
            "unicode font hazard"
        );
        assert!(
            Triple::from_str("x86_64-ćúśtőḿvéńdőŕ-linux").is_err(),
            "diacritical mark stripping hazard"
        );
        assert!(
            Triple::from_str("x86_64-customvendοr-linux").is_err(),
            "homoglyph hazard"
        );
        assert!(Triple::from_str("x86_64-customvendor-linux").is_ok());
        assert!(
            Triple::from_str("x86_64-ﬃ-linux").is_err(),
            "normalization hazard"
        );
        assert!(Triple::from_str("x86_64-ffi-linux").is_ok());
        assert!(
            Triple::from_str("x86_64-custom‍vendor-linux").is_err(),
            "zero-width character hazard"
        );
        assert!(
            Triple::from_str("x86_64-﻿customvendor-linux").is_err(),
            "BOM hazard"
        );

        // Test some valid cases.
        let t = Triple::from_str("x86_64-customvendor-linux")
            .expect("can't parse target with custom vendor");
        assert_eq!(t.architecture, Architecture::X86_64);
        assert_eq!(
            t.vendor,
            Vendor::Custom(CustomVendor::Static("customvendor"))
        );
        assert_eq!(t.operating_system, OperatingSystem::Linux);
        assert_eq!(t.environment, Environment::Unknown);
        assert_eq!(t.binary_format, BinaryFormat::Elf);
        assert_eq!(t.to_string(), "x86_64-customvendor-linux");

        let t =
            Triple::from_str("x86_64-customvendor").expect("can't parse target with custom vendor");
        assert_eq!(t.architecture, Architecture::X86_64);
        assert_eq!(
            t.vendor,
            Vendor::Custom(CustomVendor::Static("customvendor"))
        );
        assert_eq!(t.operating_system, OperatingSystem::Unknown);
        assert_eq!(t.environment, Environment::Unknown);
        assert_eq!(t.binary_format, BinaryFormat::Unknown);

        assert_eq!(
            Triple::from_str("unknown-foo"),
            Ok(Triple {
                architecture: Architecture::Unknown,
                vendor: Vendor::Custom(CustomVendor::Static("foo")),
                operating_system: OperatingSystem::Unknown,
                environment: Environment::Unknown,
                binary_format: BinaryFormat::Unknown,
            })
        );
    }
}
