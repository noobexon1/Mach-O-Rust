use std::fs::File;
use std::io;
use std::io::Seek;

// cputype
const CPU_ARCH_MASK: i32 = 0xff000000u32 as i32;   // Mask for architecture bits
const CPU_ARCH_ABI64: i32 = 0x01000000u32 as i32;  // 64-bit ABI

const CPU_TYPE_ANY: i32 = -1;
const CPU_TYPE_VAX: i32 = 1;
const CPU_TYPE_ROMP: i32 = 2;
const CPU_TYPE_NS32032: i32 = 4;
const CPU_TYPE_NS32332: i32 = 5;
const CPU_TYPE_MC680X0: i32 = 6;
const CPU_TYPE_X86: i32 = 7;
const CPU_TYPE_I386: i32 = CPU_TYPE_X86;
const CPU_TYPE_X86_64: i32 = CPU_TYPE_X86 | CPU_ARCH_ABI64;
const CPU_TYPE_MIPS: i32 = 8;
const CPU_TYPE_NS32352: i32 = 9;
const CPU_TYPE_MC98000: i32 = 10;
const CPU_TYPE_HPPA: i32 = 11;
const CPU_TYPE_ARM: i32 = 12;
const CPU_TYPE_ARM64: i32 = CPU_TYPE_ARM | CPU_ARCH_ABI64;
const CPU_TYPE_MC88000: i32 = 13;
const CPU_TYPE_SPARC: i32 = 14;
const CPU_TYPE_I860_LE: i32 = 15;
const CPU_TYPE_I860_BE: i32 = 16;
const CPU_TYPE_RS6000: i32 = 17;
const CPU_TYPE_POWERPC: i32 = 18;
const CPU_TYPE_POWERPC64: i32 = CPU_TYPE_POWERPC | CPU_ARCH_ABI64;

// cpusubtype
// TODO: add the rest of the subtypes from here: https://opensource.apple.com/source/xnu/xnu-4570.41.2/osfmk/mach/machine.h.auto.html
const CPU_SUBTYPE_MASK: u32 = 0xff000000;   // Mask for feature flags
const CPU_SUBTYPE_LIB64: u32 = 0x80000000;  // 64-bit libraries

const CPU_SUBTYPE_MULTIPLE: i32 = -1;
const CPU_SUBTYPE_LITTLE_ENDIAN: i32 = 0;
const CPU_SUBTYPE_BIG_ENDIAN: i32 = 1;

const CPU_THREADTYPE_NONE: i32 = 0;
const CPU_SUBTYPE_VAX_ALL: i32 = 0;
const CPU_SUBTYPE_VAX780: i32 = 1;
const CPU_SUBTYPE_VAX785: i32 = 2;
const CPU_SUBTYPE_VAX750: i32 = 3;
const CPU_SUBTYPE_VAX730: i32 = 4;
const CPU_SUBTYPE_UVAXI: i32 = 5;
const CPU_SUBTYPE_UVAXII: i32 = 6;
const CPU_SUBTYPE_VAX8200: i32 = 7;
const CPU_SUBTYPE_VAX8500: i32 = 8;
const CPU_SUBTYPE_VAX8600: i32 = 9;
const CPU_SUBTYPE_VAX8650: i32 = 10;
const CPU_SUBTYPE_VAX8800: i32 = 11;
const CPU_SUBTYPE_UVAXIII: i32 = 12;

// filetype
const MH_OBJECT: u32 = 0x1;      // Relocatable object file
const MH_EXECUTE: u32 = 0x2;     // Demand paged executable file
const MH_FVMLIB: u32 = 0x3;      // Fixed VM shared library file
const MH_CORE: u32 = 0x4;        // Core file
const MH_PRELOAD: u32 = 0x5;     // Preloaded executable file
const MH_DYLIB: u32 = 0x6;       // Dynamically bound shared library
const MH_DYLINKER: u32 = 0x7;    // Dynamic link editor
const MH_BUNDLE: u32 = 0x8;      // Dynamically bound bundle file
const MH_DYLIB_STUB: u32 = 0x9;  // Shared library stub for static linking only, no section contents
const MH_DSYM: u32 = 0xa;        // Companion file with only debug sections
const MH_KEXT_BUNDLE: u32 = 0xb; // x86_64 kexts

// flags
const MH_NOUNDEFS: u32 = 0x1;                     // The object file has no undefined references
const MH_INCRLINK: u32 = 0x2;                     // Output of an incremental link against a base file
const MH_DYLDLINK: u32 = 0x4;                     // Input for the dynamic linker
const MH_BINDATLOAD: u32 = 0x8;                   // Undefined references are bound by the dynamic linker when loaded
const MH_PREBOUND: u32 = 0x10;                    // Dynamic undefined references are prebound
const MH_SPLIT_SEGS: u32 = 0x20;                  // Read-only and read-write segments are split
const MH_LAZY_INIT: u32 = 0x40;                   // Shared library init routine to be run lazily (obsolete)
const MH_TWOLEVEL: u32 = 0x80;                    // Using two-level namespace bindings
const MH_FORCE_FLAT: u32 = 0x100;                 // Forcing all images to use flat namespace bindings
const MH_NOMULTIDEFS: u32 = 0x200;                // Guarantees no multiple definitions of symbols in its sub-images
const MH_NOFIXPREBINDING: u32 = 0x400;            // Do not notify the prebinding agent about this executable
const MH_PREBINDABLE: u32 = 0x800;                // Not prebound but can have its prebinding redone
const MH_ALLMODSBOUND: u32 = 0x1000;              // Binds to all two-level namespace modules of its dependent libraries
const MH_SUBSECTIONS_VIA_SYMBOLS: u32 = 0x2000;   // Safe to divide up the sections into subsections via symbols
const MH_CANONICAL: u32 = 0x4000;                 // The binary has been canonicalized via the unprebind operation
const MH_WEAK_DEFINES: u32 = 0x8000;              // Contains external weak symbols
const MH_BINDS_TO_WEAK: u32 = 0x10000;            // Uses weak symbols
const MH_ALLOW_STACK_EXECUTION: u32 = 0x20000;    // Allows stack execution privilege
const MH_ROOT_SAFE: u32 = 0x40000;                // Safe for use in processes with uid zero
const MH_SETUID_SAFE: u32 = 0x80000;              // Safe for use in processes when issetugid() is true
const MH_NO_REEXPORTED_DYLIBS: u32 = 0x100000;    // Static linker does not need to examine dependent dylibs
const MH_PIE: u32 = 0x200000;                     // OS will load the main executable at a random address
const MH_DEAD_STRIPPABLE_DYLIB: u32 = 0x400000;   // When linking, the static linker will not create a load command if no symbols are referenced
const MH_HAS_TLV_DESCRIPTORS: u32 = 0x800000;     // Contains a section of type S_THREAD_LOCAL_VARIABLES
const MH_NO_HEAP_EXECUTION: u32 = 0x1000000;      // Runs the main executable with a non-executable heap
const MH_APP_EXTENSION_SAFE: u32 = 0x02000000;    // Linked for use in an application extension

// mach_header
#[repr(C)]
pub struct MachHeader {
    pub magic: u32,
    pub cputype: i32,
    pub cpusubtype: i32,
    pub filetype: u32,
    pub ncmds: u32,
    pub sizeofcmds: u32,
    pub flags: u32,
}

const MH_MAGIC: u32 = 0xfeedface; // Big endian, 32 bit Mach-O
const MH_CIGAM: u32 = 0xcefaedfe; // Little endian, 32 bit Mach-O

// mach_header_64
#[repr(C)]
pub struct MachHeader64 {
    pub magic: u32,
    pub cputype: i32,
    pub cpusubtype: i32,
    pub filetype: u32,
    pub ncmds: u32,
    pub sizeofcmds: u32,
    pub flags: u32,
    pub reserved: u32,
}

const MH_MAGIC_64: u32 = 0xfeedfacf; // Big endian, 64 bit Mach-O
const MH_CIGAM_64: u32 = 0xcffaedfe; // Little endian, 64 bit Mach-O

// Check that the file size is at least mach_header.size() bytes (=28).
// TODO: consider refactoring into a non boolean function... this requires change tto tests.
fn is_file_size_ok(file: &mut File) -> bool {
    let mach_o_header_size = 28;
    let file_metadata = file.metadata();
    match file_metadata {
        Ok(metadata) => {
            metadata.len() >= mach_o_header_size
        }
        Err(e) => {
            eprintln!("Failed to get file meta-data: {}", e);
            false
        }
    }
}

pub fn parse(file: &mut File) {
    if is_file_size_ok(file) {

    } else {
        eprintln!("File size is too small!");
    }

}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{self};
    use std::path::PathBuf;

    use super::*;

    // Helper function to open a test file
    fn open_test_file(filename: &str) -> io::Result<File> {
        let test_file_path = PathBuf::from(format!("test_files/{}", filename));
        File::open(test_file_path)
    }

    #[test]
    fn test_file_size_too_small() {
        let mut file = open_test_file("file_size_test_too_small.txt").expect("file should open!");
        assert!(!is_file_size_ok(&mut file));
    }

    #[test]
    fn test_file_size_ok() {
        let mut file = open_test_file("file_size_test_ok.txt").expect("file should open!");
        assert!(is_file_size_ok(&mut file));
    }

}