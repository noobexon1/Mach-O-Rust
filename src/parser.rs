use std::io;
use std::io::{Read, Seek};

use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};

use crate::constants::*;
use crate::error::AppError;
use crate::header::*;
use crate::load_commands::*;
use crate::mach_o::MachO;
use crate::memory_utils::{advance_to_next_load_command, get_file_offset};

pub fn parse<R: Read + Seek>(file: &mut R) -> Result<MachO, AppError> {
    let magic = file.read_u32::<BigEndian>()?;
    check_magic_number(magic)?;

    let (header, load_commands) = match magic {
        MH_MAGIC | MH_MAGIC_64 => {
            let header = parse_header::<R, BigEndian>(file, magic)?;
            let load_commands = parse_load_commands::<R, BigEndian>(file, &header)?;
            (header, load_commands)
        },
        MH_CIGAM | MH_CIGAM_64 => {
            let header = parse_header::<R, LittleEndian>(file, magic)?;
            let load_commands = parse_load_commands::<R, LittleEndian>(file, &header)?;
            (header, load_commands)
        },
        _ => unreachable!(),
    };

    Ok(MachO {
        header,
        load_commands,
    })
}

fn check_magic_number(magic: u32) -> Result<(), AppError> {
    match magic {
        MH_MAGIC | MH_MAGIC_64 | MH_CIGAM | MH_CIGAM_64 => Ok(()),
        _ => Err(AppError::from(io::Error::new(io::ErrorKind::InvalidData, "Invalid Mach-O magic number")))
    }
}

fn parse_header<R: Read + Seek, E: ByteOrder>(file: &mut R, magic: u32) -> Result<MachHeader, AppError> {
    match magic {
        MH_MAGIC | MH_CIGAM => MachHeader32::from_file::<R, E>(file, magic),
        MH_MAGIC_64 | MH_CIGAM_64 => MachHeader64::from_file::<R, E>(file, magic),
        _ => unreachable!(),
    }
}

fn parse_load_commands<R: Read + Seek, E: ByteOrder>(file: &mut R, header: &MachHeader) -> Result<(Vec<LoadCommand>, Vec<Vec<Section>>, Vec<LcStr>), AppError> {
    let mut load_commands: Vec<LoadCommand> = Vec::new();
    let mut sections: Vec<Vec<Section>> = Vec::new();
    let mut load_commands_strings: Vec<LcStr> = Vec::new();

    for _ in 0..header.ncmds() {
        let offset = get_file_offset(file)?;
        let load_command_prefix = LoadCommandPrefix::from_file::<R, E>(file)?;
        let load_command = parse_command::<R, E>(file, &load_command_prefix)?;
        let load_command_sections = parse_sections_for_segment::<R, E>(file, &load_command)?;
        let load_command_string = parse_load_command_string::<R, E>(file, &load_command, offset, load_command_prefix.cmdsize)?;

        load_commands.push(load_command);
        sections.push(load_command_sections);
        load_commands_strings.push(load_command_string);

        // TODO: this one could be removed after we finish the parsing inline logic.
        advance_to_next_load_command(file, offset, load_command_prefix.cmdsize as u64)?;
    }
    Ok((load_commands, sections, load_commands_strings))
}

// TODO: Handle all of the warnings emitted from building. I need to sub-parse some commands...
fn parse_command<R: Read, E: ByteOrder>(file: &mut R, load_command_prefix: &LoadCommandPrefix) ->Result<LoadCommand, AppError> {
    match load_command_prefix.cmd {
        LC_SYMTAB => SymtabCommand::from_file::<R, E>(file, load_command_prefix),
        LC_SYMSEG => SymsegCommand::from_file::<R, E>(file, load_command_prefix),
        LC_THREAD | LC_UNIXTHREAD => ThreadCommand::from_file::<R, E>(file, load_command_prefix),
        LC_IDENT => IdentCommand::from_file::<E>(load_command_prefix),
        LC_DYSYMTAB => DynSymtabCommand::from_file::<R, E>(file, load_command_prefix),
        LC_LOAD_DYLIB | LC_ID_DYLIB | LC_LOAD_WEAK_DYLIB | LC_REEXPORT_DYLIB => DylibCommand::from_file::<R, E>(file, load_command_prefix),
        LC_LOAD_DYLINKER | LC_ID_DYLINKER | LC_DYLD_ENVIRONMENT => DylinkerCommand::from_file::<R, E>(file, load_command_prefix),
        LC_PREBOUND_DYLIB => PreboundDylibCommand::from_file::<R, E>(file, load_command_prefix),
        LC_ROUTINES => RoutinesCommand32::from_file::<R, E>(file, load_command_prefix),
        LC_SUB_FRAMEWORK => SubFrameWorkCommand::from_file::<R, E>(file, load_command_prefix),
        LC_SUB_UMBRELLA => SubUmbrellaCommand::from_file::<R, E>(file, load_command_prefix),
        LC_SUB_CLIENT => SubClientCommand::from_file::<R, E>(file, load_command_prefix),
        LC_SUB_LIBRARY => SubLibraryCommand::from_file::<R, E>(file, load_command_prefix),
        LC_TWOLEVEL_HINTS => TwoLevelHintsCommand::from_file::<R, E>(file, load_command_prefix),
        LC_PREBIND_CKSUM => PrebindCksumCommand::from_file::<R, E>(file, load_command_prefix),
        LC_SEGMENT => SegmentCommand32::from_file::<R, E>(file, load_command_prefix),
        LC_SEGMENT_64 => SegmentCommand64::from_file::<R, E>(file, load_command_prefix),
        LC_ROUTINES_64 => RoutinesCommand64::from_file::<R, E>(file, load_command_prefix),
        LC_UUID => UuidCommand::from_file::<R, E>(file, load_command_prefix),
        LC_RPATH => RpathCommand::from_file::<R, E>(file, load_command_prefix),
        LC_CODE_SIGNATURE | LC_SEGMENT_SPLIT_INFO | LC_FUNCTION_STARTS | LC_DATA_IN_CODE | LC_DYLIB_CODE_SIGN_DRS | LC_LINKER_OPTIMIZATION_HINT => LinkeditDataCommand::from_file::<R, E>(file, load_command_prefix),
        LC_ENCRYPTION_INFO => EncryptionInfoCommand32::from_file::<R, E>(file, load_command_prefix),
        LC_DYLD_INFO | LC_DYLD_INFO_ONLY => DyldInfoCommand::from_file::<R, E>(file, load_command_prefix),
        LC_VERSION_MIN_MACOSX | LC_VERSION_MIN_IPHONEOS | LC_VERSION_MIN_TVOS | LC_VERSION_MIN_WATCHOS => SegmentCommand32::from_file::<R, E>(file, load_command_prefix),
        LC_MAIN => EntryPointCommand::from_file::<R, E>(file, load_command_prefix),
        LC_SOURCE_VERSION => SourceVersionCommand::from_file::<R, E>(file, load_command_prefix),
        LC_ENCRYPTION_INFO_64 => EncryptionInfoCommand64::from_file::<R, E>(file, load_command_prefix),
        LC_LINKER_OPTION => LinkerOptionCommand::from_file::<R, E>(file, load_command_prefix),
        LC_NOTE => NoteCommand::from_file::<R, E>(file, load_command_prefix),
        LC_BUILD_VERSION => BuildVersionCommand::from_file::<R, E>(file, load_command_prefix),
        _ => Err(AppError::from(io::Error::new(io::ErrorKind::InvalidData, "unknown load command type!")))
    }
}


// TODO: make sure this works as intentional by printing it in printer.rs or by debug...
fn parse_sections_for_segment<R: Read + Seek, E: ByteOrder>(file: &mut R, load_command: &LoadCommand) -> Result<Vec<Section>, AppError> {
    let mut load_command_sections = Vec::new();
    match load_command {
        LoadCommand::SegmentCommand(command) => {
            match command {
                SegmentCommand::SEG32(command) => {
                    for _ in 0..command.nsects {
                        let section = Section32::from_file::<R, E>(file)?;
                        load_command_sections.push(section);
                    }
                }
                SegmentCommand::SEG64(command) => {
                    for _ in 0..command.nsects {
                        let section = Section64::from_file::<R, E>(file)?;
                        load_command_sections.push(section);
                    }
                }
            }
        }
        _ => {}
    }
    Ok(load_command_sections)
}

fn parse_load_command_string<R: Read + Seek, E: ByteOrder>(file: &mut R, load_command: &LoadCommand, lc_offset: u64, cmdsize: u32) -> Result<LcStr, AppError> {
    let mut load_command_string = Vec::new();
    match load_command {
        LoadCommand::DylibCommand(_) |
        LoadCommand::SubFrameWorkCommand(_) |
        LoadCommand::SubClientCommand(_) |
        LoadCommand::SubUmbrellaCommand(_) |
        LoadCommand::SubLibraryCommand(_) |
        LoadCommand::PreboundDylibCommand(_) |
        LoadCommand::DylinkerCommand(_) |
        LoadCommand::RpathCommand(_) => {
            let remaining_size = get_load_command_remaining_size(lc_offset, cmdsize as u64, get_file_offset(file)?)?;
            if remaining_size > 0 {
                for _ in 0..remaining_size {
                    load_command_string.push(file.read_u8()?);
                }
            }
        },
        _ => {}
    }
    Ok(load_command_string)
}

fn get_load_command_remaining_size(lc_offset: u64, lc_size: u64, file_offset: u64) -> Result<u64, AppError> {
    Ok((lc_offset + lc_size) - file_offset)
}