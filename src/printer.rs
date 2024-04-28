use prettytable::{row, Table};

use crate::constants::*;
use crate::header::{MachHeader, MachHeader32, MachHeader64};
use crate::load_commands::{EncryptionInfoCommand, LoadCommand, RoutinesCommand, SegmentCommand};

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
    table.add_row(row![ Fcc->"ncmds", Fyc->format!("0x{:x}", header.ncmds),  c->"number of load commands"]);
    table.add_row(row![ Fcc->"sizeofcmds", Fyc->format!("0x{:x}", header.sizeofcmds),  c->"size of all of the load commands in bytes"]);
    print_header_flags(header.flags, table);
    table.add_row(row![c=>"***", "***", "***"]);
}

fn print_header_64(header: &MachHeader64, table: &mut Table) {
    print_header_magic(header.magic, table);
    print_header_cputype(header.cputype, table);
    print_header_cpusubtype(header.cpusubtype, table);
    print_header_filetype(header.filetype, table);
    table.add_row(row![ Fcc->"ncmds", Fyc->format!("0x{:x}", header.ncmds),  c->"number of load commands"]);
    table.add_row(row![ Fcc->"sizeofcmds", Fyc->format!("0x{:x}", header.sizeofcmds),  c->"size of all of the load commands in bytes"]);
    print_header_flags(header.flags, table);
    table.add_row(row![ Fcc->"reserved", Fyc->format!("0x{:x}", header.reserved), c->"-"]);
}

fn print_header_magic(magic: u32, table: &mut Table) {
    let magic_string = match magic {
        MH_MAGIC => "MH_MAGIC (Big endian, 32 bit Mach-O)",
        MH_CIGAM => "MH_CIGAM (Little endian, 32 bit Mach-O)",
        MH_MAGIC_64 => "MH_MAGIC_64 (Big endian, 64 bit Mach-O)",
        MH_CIGAM_64 => "MH_CIGAM_64 (Little endian, 64 bit Mach-O)",
        _ => "Unrecognized mach-o magic!",
    };
    table.add_row(row![ Fcc->"magic", Fyc->format!("0x{:x}", magic),  c->magic_string]);
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
    table.add_row(row![ Fcc->"cputype", Fyc->format!("0x{:x}", cputype),  c->cputype_string]);
}

fn print_header_cpusubtype(cpusubtype: i32, table: &mut Table) {
    let cpusubtype_string = match cpusubtype {
        CPU_SUBTYPE_MULTIPLE => "CPU_SUBTYPE_MULTIPLE",
        CPU_SUBTYPE_LITTLE_ENDIAN => "CPU_SUBTYPE_LITTLE_ENDIAN",
        CPU_SUBTYPE_BIG_ENDIAN => "CPU_SUBTYPE_BIG_ENDIAN",
        _ => "Unrecogninzed cpusubtype!",
    };
    table.add_row(row![ Fcc->"cpusubtype", Fyc->format!("0x{:x}", cpusubtype),  c->cpusubtype_string]);
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
    table.add_row(row![ Fcc->"filetype", Fyc->format!("0x{:x}", filetype),  c->filetype_string]);
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
    table.add_row(row![ Fcc->"flags", Fyc->format!("0x{:x}", flags_combined),  c->format!("{}", decomposed_flags.join(" | "))]);
}

pub fn print_load_commands(load_commands: &Vec<LoadCommand>) {
    let mut table = Table::new();
    table.add_row(row![FBbc->"Load Commands", c->"-", c->"-"]);
    table.add_row(row![Bbbc=>"Field", "Value", "Info"]);

    for (index, load_command) in load_commands.iter().enumerate() {
        table.add_row(row![Fmbc->format!("Load Command #{}", index), c->"-", c->"-"]);
        match load_command {
            LoadCommand::SegmentCommand(command) => {
                match command {
                    SegmentCommand::SEG32(command) => {
                        print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
                    }
                    SegmentCommand::SEG64(command) => {
                        print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
                    }
                }
            }
            LoadCommand::DylibCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::SubFrameWorkCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::SubClientCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::SubUmbrellaCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::SubLibraryCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::PreboundDylibCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::DylinkerCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::ThreadCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::RoutinesCommand(command) => {
                match command {
                    RoutinesCommand::RTN32(command) => {
                        print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
                    }
                    RoutinesCommand::RTN64(command) => {
                        print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
                    }
                }
            }
            LoadCommand::SymtabCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::DynSymtabCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::TwoLevelHintsCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::PrebindCksumCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::UuidCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::RpathCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::LinkeditDataCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::EncryptionInfoCommand(command) => {
                match command {
                    EncryptionInfoCommand::ENI32(command) => {
                        print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
                    }
                    EncryptionInfoCommand::ENI64(command) => {
                        print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
                    }
                }
            }
            LoadCommand::VersionMinCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::BuildVersionCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::DyldInfoCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::LinkerOptionCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::SymsegCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::IdentCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::EntryPointCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::SourceVersionCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
            LoadCommand::NoteCommand(command) => {
                print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
            }
        }
        table.add_row(row![c=>"-", "-", "-"]);
    }
    table.printstd();
}

fn print_lc_cmd_and_cmdsize(cmd: u32, cmdsize: u32, table: &mut Table) {
    let cmd_string = match cmd {
        LC_SEGMENT => "LC_SEGMENT",
        LC_SYMTAB => "LC_SYMTAB",
        LC_SYMSEG => "LC_SYMSEG",
        LC_THREAD => "LC_THREAD",
        LC_UNIXTHREAD => "LC_UNIXTHREAD",
        LC_LOADFVMLIB => "LC_LOADFVMLIB",
        LC_IDFVMLIB =>"LC_IDFVMLIB",
        LC_IDENT => "LC_IDENT",
        LC_FVMFILE => "LC_FVMFILE",
        LC_PREPAGE => "LC_PREPAGE",
        LC_DYSYMTAB => "LC_DYSYMTAB",
        LC_LOAD_DYLIB => "LC_LOAD_DYLIB",
        LC_ID_DYLIB => "LC_ID_DYLIB",
        LC_LOAD_DYLINKER => "LC_LOAD_DYLINKER",
        LC_ID_DYLINKER => "LC_ID_DYLINKER",
        LC_PREBOUND_DYLIB => "LC_PREBOUND_DYLIB",
        LC_ROUTINES => "LC_ROUTINES",
        LC_SUB_FRAMEWORK => "LC_SUB_FRAMEWORK",
        LC_SUB_UMBRELLA => "LC_SUB_UMBRELLA",
        LC_SUB_CLIENT => "LC_SUB_CLIENT",
        LC_SUB_LIBRARY => "LC_SUB_LIBRARY",
        LC_TWOLEVEL_HINTS => "LC_TWOLEVEL_HINTS",
        LC_PREBIND_CKSUM => "LC_PREBIND_CKSUM",
        LC_LOAD_WEAK_DYLIB => "LC_LOAD_WEAK_DYLIB",
        LC_SEGMENT_64 => "LC_SEGMENT_64",
        LC_ROUTINES_64 => "LC_ROUTINES_64",
        LC_UUID => "LC_UUID",
        LC_RPATH => "LC_RPATH",
        LC_CODE_SIGNATURE => "LC_CODE_SIGNATURE",
        LC_SEGMENT_SPLIT_INFO => "LC_SEGMENT_SPLIT_INFO",
        LC_REEXPORT_DYLIB => "LC_REEXPORT_DYLIB",
        LC_LAZY_LOAD_DYLIB => "LC_LAZY_LOAD_DYLIB",
        LC_ENCRYPTION_INFO => "LC_ENCRYPTION_INFO",
        LC_DYLD_INFO => "LC_DYLD_INFO",
        LC_DYLD_INFO_ONLY => "LC_DYLD_INFO_ONLY",
        LC_LOAD_UPWARD_DYLIB => "LC_LOAD_UPWARD_DYLIB",
        LC_VERSION_MIN_MACOSX => "LC_VERSION_MIN_MACOSX",
        LC_VERSION_MIN_IPHONEOS => "LC_VERSION_MIN_IPHONEOS",
        LC_FUNCTION_STARTS => "LC_FUNCTION_STARTS",
        LC_DYLD_ENVIRONMENT => "LC_DYLD_ENVIRONMENT",
        LC_MAIN => "LC_MAIN",
        LC_DATA_IN_CODE => "LC_DATA_IN_CODE",
        LC_SOURCE_VERSION => "LC_SOURCE_VERSION",
        LC_DYLIB_CODE_SIGN_DRS => "LC_DYLIB_CODE_SIGN_DRS",
        LC_ENCRYPTION_INFO_64 => "LC_ENCRYPTION_INFO_64",
        LC_LINKER_OPTION => "LC_LINKER_OPTION",
        LC_LINKER_OPTIMIZATION_HINT => "LC_LINKER_OPTIMIZATION_HINT",
        LC_VERSION_MIN_TVOS => "LC_VERSION_MIN_TVOS",
        LC_VERSION_MIN_WATCHOS => "LC_VERSION_MIN_WATCHOS",
        LC_NOTE => "LC_NOTE",
        LC_BUILD_VERSION => "LC_BUILD_VERSION",
        _ => "",
    };

    table.add_row(row![ Fcc->"cmd", Fyc->format!("0x{:x} ({})", cmd, cmd_string),  c->"-"]);
    table.add_row(row![ Fcc->"cmdsize", Fyc->format!("0x{:x}", cmdsize),  c->"size of the load command in bytes"]);
}
