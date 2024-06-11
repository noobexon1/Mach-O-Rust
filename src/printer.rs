use prettytable::{row, Table};

use crate::constants::*;
use crate::header::{MachHeader, MachHeader32, MachHeader64};
use crate::load_commands::{BuildVersionCommand, DyldInfoCommand, DylibCommand, DynSymtabCommand, EncryptionInfoCommand, EncryptionInfoCommand32, EncryptionInfoCommand64, EntryPointCommand, LcStr, LinkeditDataCommand, LinkerOptionCommand, LoadCommand, NoteCommand, PrebindCksumCommand, PreboundDylibCommand, RoutinesCommand, RoutinesCommand32, RoutinesCommand64, Section, Section32, Section64, SegmentCommand, SegmentCommand32, SegmentCommand64, SourceVersionCommand, SymsegCommand, SymtabCommand, ThreadCommand, TwoLevelHintsCommand, UuidCommand, VersionMinCommand};

pub fn print_header(header: &MachHeader) {
    let mut table = Table::new();
    print_common_title("Header", &mut table);
    match header {
        MachHeader::MH32(header) => print_header_32(header, &mut table),
        MachHeader::MH64(header) => print_header_64(header, &mut table),
    }
    table.printstd();
}

fn print_header_32(header: &MachHeader32, table: &mut Table) {
    print_common_header_fields(header.magic, header.cputype, header.cpusubtype, header.filetype, header.ncmds, header.sizeofcmds, header.flags, None, table);
}

fn print_header_64(header: &MachHeader64, table: &mut Table) {
    print_common_header_fields(header.magic, header.cputype, header.cpusubtype, header.filetype, header.ncmds, header.sizeofcmds, header.flags, Some(header.reserved), table);
}

fn print_common_header_fields(magic: u32, cputype: i32, cpusubtype: i32, filetype: u32, ncmds: u32, sizeofcmds: u32, flags: u32, reserved: Option<u32>, table: &mut Table) {
    print_header_magic(magic, table);
    print_header_cputype(cputype, table);
    print_header_cpusubtype(cpusubtype, table);
    print_header_filetype(filetype, table);
    table.add_row(row![ Fcc->"ncmds", Fyc->format!("0x{:x}", ncmds),  c->"-"]);
    table.add_row(row![ Fcc->"sizeofcmds", Fyc->format!("0x{:x}", sizeofcmds),  c->"-"]);
    print_header_flags(flags, table);
    if let Some(r) = reserved {
        table.add_row(row![ Fcc->"reserved", Fyc->format!("0x{:x}",r), c->"-"]);
    }
}

fn print_header_magic(magic: u32, table: &mut Table) {
    let (magic_string, info) = match magic {
        MH_MAGIC => ("MH_MAGIC", "Big endian, 32 bit Mach-O"),
        MH_CIGAM => ("MH_CIGAM", "Little endian, 32 bit Mach-O"),
        MH_MAGIC_64 => ("MH_MAGIC_64", "Big endian, 64 bit Mach-O"),
        MH_CIGAM_64 => ("MH_CIGAM_64", "Little endian, 64 bit Mach-O"),
        _ => ("", "Unrecognized mach-o magic!"),
    };
    table.add_row(row![ Fcc->"magic", Fyc->format!("0x{:x}\n({})", magic, magic_string),  c->info]);
}

fn print_header_cputype(cputype: i32, table: &mut Table) {
    let (cputype_string, info) = match cputype {
        CPU_TYPE_ANY => ("CPU_TYPE_ANY", "ANY"),
        CPU_TYPE_VAX => ("CPU_TYPE_VAX", "VAX"),
        CPU_TYPE_ROMP => ("CPU_TYPE_ROMP", "ROMP"),
        CPU_TYPE_NS32032 => ("CPU_TYPE_NS32032", "NS32032"),
        CPU_TYPE_NS32332 => ("CPU_TYPE_NS32332", "NS32332"),
        CPU_TYPE_MC680X0 => ("CPU_TYPE_MC680X0", "MC680X0"),
        CPU_TYPE_X86 => ("CPU_TYPE_X86", "X86"),
        CPU_TYPE_X86_64 => ("CPU_TYPE_X86_64", "X86_64"),
        CPU_TYPE_MIPS => ("CPU_TYPE_MIPS", "MIPS"),
        CPU_TYPE_NS32352 => ("CPU_TYPE_NS32352", "NS32352"),
        CPU_TYPE_MC98000 => ("CPU_TYPE_MC98000", "MC98000"),
        CPU_TYPE_HPPA => ("CPU_TYPE_HPPA", "HPPA"),
        CPU_TYPE_ARM => ("CPU_TYPE_ARM", "ARM"),
        CPU_TYPE_ARM64 => ("CPU_TYPE_ARM64", "ARM64"),
        CPU_ARCH_ABI64U_TYPE_MC88000 => ("CPU_ARCH_ABI64U_TYPE_MC88000", "ABI64U_TYPE_MC88000"),
        CPU_TYPE_SPARC => ("CPU_TYPE_SPARC", "SPARC"),
        CPU_TYPE_I860_LE => ("CPU_TYPE_I860_LE", "I860_LE"),
        CPU_TYPE_I860_BE => ("CPU_TYPE_I860_BE", "I860_BE"),
        CPU_TYPE_RS6000 => ("CPU_TYPE_RS6000", "RS6000"),
        CPU_TYPE_POWERPC => ("CPU_TYPE_POWERPC", "POWERPC"),
        CPU_TYPE_POWERPC64 => ("CPU_TYPE_POWERPC64", "POWERPC64"),
        _ => ("Unrecognized cputype!", ""),
    };
    table.add_row(row![ Fcc->"cputype", Fyc->format!("0x{:x}\n({})", cputype, cputype_string),  c->info]);
}

fn print_header_cpusubtype(cpusubtype: i32, table: &mut Table) {
    let cpusubtype_string = match cpusubtype {
        CPU_SUBTYPE_MULTIPLE => "CPU_SUBTYPE_MULTIPLE",
        CPU_SUBTYPE_LITTLE_ENDIAN => "CPU_SUBTYPE_LITTLE_ENDIAN",
        CPU_SUBTYPE_BIG_ENDIAN => "CPU_SUBTYPE_BIG_ENDIAN",
        _ => "Unrecogninzed cpusubtype!",
    };
    table.add_row(row![ Fcc->"cpusubtype", Fyc->format!("0x{:x}\n({})", cpusubtype, cpusubtype_string),  c->"-"]);
}

fn print_header_filetype(filetype: u32, table: &mut Table) {
    let (filetype_string, info) = match filetype {
        MH_OBJECT => ("MH_OBJECT", "Relocatable object file (Intermediate format.\nContains symbol and relocation tables.\nLinker uses it to create executable or libraries by\n resolving symbols and adjusting addresses)"),
        MH_EXECUTE => ("MH_EXECUTE", "Demand paged executable file (The \"demand paged\" refers to\nhow the OS manages memory for executables.\nIt loads memory pages into physical memory\nonly when they are needed)"),
        MH_FVMLIB => ("MH_FVMLIB", "Fixed VM shared library file (The \"fixed virtual\" refers to how\nthe shared library is loaded into memory.\nIt means the library will always\noccupy the same address space when loaded)"),
        MH_CORE => ("MH_CORE", "Core file (Refers to a core dump file.\nUsed primarily for debugging.\nIt\'s essentially a snapshot of a program's memory\nat a specific time. This also includes CPU registers and\n other state info)"),
        MH_PRELOAD => ("MH_PRELOAD", "Preloaded executable file\n(Designed to be loaded into memory before it is actually executed.\nUsed in strict execution environments)"),
        MH_DYLIB => ("MH_DYLIB", "Dynamically bound shared library (A shared library that can be loaded\ninto the memory space of 1 or more running processes.\n Linked by the dynamic linker)"),
        MH_DYLINKER => ("MH_DYLINKER", "Dynamic link editor (A file representing a dynamic linker)"),
        MH_BUNDLE => ("MH_BUNDLE", "Dynamically bound bundle file (A bundle represents a directory with executable code\nand its resources. basically its an app or plugin)"),
        MH_DYLIB_STUB => ("MH_DYLIB_STUB", "Shared library stub for static linking only, no section contents (Contains only symbol\ninformation necessary for linking but no actual executable code)"),
        MH_DSYM => ("MH_DSYM", "Companion file with only debug sections"),
        MH_KEXT_BUNDLE => ("MH_KEXT_BUNDLE", "x86_64 kexts"),
        _ => ("", "Unrecogninzed filetype!"),
    };
    table.add_row(row![ Fcc->"filetype", Fyc->format!("0x{:x}\n({})", filetype, filetype_string),  c->info]);
}

fn print_header_flags(flags_combined: u32, table: &mut Table) {
    let flags_to_strings = [
        (MH_NOUNDEFS, "MH_NOUNDEFS", "The object file has no undefined references"),
        (MH_INCRLINK, "MH_INCRLINK", "The object file is the output of an incremental link\nagainst a base file and can't be link edited again"),
        (MH_DYLDLINK, "MH_DYLDLINK", "The object file is input for the dynamic linker and\ncan't be statically link edited again"),
        (MH_BINDATLOAD, "MH_BINDATLOAD", "The object file's undefined references are\nbound by the dynamic linker when loaded"),
        (MH_PREBOUND, "MH_PREBOUND", "The file has its dynamic undefined references prebound"),
        (MH_SPLIT_SEGS, "MH_SPLIT_SEGS", "The file has its read-only and read-write segments split"),
        (MH_LAZY_INIT, "MH_LAZY_INIT", "The shared library init routine is to be run lazily\nvia catching memory faults to its writeable segments"),
        (MH_TWOLEVEL, "MH_TWOLEVEL", "The image is using two-level name space bindings"),
        (MH_FORCE_FLAT, "MH_FORCE_FLAT", "The executable is forcing all images to use flat name space bindings"),
        (MH_NOMULTIDEFS, "MH_NOMULTIDEFS", "This umbrella guarantees no multiple definitions\nof symbols in its sub-images so the two-level namespace\nhints can always be used"),
        (MH_NOFIXPREBINDING, "MH_NOFIXPREBINDING", "Do not have dyld notify the prebinding agent about this executable"),
        (MH_PREBINDABLE, "MH_PREBINDABLE", "The binary is not prebound but can have its prebinding redone.\nOnly used when MH_PREBOUND is not set"),
        (MH_ALLMODSBOUND, "MH_ALLMODSBOUND", "Indicates that this binary binds to all two-level namespace\nmodules of its dependent libraries"),
        (MH_SUBSECTIONS_VIA_SYMBOLS, "MH_SUBSECTIONS_VIA_SYMBOLS", "Safe to divide up the sections into\nsub-sections via symbols for dead code stripping"),
        (MH_CANONICAL, "MH_CANONICAL", "The binary has been canonicalized via the un-prebind operation"),
        (MH_WEAK_DEFINES, "MH_WEAK_DEFINES", "The final linked image contains external weak symbols"),
        (MH_BINDS_TO_WEAK, "MH_BINDS_TO_WEAK", "The final linked image uses weak symbols"),
        (MH_ALLOW_STACK_EXECUTION, "MH_ALLOW_STACK_EXECUTION", "When this bit is set, all stacks\nin the task will be given stack execution privilege.\nOnly used in MH_EXECUTE filetypes"),
        (MH_ROOT_SAFE, "MH_ROOT_SAFE", "When this bit is set, the binary declares it is safe for\nuse in processes with uid zero"),
        (MH_SETUID_SAFE, "MH_SETUID_SAFE", "When this bit is set, the binary declares it is safe\nfor use in processes when issetugid() is true"),
        (MH_NO_REEXPORTED_DYLIBS, "MH_NO_REEXPORTED_DYLIBS", "When this bit is set on a dylib, the static\nlinker does not need to examine dependent dylibs to see if any are re-exported"),
        (MH_PIE, "MH_PIE", "When this bit is set, the OS will load the main executable at a random address"),
        (MH_DEAD_STRIPPABLE_DYLIB, "MH_DEAD_STRIPPABLE_DYLIB", "Only for use on dylibs. When linking\nagainst a dylib that has this bit set, the static linker\nwill automatically not create a load command to the dylib\nif no symbols are being referenced from the dylib"),
        (MH_HAS_TLV_DESCRIPTORS, "MH_HAS_TLV_DESCRIPTORS", "Contains a section of type S_THREAD_LOCAL_VARIABLES"),
        (MH_NO_HEAP_EXECUTION, "MH_NO_HEAP_EXECUTION", "When this bit is set, the OS will\nrun the main executable with a non-executable\nheap even on platforms (e.g. i386) that don't require it.\nOnly used in MH_EXECUTE filetypes"),
        (MH_APP_EXTENSION_SAFE, "MH_APP_EXTENSION_SAFE", "The code was linked for use in an\napplication extension"),
    ];
    let mut decomposed_flags = Vec::new();
    let mut flags_table = Table::new();
    for (flag, name, description) in flags_to_strings.iter() {
        if flags_combined & flag != 0 {
            decomposed_flags.push(*name);
            flags_table.add_row(row![*name, *description]);
        }
    }
    table.add_row(row![Fcc->"flags", Fyc->format!("0x{:x}\n({})", flags_combined, format!("{}", decomposed_flags.join(" | "))), c->flags_table]);
}

pub fn print_load_commands(load_commands: &(Vec<LoadCommand>, Vec<Vec<Section>>, Vec<LcStr>)) {
    let mut table = Table::new();

    print_common_title("Load Commands", &mut table);
    for (index, load_command) in load_commands.0.iter().enumerate() {
        table.add_row(row![Fmbc->format!("Load Command #{}", index), c->"-", c->"-"]);
        match load_command {
            LoadCommand::SegmentCommand(command) => {
                match command {
                    SegmentCommand::SEG32(command) => print_segment_command32(command, &mut table),
                    SegmentCommand::SEG64(command) => print_segment_command64(command, &mut table),
                }
                print_sections_for_segment(&load_commands.1[index], &mut table);
            }
            LoadCommand::DylibCommand(command) => unsafe { print_dylib_command(command, String::from_utf8(load_commands.2[index].clone()).unwrap() ,&mut table) },
            LoadCommand::SubFrameWorkCommand(command) => print_common_lcstr(command.cmd, command.cmdsize, "umbrella", String::from_utf8(load_commands.2[index].clone()).unwrap(), &mut table),
            LoadCommand::SubClientCommand(command) => print_common_lcstr(command.cmd, command.cmdsize, "client", String::from_utf8(load_commands.2[index].clone()).unwrap(), &mut table),
            LoadCommand::SubUmbrellaCommand(command) => print_common_lcstr(command.cmd, command.cmdsize, "sub_umbrella", String::from_utf8(load_commands.2[index].clone()).unwrap(), &mut table),
            LoadCommand::SubLibraryCommand(command) => print_common_lcstr(command.cmd, command.cmdsize, "sub_library", String::from_utf8(load_commands.2[index].clone()).unwrap(), &mut table),
            LoadCommand::PreboundDylibCommand(command) => print_prebound_dylib_command(command, &mut table),
            LoadCommand::DylinkerCommand(command) => print_common_lcstr(command.cmd, command.cmdsize, "name", String::from_utf8(load_commands.2[index].clone()).unwrap(), &mut table),
            LoadCommand::ThreadCommand(command) => print_thread_command(command, &mut table),
            LoadCommand::RoutinesCommand(command) => {
                match command {
                    RoutinesCommand::RTN32(command) => print_routines_command_32(command, &mut table),
                    RoutinesCommand::RTN64(command) => print_routines_command_64(command, &mut table),
                }
            }
            LoadCommand::SymtabCommand(command) => print_symtab_command(command, &mut table),
            LoadCommand::DynSymtabCommand(command) => print_dynsymtab_command(command, &mut table),
            LoadCommand::TwoLevelHintsCommand(command) => print_two_level_hints_command(command, &mut table),
            LoadCommand::PrebindCksumCommand(command) => print_prebind_cksum_command(command, &mut table),
            LoadCommand::UuidCommand(command) => print_uuid_command(command, &mut table),
            LoadCommand::RpathCommand(command) => print_common_lcstr(command.cmd, command.cmdsize, "path", String::from_utf8(load_commands.2[index].clone()).unwrap(), &mut table),
            LoadCommand::LinkeditDataCommand(command) => print_linkedit_data_command(command, &mut table),
            LoadCommand::EncryptionInfoCommand(command) => {
                match command {
                    EncryptionInfoCommand::ENI32(command) => print_encryption_info_command_32(command, &mut table),
                    EncryptionInfoCommand::ENI64(command) => print_encryption_info_command_64(command, &mut table),
                }
            }
            LoadCommand::VersionMinCommand(command) => print_version_min_command(command, &mut table),
            LoadCommand::BuildVersionCommand(command) => print_build_version_command(command, &mut table),
            LoadCommand::DyldInfoCommand(command) => print_dyld_info_command(command, &mut table),
            LoadCommand::LinkerOptionCommand(command) => print_linker_options_command(command, &mut table),
            LoadCommand::SymsegCommand(command) => print_symseg_command(command, &mut table),
            LoadCommand::IdentCommand(command) => print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table),
            LoadCommand::EntryPointCommand(command) => print_entry_point_command(command, &mut table),
            LoadCommand::SourceVersionCommand(command) => print_source_version_command(command, &mut table),
            LoadCommand::NoteCommand(command) => print_note_command(command, &mut table),
        }
        table.add_row(row![c=>"-", "-", "-"]);
    }
    table.printstd();
}

fn print_segment_command32(command: &SegmentCommand32, table: &mut Table) {
    print_common_segment_fields(command.cmd, command.cmdsize, &command.segname, command.vmaddr as u64, command.vmsize as u64, command.fileoff as u64, command.filesize as u64, command.nsects, command.maxprot, command.initprot, command.flags, table);
}

fn print_segment_command64(command: &SegmentCommand64, table: &mut Table) {
    print_common_segment_fields(command.cmd, command.cmdsize, &command.segname, command.vmaddr, command.vmsize, command.fileoff, command.filesize, command.nsects, command.maxprot, command.initprot, command.flags, table);
}

fn print_common_segment_fields(cmd: u32, cmdsize: u32, segname: &[u8], vmaddr: u64, vmsize: u64, fileoff: u64, filesize: u64, nsects: u32, maxprot: i32, initprot: i32,flags: u32, table: &mut Table) {
    print_lc_cmd_and_cmdsize(cmd, cmdsize, table);
    print_segname_or_sectname_bytes_array(segname, table);
    table.add_row(row![Fcc->"vmaddr", Fyc->format!("0x{:x}", vmaddr), c->"-"]);
    table.add_row(row![Fcc->"vmsize", Fyc->format!("0x{:x}", vmsize), c->"-"]);
    table.add_row(row![Fcc->"fileoff", Fyc->format!("0x{:x}", fileoff), c->"-"]);
    table.add_row(row![Fcc->"filesize", Fyc->format!("0x{:x}", filesize), c->"-"]);
    print_segment_maxprot_or_initprot(maxprot, table);
    print_segment_maxprot_or_initprot(initprot, table);
    table.add_row(row![Fcc->"nsects", Fyc->format!("0x{:x}", nsects), c->"-"]);
    print_segment_flags(flags, table);
}

fn print_segname_or_sectname_bytes_array(bytes: &[u8], table: &mut Table) {
    let mut result = String::from("[");
    for (index, &byte) in bytes.iter().enumerate() {
        if index % 4 == 0 && index != 0 {
            result.push_str("\n ");
        }
        result.push_str(&format!("0x{:02X}", byte));
        if index < bytes.len() - 1 {
            result.push_str(", ");
        }
    }
    result.push(']');
    let as_string =  String::from_utf8(bytes.to_vec()).unwrap();
    table.add_row(row![ Fcc->"segname", Fyc->format!("{}", result),  c->as_string]);
}

fn print_segment_maxprot_or_initprot(prot: i32, table: &mut Table) {
    let flags_to_strings = [
        (VM_PROT_READ, "VM_PROT_READ", "r"),
        (VM_PROT_WRITE, "VM_PROT_WRITE", "w"),
        (VM_PROT_EXECUTE, "VM_PROT_EXECUTE", "x"),
    ];
    let mut decomposed_flags = Vec::new();
    let mut decomposed_descriptions = Vec::new();
    for (flag, name, description) in flags_to_strings.iter() {
        if prot & flag != 0 {
            decomposed_flags.push(*name);
            decomposed_descriptions.push(*description);
        } else {
            decomposed_descriptions.push("_");
        }
    }
    if decomposed_flags.is_empty() {
        decomposed_flags.push("VM_PROT_NONE")
    }
    table.add_row(row![ Fcc->"maxprot", Fyc->format!("{}\n({})", prot, decomposed_flags.join(" | ")),  c->format!("{}", decomposed_descriptions.join(""))]);
}

fn print_segment_flags(flags_combined: u32, table: &mut Table) {
    let flags_to_strings = [
        (SG_HIGHVM, "SG_HIGHVM", "the file contents for this segment is for\nthe high part of the VM space, the low part\nis zero filled (for stacks in core files)"),
        (SG_FVMLIB, "SG_FVMLIB", "this segment is the VM that is allocated by\na fixed VM library, for overlap checking in\nthe link editor"),
        (SG_NORELOC, "SG_NORELOC", "this segment has nothing that was relocated\nin it and nothing relocated to it, that is\nit maybe safely replaced without relocation"),
        (SG_PROTECTED_VERSION_1, "SG_PROTECTED_VERSION_1", "This segment is protected.  If the\nsegment starts at file offset 0, the\nfirst page of the segment is not\nprotected.  All other pages of the\nsegment are protected."),
    ];
    let mut decomposed_flags = Vec::new();
    let mut flags_table = Table::new();
    for (flag, name, description) in flags_to_strings.iter() {
        if flags_combined & flag != 0 {
            decomposed_flags.push(*name);
            flags_table.add_row(row![*name, *description]);
        }
    }
    if decomposed_flags.is_empty() {
        table.add_row(row![ Fcc->"flags", Fyc->format!("0x{:x}", flags_combined), c->"-"]);
    } else {
        table.add_row(row![Fcc->"flags", Fyc->format!("0x{:x}\n({})", flags_combined, format!("{}", decomposed_flags.join(" | "))), c->flags_table]);
    }
}

fn print_sections_for_segment(sections: &Vec<Section>, table: &mut Table) {
    for (index, section) in sections.iter().enumerate() {
        table.add_row(row![Fgbc->format!("Section #{}", index), c->"-", c->"-"]);
        match section {
            Section::SEC32(section32) => print_section32(section32, table),
            Section::SEC64(section64) => print_section64(section64, table),
        }
    }
}

fn print_section32(section: &Section32, table: &mut Table) {
    print_common_section_fields(&section.sectname, &section.segname, section.addr as u64, section.size as u64, section.offset, section.align, section.reloff, section.nreloc, section.flags, section.reserved1, section.reserved2, None, table);
}

fn print_section64(section: &Section64, table: &mut Table) {
    print_common_section_fields(&section.sectname, &section.segname, section.addr, section.size, section.offset, section.align, section.reloff, section.nreloc, section.flags, section.reserved1, section.reserved2, None, table);
}

fn print_common_section_fields(sectname: &[u8], segname: &[u8], addr: u64, size: u64, offset: u32, align: u32, reloff: u32, nreloc: u32, flags: u32, reserved1: u32, reserved2: u32, reserved3: Option<u32>, table: &mut Table) {
    print_segname_or_sectname_bytes_array(sectname, table);
    print_segname_or_sectname_bytes_array(segname, table);
    table.add_row(row![Fcc->"addr", Fyc->format!("0x{:x}", addr), c->"-"]);
    table.add_row(row![Fcc->"size", Fyc->format!("0x{:x}", size), c->"-"]);
    table.add_row(row![Fcc->"offset", Fyc->format!("0x{:x}", offset), c->"-"]);
    table.add_row(row![Fcc->"align", Fyc->format!("0x{:x}", align), c->"-"]);
    table.add_row(row![Fcc->"reloff", Fyc->format!("0x{:x}", reloff), c->"-"]);
    table.add_row(row![Fcc->"nreloc", Fyc->format!("0x{:x}", nreloc), c->"-"]);
    table.add_row(row![Fcc->"flags", Fyc->format!("0x{:x}", flags), c->"-"]);
    table.add_row(row![Fcc->"reserved1", Fyc->format!("0x{:x}", reserved1), c->"-"]);
    table.add_row(row![Fcc->"reserved2", Fyc->format!("0x{:x}", reserved2), c->"-"]);
    if let Some(r3) = reserved3 {
        table.add_row(row![Fcc->"reserved3", Fyc->format!("0x{:x}", r3), c->"-"]);
    }
}

unsafe fn print_dylib_command(command: &DylibCommand, lc_str: String, mut table: &mut Table) {
    print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, &mut table);
    table.add_row(row![Frbc->"struct dylib = {", c->"-", c->"-"]);
    table.add_row(row![ Fcc->"name.offset", Fyc->format!("0x{:x}", command.dylib.name.offset),  c->"-"]);
    table.add_row(row![ Fcc->"timestamp", Fyc->format!("0x{:x}", command.dylib.timestamp),  c->"-"]);
    table.add_row(row![ Fcc->"current_version", Fyc->format!("0x{:x}", command.dylib.current_version),  c->"-"]);
    table.add_row(row![ Fcc->"compatibility_version", Fyc->format!("0x{:x}", command.dylib.compatibility_version),  c->"-"]);
    table.add_row(row![Frbc->"}", c->"-", c->"-"]);
    table.add_row(row![ Fcc->"name (lc_str)", Fyc->"-",  c->lc_str]);
}

fn print_common_lcstr(cmd: u32, cmdsize: u32, lc_str_name: &str, lc_str: String, table: &mut Table) {
    print_lc_cmd_and_cmdsize(cmd, cmdsize, table);
    table.add_row(row![ Fcc->format!("{} (lc_str)", lc_str_name), Fyc->"-",  c->lc_str]);
}

fn print_prebound_dylib_command(command: &PreboundDylibCommand, table: &mut Table) {
    print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, table);
    // TODO: this is problematic because this command has 2 lc_str in its LcStr struct (2 in one Vec<u8>) printing should be different.
}

fn print_thread_command(command: &ThreadCommand, table: &mut Table) {
    print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, table);
    // TODO: implement this after we manage to make it work in parser.rs as well.
}

fn print_routines_command_32(command: &RoutinesCommand32, table: &mut Table) {
    print_common_routines_fields(command.cmd, command.cmdsize, command.init_address as u64, command.init_module as u64, command.reserved1 as u64, command.reserved2 as u64, command.reserved3 as u64, command.reserved4 as u64, command.reserved5 as u64, command.reserved6 as u64, table);
}

fn print_routines_command_64(command: &RoutinesCommand64, table: &mut Table) {
    print_common_routines_fields(command.cmd, command.cmdsize, command.init_address, command.init_module, command.reserved1, command.reserved2, command.reserved3, command.reserved4, command.reserved5, command.reserved6, table);
}

fn print_common_routines_fields(cmd: u32, cmdsize: u32, init_address: u64, init_module: u64, reserved1: u64, reserved2: u64, reserved3: u64, reserved4: u64, reserved5: u64, reserved6: u64, table: &mut Table) {
    print_lc_cmd_and_cmdsize(cmd, cmdsize, table);
    table.add_row(row![Fcc->"init_address", Fyc->format!("0x{:x}", init_address), c->"-"]);
    table.add_row(row![Fcc->"init_module", Fyc->format!("0x{:x}", init_module), c->"-"]);
    table.add_row(row![Fcc->"reserved1", Fyc->format!("0x{:x}", reserved1), c->"-"]);
    table.add_row(row![Fcc->"reserved2", Fyc->format!("0x{:x}", reserved2), c->"-"]);
    table.add_row(row![Fcc->"reserved3", Fyc->format!("0x{:x}", reserved3), c->"-"]);
    table.add_row(row![Fcc->"reserved4", Fyc->format!("0x{:x}", reserved4), c->"-"]);
    table.add_row(row![Fcc->"reserved5", Fyc->format!("0x{:x}", reserved5), c->"-"]);
    table.add_row(row![Fcc->"reserved6", Fyc->format!("0x{:x}", reserved6), c->"-"]);
}

fn print_symtab_command(command: &SymtabCommand, table: &mut Table) {
    print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, table);
    table.add_row(row![ Fcc->"symoff", Fyc->format!("0x{:x}", command.symoff),  c->"-"]);
    table.add_row(row![ Fcc->"nsyms", Fyc->format!("0x{:x}", command.nsyms),  c->"-"]);
    table.add_row(row![ Fcc->"stroff", Fyc->format!("0x{:x}", command.stroff),  c->"-"]);
    table.add_row(row![ Fcc->"strsize", Fyc->format!("0x{:x}", command.strsize),  c->"-"]);
}

fn print_dynsymtab_command(command: &DynSymtabCommand, table: &mut Table) {
    print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, table);
    table.add_row(row![ Fcc->"ilocalsym", Fyc->format!("0x{:x}", command.ilocalsym),  c->"-"]);
    table.add_row(row![ Fcc->"nlocalsym", Fyc->format!("0x{:x}", command.nlocalsym),  c->"-"]);
    table.add_row(row![ Fcc->"iextdefsym", Fyc->format!("0x{:x}", command.iextdefsym),  c->"-"]);
    table.add_row(row![ Fcc->"nextdefsym", Fyc->format!("0x{:x}", command.nextdefsym),  c->"-"]);
    table.add_row(row![ Fcc->"iundefsym", Fyc->format!("0x{:x}", command.iundefsym),  c->"-"]);
    table.add_row(row![ Fcc->"nundefsym", Fyc->format!("0x{:x}", command.nundefsym),  c->"-"]);
    table.add_row(row![ Fcc->"tocoff", Fyc->format!("0x{:x}", command.tocoff),  c->"-"]);
    table.add_row(row![ Fcc->"ntoc", Fyc->format!("0x{:x}", command.ntoc),  c->"-"]);
    table.add_row(row![ Fcc->"modtaboff", Fyc->format!("0x{:x}", command.modtaboff),  c->"-"]);
    table.add_row(row![ Fcc->"nmodtab", Fyc->format!("0x{:x}", command.nmodtab),  c->"-"]);
    table.add_row(row![ Fcc->"extrefsymoff", Fyc->format!("0x{:x}", command.extrefsymoff),  c->"-"]);
    table.add_row(row![ Fcc->"nextrefsyms", Fyc->format!("0x{:x}", command.nextrefsyms),  c->"-"]);
    table.add_row(row![ Fcc->"indirectsymoff", Fyc->format!("0x{:x}", command.indirectsymoff),  c->"-"]);
    table.add_row(row![ Fcc->"nindirectsyms", Fyc->format!("0x{:x}", command.nindirectsyms),  c->"-"]);
    table.add_row(row![ Fcc->"extreloff", Fyc->format!("0x{:x}", command.extreloff),  c->"-"]);
    table.add_row(row![ Fcc->"nextrel", Fyc->format!("0x{:x}", command.nextrel),  c->"-"]);
    table.add_row(row![ Fcc->"locreloff", Fyc->format!("0x{:x}", command.locreloff),  c->"-"]);
    table.add_row(row![ Fcc->"nlocrel", Fyc->format!("0x{:x}", command.nlocrel),  c->"-"]);
}

fn print_two_level_hints_command(command: &TwoLevelHintsCommand, table: &mut Table) {
    print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, table);
    table.add_row(row![ Fcc->"offset", Fyc->format!("0x{:x}", command.offset),  c->"-"]);
    table.add_row(row![ Fcc->"nhints", Fyc->format!("0x{:x}", command.nhints),  c->"-"]);
}

fn print_prebind_cksum_command(command: &PrebindCksumCommand, table: &mut Table) {
    print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, table);
    table.add_row(row![ Fcc->"cksum", Fyc->format!("0x{:x}", command.cksum),  c->"-"]);
}

fn print_uuid_command(command: &UuidCommand, table: &mut Table) {
    print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, table);
    //TODO print as bytes...
}

fn print_linkedit_data_command(command: &LinkeditDataCommand, table: &mut Table) {
    print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, table);
    table.add_row(row![ Fcc->"dataoff", Fyc->format!("0x{:x}", command.dataoff),  c->"-"]);
    table.add_row(row![ Fcc->"datasize", Fyc->format!("0x{:x}", command.datasize),  c->"-"]);
}

fn print_encryption_info_command_32(command: &EncryptionInfoCommand32, table: &mut Table) {
    print_common_encryption_info_fields(command.cmd, command.cmdsize, command.cryptoff, command.cryptsize, command.cryptid, None, table);
}

fn print_encryption_info_command_64(command: &EncryptionInfoCommand64, table: &mut Table) {
    print_common_encryption_info_fields(command.cmd, command.cmdsize, command.cryptoff, command.cryptsize, command.cryptid, Some(command.pad), table);
}

fn print_version_min_command(command: &VersionMinCommand, table: &mut Table) {
    print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, table);
    table.add_row(row![ Fcc->"version", Fyc->format!("0x{:x}", command.version),  c->"-"]);
    table.add_row(row![ Fcc->"sdk", Fyc->format!("0x{:x}", command.sdk),  c->"-"]);
}

fn print_build_version_command(command: &BuildVersionCommand, table: &mut Table) {
    print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, table);
    table.add_row(row![ Fcc->"platform", Fyc->format!("0x{:x}", command.platform),  c->"-"]);
    table.add_row(row![ Fcc->"minos", Fyc->format!("0x{:x}", command.minos),  c->"-"]);
    table.add_row(row![ Fcc->"sdk", Fyc->format!("0x{:x}", command.sdk),  c->"-"]);
    table.add_row(row![ Fcc->"ntools", Fyc->format!("0x{:x}", command.ntools),  c->"-"]);
}

fn print_dyld_info_command(command: &DyldInfoCommand, table: &mut Table) {
    print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, table);
    table.add_row(row![ Fcc->"rebase_off", Fyc->format!("0x{:x}", command.rebase_off),  c->"-"]);
    table.add_row(row![ Fcc->"rebase_size", Fyc->format!("0x{:x}", command.rebase_size),  c->"-"]);
    table.add_row(row![ Fcc->"bind_off", Fyc->format!("0x{:x}", command.bind_off),  c->"-"]);
    table.add_row(row![ Fcc->"bind_size", Fyc->format!("0x{:x}", command.bind_size),  c->"-"]);
    table.add_row(row![ Fcc->"weak_bind_off", Fyc->format!("0x{:x}", command.weak_bind_off),  c->"-"]);
    table.add_row(row![ Fcc->"weak_bind_size", Fyc->format!("0x{:x}", command.weak_bind_size),  c->"-"]);
    table.add_row(row![ Fcc->"lazy_bind_off", Fyc->format!("0x{:x}", command.lazy_bind_off),  c->"-"]);
    table.add_row(row![ Fcc->"lazy_bind_size", Fyc->format!("0x{:x}", command.lazy_bind_size),  c->"-"]);
    table.add_row(row![ Fcc->"export_off", Fyc->format!("0x{:x}", command.export_off),  c->"-"]);
    table.add_row(row![ Fcc->"export_size", Fyc->format!("0x{:x}", command.export_size),  c->"-"]);
}

fn print_linker_options_command(command: &LinkerOptionCommand, table: &mut Table) {
    print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, table);
    table.add_row(row![ Fcc->"count", Fyc->format!("0x{:x}", command.count),  c->"-"]);
}

fn print_symseg_command(command: &SymsegCommand, table: &mut Table) {
    print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, table);
    table.add_row(row![ Fcc->"offset", Fyc->format!("0x{:x}", command.offset),  c->"-"]);
    table.add_row(row![ Fcc->"size", Fyc->format!("0x{:x}", command.size),  c->"-"]);
}

fn print_entry_point_command(command: &EntryPointCommand, table: &mut Table) {
    print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, table);
    table.add_row(row![ Fcc->"entryoff", Fyc->format!("0x{:x}", command.entryoff),  c->"-"]);
    table.add_row(row![ Fcc->"stacksize", Fyc->format!("0x{:x}", command.stacksize),  c->"-"]);
}

fn print_source_version_command(command: &SourceVersionCommand, table: &mut Table) {
    print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, table);
    table.add_row(row![ Fcc->"version", Fyc->format!("0x{:x}", command.version),  c->"-"]);
}

fn print_note_command(command: &NoteCommand, table: &mut Table) {
    print_lc_cmd_and_cmdsize(command.cmd, command.cmdsize, table);
    //TODO: print as bytes...
    table.add_row(row![ Fcc->"offset", Fyc->format!("0x{:x}", command.offset),  c->"-"]);
    table.add_row(row![ Fcc->"size", Fyc->format!("0x{:x}", command.size),  c->"-"]);
}

fn print_common_encryption_info_fields(cmd: u32, cmdsize: u32, cryptoff: u32, cryptsize: u32, cryptid: u32, pad: Option<u32>, table: &mut Table) {
    print_lc_cmd_and_cmdsize(cmd, cmdsize, table);
    table.add_row(row![Fcc->"cryptoff", Fyc->format!("0x{:x}", cryptoff), c->"-"]);
    table.add_row(row![Fcc->"cryptsize", Fyc->format!("0x{:x}", cryptsize), c->"-"]);
    table.add_row(row![Fcc->"cryptid", Fyc->format!("0x{:x}", cryptid), c->"-"]);
    if let Some(p) = pad {
        table.add_row(row![Fcc->"pad", Fyc->format!("0x{:x}", p), c->"-"]);
    }
}

fn print_common_title(title: &str, table: &mut Table) {
    table.add_row(row![FBbc->title, c->"-", c->"-"]);
    table.add_row(row![Bbbc=>"Field", "Value", "Extra Info"]);
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

    table.add_row(row![ Fcc->"cmd", Fyc->format!("0x{:x}\n({})", cmd, cmd_string),  c->"-"]);
    table.add_row(row![ Fcc->"cmdsize", Fyc->format!("0x{:x}", cmdsize),  c->"-"]);
}
