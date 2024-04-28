use prettytable::{row, Table};

use crate::constants::*;
use crate::header::{MachHeader, MachHeader32, MachHeader64};
use crate::load_commands::LoadCommand;

pub fn print_header(header: &MachHeader) {
    let mut table = Table::new();
    table.add_row(row![FBbc->"Header", c->"-", c->"-"]);
    table.add_row(row![Bbbc=>"Field", "Value", "Info"]);

    match header {
        MachHeader::MH32(header) => print_header_32(header, &mut table),
        MachHeader::MH64(header) => print_header_64(header, &mut table),
    }
    
    table.printstd();
}

fn print_header_32(header: &MachHeader32, table: &mut Table) {
    print_header_magic(header.magic, table);
    print_header_cputype(header.cputype, table);
    print_header_cpusubtype(header.cpusubtype, table);
    print_header_filetype(header.filetype, table);
    table.add_row(row![Frc->"ncmds", Fyc->format!("0x{:x}", header.ncmds), Fgc->"number of load commands"]);
    table.add_row(row![Frc->"sizeofcmds", Fyc->format!("0x{:x}", header.sizeofcmds), Fgc->"size of all of the load commands in bytes"]);
    print_header_flags(header.flags, table);
}

fn print_header_64(header: &MachHeader64, table: &mut Table) {
    print_header_magic(header.magic, table);
    print_header_cputype(header.cputype, table);
    print_header_cpusubtype(header.cpusubtype, table);
    print_header_filetype(header.filetype, table);
    table.add_row(row![Frc->"ncmds", Fyc->format!("0x{:x}", header.ncmds), Fgc->"number of load commands"]);
    table.add_row(row![Frc->"sizeofcmds", Fyc->format!("0x{:x}", header.sizeofcmds), Fgc->"size of all of the load commands in bytes"]);
    print_header_flags(header.flags, table);
    table.add_row(row![Frc->"reserved", Fyc->format!("0x{:x}", header.reserved), c->"-"]);
}

fn print_header_magic(magic: u32, table: &mut Table) {
    let magic_string = match magic {
        MH_MAGIC => "MH_MAGIC (Big endian, 32 bit Mach-O)",
        MH_CIGAM => "MH_CIGAM (Little endian, 32 bit Mach-O)",
        MH_MAGIC_64 => "MH_MAGIC_64 (Big endian, 64 bit Mach-O)",
        MH_CIGAM_64 => "MH_CIGAM_64 (Little endian, 64 bit Mach-O)",
        _ => "Unrecognized mach-o magic!",
    };
    table.add_row(row![Frc->"magic", Fyc->format!("0x{:x}", magic), Fgc->magic_string]);
}

fn print_header_cputype(cputype: i32, table: &mut Table) {
    let cputype_string = match cputype {
        CPU_TYPE_ANY => "CPU_TYPE_ANY",
        CPU_TYPE_VAX => "CPU_TYPE_VAX",
        CPU_TYPE_ROMP => "CPU_TYPE_ROMP",
        CPU_TYPE_NS32032 => "CPU_TYPE_NS32032",
        CPU_TYPE_NS32332 => "CPU_TYPE_NS32332",
        CPU_TYPE_MC680X0 => "CPU_TYPE_MC680X0",
        CPU_TYPE_X86 => "CPU_TYPE_X86",
        CPU_TYPE_I386 => "CPU_TYPE_I386",
        CPU_TYPE_X86_64 => "CPU_TYPE_X86_64",
        CPU_TYPE_MIPS => "CPU_TYPE_MIPS",
        CPU_TYPE_NS32352 => "CPU_TYPE_NS32352",
        CPU_TYPE_MC98000 => "CPU_TYPE_MC98000",
        CPU_TYPE_HPPA => "CPU_TYPE_HPPA",
        CPU_TYPE_ARM => "CPU_TYPE_ARM",
        CPU_TYPE_ARM64 => "CPU_TYPE_ARM64",
        CPU_ARCH_ABI64U_TYPE_MC88000 => "CPU_ARCH_ABI64U_TYPE_MC88000",
        CPU_TYPE_SPARC => "CPU_TYPE_SPARC",
        CPU_TYPE_I860_LE => "CPU_TYPE_I860_LE",
        CPU_TYPE_I860_BE => "CPU_TYPE_I860_BE",
        CPU_TYPE_RS6000 => "CPU_TYPE_RS6000",
        CPU_TYPE_POWERPC => "CPU_TYPE_POWERPC",
        CPU_TYPE_POWERPC64 => "CPU_TYPE_POWERPC64",
        _ => "Unrecognized cputype!",
    };
    table.add_row(row![Frc->"cputype", Fyc->format!("0x{:x}", cputype), Fgc->cputype_string]);
}

fn print_header_cpusubtype(cpusubtype: i32, table: &mut Table) {
    let cpusubtype_string = match cpusubtype {
        CPU_SUBTYPE_MULTIPLE => "CPU_SUBTYPE_MULTIPLE",
        CPU_SUBTYPE_LITTLE_ENDIAN => "CPU_SUBTYPE_LITTLE_ENDIAN",
        CPU_SUBTYPE_BIG_ENDIAN => "CPU_SUBTYPE_BIG_ENDIAN",
        _ => "Unrecogninzed cpusubtype!",
    };
    table.add_row(row![Frc->"cpusubtype", Fyc->format!("0x{:x}", cpusubtype), Fgc->cpusubtype_string]);
}

fn print_header_filetype(filetype: u32, table: &mut Table) {
    let filetype_string = match filetype {
        MH_OBJECT => "MH_OBJECT (Relocatable object file)",
        MH_EXECUTE => "MH_EXECUTE (Demand paged executable file)",
        MH_FVMLIB => "MH_FVMLIB (Fixed VM shared library file)",
        MH_CORE => "MH_CORE (Core file)",
        MH_PRELOAD => "MH_PRELOAD (Preloaded executable file)",
        MH_DYLIB => "MH_DYLIB (Dynamically bound shared library)",
        MH_DYLINKER => "MH_DYLINKER (Dynamic link editor)",
        MH_BUNDLE => "MH_BUNDLE (Dynamically bound bundle file)",
        MH_DYLIB_STUB => "MH_DYLIB_STUB (Shared library stub for static linking only, no section contents)",
        MH_DSYM => "MH_DSYM (Companion file with only debug sections)",
        MH_KEXT_BUNDLE => "MH_KEXT_BUNDLE (x86_64 kexts)",
        _ => "Unrecogninzed filetype!",
    };
    table.add_row(row![Frc->"filetype", Fyc->format!("0x{:x}", filetype), Fgc->filetype_string]);
}

fn print_header_flags(flags_combined: u32, table: &mut Table) {
    let flags_to_strings = [
        (MH_NOUNDEFS, "MH_NOUNDEFS"),
        (MH_INCRLINK, "MH_INCRLINK"),
        (MH_DYLDLINK, "MH_DYLDLINK"),
        (MH_BINDATLOAD, "MH_BINDATLOAD"),
        (MH_PREBOUND, "MH_PREBOUND"),
        (MH_SPLIT_SEGS, "MH_SPLIT_SEGS"),
        (MH_LAZY_INIT, "MH_LAZY_INIT"),
        (MH_TWOLEVEL, "MH_TWOLEVEL"),
        (MH_FORCE_FLAT, "MH_FORCE_FLAT"),
        (MH_NOMULTIDEFS, "MH_NOMULTIDEFS"),
        (MH_NOFIXPREBINDING, "MH_NOFIXPREBINDING"),
        (MH_PREBINDABLE, "MH_PREBINDABLE"),
        (MH_ALLMODSBOUND, "MH_ALLMODSBOUND"),
        (MH_SUBSECTIONS_VIA_SYMBOLS, "MH_SUBSECTIONS_VIA_SYMBOLS"),
        (MH_CANONICAL, "MH_CANONICAL"),
        (MH_WEAK_DEFINES, "MH_WEAK_DEFINES"),
        (MH_BINDS_TO_WEAK, "MH_BINDS_TO_WEAK"),
        (MH_ALLOW_STACK_EXECUTION, "MH_ALLOW_STACK_EXECUTION"),
        (MH_ROOT_SAFE, "MH_ROOT_SAFE"),
        (MH_SETUID_SAFE, "MH_SETUID_SAFE"),
        (MH_NO_REEXPORTED_DYLIBS, "MH_NO_REEXPORTED_DYLIBS"),
        (MH_PIE, "MH_PIE"),
        (MH_DEAD_STRIPPABLE_DYLIB, "MH_DEAD_STRIPPABLE_DYLIB"),
        (MH_HAS_TLV_DESCRIPTORS, "MH_HAS_TLV_DESCRIPTORS"),
        (MH_NO_HEAP_EXECUTION, "MH_NO_HEAP_EXECUTION"),
        (MH_APP_EXTENSION_SAFE, "MH_APP_EXTENSION_SAFE"),
    ];

    let mut decomposed_flags = Vec::new();

    for (flag, name) in flags_to_strings.iter() {
        if flags_combined & flag != 0 {
            decomposed_flags.push(*name);
        }
    }
    table.add_row(row![Frc->"flags", Fyc->format!("0x{:x}", flags_combined), Fgc->format!("{}", decomposed_flags.join(" | "))]);
}

pub fn print_load_commands(load_commands: &Vec<LoadCommand>) {
    println!("Load Commands");
}
