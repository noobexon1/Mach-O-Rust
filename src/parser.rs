use std::io;
use std::io::Read;

use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};

use crate::header::*;
use crate::load_commands::{LoadCommand, LoadCommandVariant};

pub struct MachO {
    header: MachHeader,
    load_commands: Vec<LoadCommandVariant>,
}

impl MachO {
    pub fn get_header(&self) -> &MachHeader {
        &self.header
    }
}

pub fn parse<R: Read>(file: &mut R) -> MachO {
    let magic = file.read_u32::<BigEndian>().unwrap();

    // TODO: use high order function to execute_with_endian(<func>, <magic>) to minimize boilerplate
    let header = match magic {
        MH_MAGIC | MH_MAGIC_64 => {
            match parse_header::<R, BigEndian>(file, magic) {
                Ok(header) => header,
                Err(e) => panic!("Error on header parsing: {}", e),
            }
        }
        MH_CIGAM | MH_CIGAM_64 => {
            match parse_header::<R, LittleEndian>(file, magic) {
                Ok(header) => header,
                Err(e) => panic!("Error on header parsing: {}", e),
            }
        }
        _ => panic!("Invalid magic number!"),
    };

    let load_commands = match magic {
        MH_MAGIC | MH_MAGIC_64 => {
            match parse_load_commands::<R, BigEndian>(file, &header) {
                Ok(load_commands) => load_commands,
                Err(e) => panic!("Error on load commands parsing: {}", e),
            }
        }
        MH_CIGAM | MH_CIGAM_64 => {
            match parse_load_commands::<R, LittleEndian>(file, &header) {
                Ok(load_commands) => load_commands,
                Err(e) => panic!("Error on load commands parsing: {}", e),
            }
        }
        _ => panic!("Invalid magic number!"),
    };

    MachO {
        header,
        load_commands
    }
}

fn parse_header<R: Read, E: ByteOrder>(file: &mut R, magic: u32) -> io::Result<MachHeader> {
    match magic {
        MH_MAGIC | MH_CIGAM => MachHeader32::from_file::<R, E>(file, magic),
        MH_MAGIC_64 | MH_CIGAM_64 => MachHeader64::from_file::<R, E>(file, magic),
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid magic number")),
    }
}

fn parse_load_commands<R: Read, E: ByteOrder>(file: &mut R, header: &MachHeader) -> io::Result<Vec<LoadCommandVariant>> {
    let mut load_commands: Vec<LoadCommandVariant> = Vec::new();

    for i in 0..header.ncmds() {
        let load_command = LoadCommand::from_file::<R, E>(file)?;

    }

    Ok(load_commands)
}
