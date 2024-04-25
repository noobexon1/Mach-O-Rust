// cputype
pub const CPU_ARCH_MASK: i32 = 0xff000000u32 as i32;   // Mask for architecture bits
pub const CPU_ARCH_ABI64: i32 = 0x01000000u32 as i32;  // 64-bit ABI

pub const CPU_TYPE_ANY: i32 = -1;
pub const CPU_TYPE_VAX: i32 = 1;
pub const CPU_TYPE_ROMP: i32 = 2;
pub const CPU_TYPE_NS32032: i32 = 4;
pub const CPU_TYPE_NS32332: i32 = 5;
pub const CPU_TYPE_MC680X0: i32 = 6;
pub const CPU_TYPE_X86: i32 = 7;
pub const CPU_TYPE_I386: i32 = CPU_TYPE_X86;
pub const CPU_TYPE_X86_64: i32 = CPU_TYPE_X86 | CPU_ARCH_ABI64;
pub const CPU_TYPE_MIPS: i32 = 8;
pub const CPU_TYPE_NS32352: i32 = 9;
pub const CPU_TYPE_MC98000: i32 = 10;
pub const CPU_TYPE_HPPA: i32 = 11;
pub const CPU_TYPE_ARM: i32 = 12;
pub const CPU_TYPE_ARM64: i32 = CPU_TYPE_ARM | CPU_ARCH_ABI64;
pub const CPU_ARCH_ABI64U_TYPE_MC88000: i32 = 13;
pub const CPU_TYPE_SPARC: i32 = 14;
pub const CPU_TYPE_I860_LE: i32 = 15;
pub const CPU_TYPE_I860_BE: i32 = 16;
pub const CPU_TYPE_RS6000: i32 = 17;
pub const CPU_TYPE_POWERPC: i32 = 18;
pub const CPU_TYPE_POWERPC64: i32 = CPU_TYPE_POWERPC | CPU_ARCH_ABI64;

// cpusubtype
pub const CPU_SUBTYPE_MASK: u32 = 0xff000000; /* mask for feature flags */
pub const CPU_SUBTYPE_LIB64: u32 = 0x80000000; /* 64 bit libraries */

pub const CPU_SUBTYPE_MULTIPLE: i32 = -1;
pub const CPU_SUBTYPE_LITTLE_ENDIAN: i32 = 0;
pub const CPU_SUBTYPE_BIG_ENDIAN: i32 = 1;

pub const CPU_THREADTYPE_NONE: i32 = 0;

pub const CPU_SUBTYPE_VAX_ALL: i32 = 0;
pub const CPU_SUBTYPE_VAX780: i32 = 1;
pub const CPU_SUBTYPE_VAX785: i32 = 2;
pub const CPU_SUBTYPE_VAX750: i32 = 3;
pub const CPU_SUBTYPE_VAX730: i32 = 4;
pub const CPU_SUBTYPE_UVAXI: i32 = 5;
pub const CPU_SUBTYPE_UVAXII: i32 = 6;
pub const CPU_SUBTYPE_VAX8200: i32 = 7;
pub const CPU_SUBTYPE_VAX8500: i32 = 8;
pub const CPU_SUBTYPE_VAX8600: i32 = 9;
pub const CPU_SUBTYPE_VAX8650: i32 = 10;
pub const CPU_SUBTYPE_VAX8800: i32 = 11;
pub const CPU_SUBTYPE_UVAXIII: i32 = 12;

pub const CPU_SUBTYPE_MC680X0_ALL: i32 = 1;
pub const CPU_SUBTYPE_MC68030: i32 = 1;
pub const CPU_SUBTYPE_MC68040: i32 = 2;
pub const CPU_SUBTYPE_MC68030_ONLY: i32 = 3;

pub const CPU_SUBTYPE_INTEL_MODEL_ALL: i32 = 0;
pub const CPU_SUBTYPE_X86_ALL: i32 = 3;
pub const CPU_SUBTYPE_X86_64_ALL: i32 = 3;
pub const CPU_SUBTYPE_X86_ARCH1: i32 = 4;
pub const CPU_SUBTYPE_X86_64_H: i32 = 8; /* Haswell feature subset */

pub const CPU_THREADTYPE_INTEL_HTT: i32 = 1;

pub const CPU_SUBTYPE_MIPS_ALL: i32 = 0;
pub const CPU_SUBTYPE_MIPS_R2300: i32 = 1;
pub const CPU_SUBTYPE_MIPS_R2600: i32 = 2;
pub const CPU_SUBTYPE_MIPS_R2800: i32 = 3;
pub const CPU_SUBTYPE_MIPS_R2000A: i32 = 4; // pmax
pub const CPU_SUBTYPE_MIPS_R2000: i32 = 5;
pub const CPU_SUBTYPE_MIPS_R3000A: i32 = 6; // 3max
pub const CPU_SUBTYPE_MIPS_R3000: i32 = 7;
pub const CPU_SUBTYPE_MC98000_ALL: i32 = 0;
pub const CPU_SUBTYPE_MC98601: i32 = 1;
pub const CPU_SUBTYPE_HPPA_ALL: i32 = 0;
pub const CPU_SUBTYPE_HPPA_7100: i32 = 0; // compat
pub const CPU_SUBTYPE_HPPA_7100LC: i32 = 1;
pub const CPU_SUBTYPE_MC88000_ALL: i32 = 0;
pub const CPU_SUBTYPE_MC88100: i32 = 1;
pub const CPU_SUBTYPE_MC88110: i32 = 2;
pub const CPU_SUBTYPE_SPARC_ALL: i32 = 0;
pub const CPU_SUBTYPE_I860_ALL: i32 = 0;
pub const CPU_SUBTYPE_I860_860: i32 = 1;

pub const CPU_SUBTYPE_POWERPC_ALL: i32 = 0;
pub const CPU_SUBTYPE_POWERPC_601: i32 = 1;
pub const CPU_SUBTYPE_POWERPC_602: i32 = 2;
pub const CPU_SUBTYPE_POWERPC_603: i32 = 3;
pub const CPU_SUBTYPE_POWERPC_603E: i32 = 4;
pub const CPU_SUBTYPE_POWERPC_603EV: i32 = 5;
pub const CPU_SUBTYPE_POWERPC_604: i32 = 6;
pub const CPU_SUBTYPE_POWERPC_604e: i32 = 7;
pub const CPU_SUBTYPE_POWERPC_620: i32 = 8;
pub const CPU_SUBTYPE_POWERPC_750: i32 = 9;
pub const CPU_SUBTYPE_POWERPC_7400: i32 = 10;
pub const CPU_SUBTYPE_POWERPC_7450: i32 = 11;
pub const CPU_SUBTYPE_POWERPC_970: i32 = 100;

pub const CPU_SUBTYPE_ARM_ALL: i32 = 0;
pub const CPU_SUBTYPE_ARM_V4T: i32 = 5;
pub const CPU_SUBTYPE_ARM_V6: i32 = 6;
pub const CPU_SUBTYPE_ARM_V5TEJ: i32 = 7;
pub const CPU_SUBTYPE_ARM_XSCALE: i32 = 8;
pub const CPU_SUBTYPE_ARM_V7: i32 = 9;
pub const CPU_SUBTYPE_ARM_V7F: i32 = 10; // Cortex A9
pub const CPU_SUBTYPE_ARM_V7S: i32 = 11; // Swift
pub const CPU_SUBTYPE_ARM_V7K: i32 = 12;
pub const CPU_SUBTYPE_ARM_V6M: i32 = 14; // Not meant to be run under xnu
pub const CPU_SUBTYPE_ARM_V7M: i32 = 15; // Not meant to be run under xnu
pub const CPU_SUBTYPE_ARM_V7EM: i32 = 16; // Not meant to be run under xnu
pub const CPU_SUBTYPE_ARM_V8: i32 = 13;
pub const CPU_SUBTYPE_ARM64_ALL: i32 = 0;
pub const CPU_SUBTYPE_ARM64_V8: i32 = 1;

pub const CPUFAMILY_UNKNOWN: u32 = 0;
pub const CPUFAMILY_POWERPC_G3: u32 = 0xcee41549;
pub const CPUFAMILY_POWERPC_G4: u32 = 0x77c184ae;
pub const CPUFAMILY_POWERPC_G5: u32 = 0xed76d8aa;
pub const CPUFAMILY_INTEL_6_13: u32 = 0xaa33392b;
pub const CPUFAMILY_INTEL_PENRYN: u32 = 0x78ea4fbc;
pub const CPUFAMILY_INTEL_NEHALEM: u32 = 0x6b5a4cd2;
pub const CPUFAMILY_INTEL_WESTMERE: u32 = 0x573b5eec;
pub const CPUFAMILY_INTEL_SANDYBRIDGE: u32 = 0x5490b78c;
pub const CPUFAMILY_INTEL_IVYBRIDGE: u32 = 0x1f65e835;
pub const CPUFAMILY_INTEL_HASWELL: u32 = 0x10b282dc;
pub const CPUFAMILY_INTEL_BROADWELL: u32 = 0x582ed09c;
pub const CPUFAMILY_INTEL_SKYLAKE: u32 = 0x37fc219f;
pub const CPUFAMILY_INTEL_KABYLAKE: u32 = 0x0f817246;
pub const CPUFAMILY_ARM_9: u32 = 0xe73283ae;
pub const CPUFAMILY_ARM_11: u32 = 0x8ff620d8;
pub const CPUFAMILY_ARM_XSCALE: u32 = 0x53b005f5;
pub const CPUFAMILY_ARM_12: u32 = 0xbd1b0ae9;
pub const CPUFAMILY_ARM_13: u32 = 0x0cc90e64;
pub const CPUFAMILY_ARM_14: u32 = 0x96077ef1;
pub const CPUFAMILY_ARM_15: u32 = 0xa8511bca;
pub const CPUFAMILY_ARM_SWIFT: u32 = 0x1e2d6381;
pub const CPUFAMILY_ARM_CYCLONE: u32 = 0x37a09642;
pub const CPUFAMILY_ARM_TYPHOON: u32 = 0x2c91a47e;
pub const CPUFAMILY_ARM_TWISTER: u32 = 0x92fb37c8;
pub const CPUFAMILY_ARM_HURRICANE: u32 = 0x67ceee93;

// filetype
pub const MH_OBJECT: u32 = 0x1;      // Relocatable object file
pub const MH_EXECUTE: u32 = 0x2;     // Demand paged executable file
pub const MH_FVMLIB: u32 = 0x3;      // Fixed VM shared library file
pub const MH_CORE: u32 = 0x4;        // Core file
pub const MH_PRELOAD: u32 = 0x5;     // Preloaded executable file
pub const MH_DYLIB: u32 = 0x6;       // Dynamically bound shared library
pub const MH_DYLINKER: u32 = 0x7;    // Dynamic link editor
pub const MH_BUNDLE: u32 = 0x8;      // Dynamically bound bundle file
pub const MH_DYLIB_STUB: u32 = 0x9;  // Shared library stub for static linking only, no section contents
pub const MH_DSYM: u32 = 0xa;        // Companion file with only debug sections
pub const MH_KEXT_BUNDLE: u32 = 0xb; // x86_64 kexts

// flags
pub const MH_NOUNDEFS: u32 = 0x1;                     // The object file has no undefined references
pub const MH_INCRLINK: u32 = 0x2;                     // Output of an incremental link against a base file
pub const MH_DYLDLINK: u32 = 0x4;                     // Input for the dynamic linker
pub const MH_BINDATLOAD: u32 = 0x8;                   // Undefined references are bound by the dynamic linker when loaded
pub const MH_PREBOUND: u32 = 0x10;                    // Dynamic undefined references are prebound
pub const MH_SPLIT_SEGS: u32 = 0x20;                  // Read-only and read-write segments are split
pub const MH_LAZY_INIT: u32 = 0x40;                   // Shared library init routine to be run lazily (obsolete)
pub const MH_TWOLEVEL: u32 = 0x80;                    // Using two-level namespace bindings
pub const MH_FORCE_FLAT: u32 = 0x100;                 // Forcing all images to use flat namespace bindings
pub const MH_NOMULTIDEFS: u32 = 0x200;                // Guarantees no multiple definitions of symbols in its sub-images
pub const MH_NOFIXPREBINDING: u32 = 0x400;            // Do not notify the prebinding agent about this executable
pub const MH_PREBINDABLE: u32 = 0x800;                // Not prebound but can have its prebinding redone
pub const MH_ALLMODSBOUND: u32 = 0x1000;              // Binds to all two-level namespace modules of its dependent libraries
pub const MH_SUBSECTIONS_VIA_SYMBOLS: u32 = 0x2000;   // Safe to divide up the sections into subsections via symbols
pub const MH_CANONICAL: u32 = 0x4000;                 // The binary has been canonicalized via the unprebind operation
pub const MH_WEAK_DEFINES: u32 = 0x8000;              // Contains external weak symbols
pub const MH_BINDS_TO_WEAK: u32 = 0x10000;            // Uses weak symbols
pub const MH_ALLOW_STACK_EXECUTION: u32 = 0x20000;    // Allows stack execution privilege
pub const MH_ROOT_SAFE: u32 = 0x40000;                // Safe for use in processes with uid zero
pub const MH_SETUID_SAFE: u32 = 0x80000;              // Safe for use in processes when issetugid() is true
pub const MH_NO_REEXPORTED_DYLIBS: u32 = 0x100000;    // Static linker does not need to examine dependent dylibs
pub const MH_PIE: u32 = 0x200000;                     // OS will load the main executable at a random address
pub const MH_DEAD_STRIPPABLE_DYLIB: u32 = 0x400000;   // When linking, the static linker will not create a load command if no symbols are referenced
pub const MH_HAS_TLV_DESCRIPTORS: u32 = 0x800000;     // Contains a section of type S_THREAD_LOCAL_VARIABLES
pub const MH_NO_HEAP_EXECUTION: u32 = 0x1000000;      // Runs the main executable with a non-executable heap
pub const MH_APP_EXTENSION_SAFE: u32 = 0x02000000;    // Linked for use in an application extension

pub const MH_MAGIC: u32 = 0xfeedface; // Big endian, 32 bit Mach-O
pub const MH_CIGAM: u32 = 0xcefaedfe; // Little endian, 32 bit Mach-O

pub const MH_MAGIC_64: u32 = 0xfeedfacf; // Big endian, 64 bit Mach-O
pub const MH_CIGAM_64: u32 = 0xcffaedfe; // Little endian, 64 bit Mach-O

// TODO: add cmd fields for all load commands from here: https://opensource.apple.com/source/xnu/xnu-4903.221.2/EXTERNAL_HEADERS/mach-o/loader.h.auto.html
