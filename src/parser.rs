use std::io;
use std::io::Read;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

//TODO: make sure in loader.h that i included all of the structs required for th parser to operate.

//TODO: consider using constructors for the occupy_""(...) things..

#[repr(C)]
pub struct MachHeader32 {
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

enum MachHeaderVariant {
    MH32(MachHeader32),
    MH64(MachHeader64),
}

#[repr(C)]
pub struct LoadCommand {
    pub cmd: u32,
    pub cmdsize: u32,
}

#[repr(C)]
pub struct SegmentCommand32 {
    pub cmd: u32,
    pub cmdsize: u32,
    pub segname: [u8; 16],
    pub vmaddr: u32,
    pub vmsize: u32,
    pub fileoff: u32,
    pub filesize: u32,
    pub maxprot: i32,
    pub initprot: i32,
    pub nsects: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct SegmentCommand64 {
    pub cmd: u32,
    pub cmdsize: u32,
    pub segname: [u8; 16],
    pub vmaddr: u64,
    pub vmsize: u64,
    pub fileoff: u64,
    pub filesize: u64,
    pub maxprot: i32,
    pub initprot: i32,
    pub nsects: u32,
    pub flags: u32,
}

enum SegmentVariant {
    SEG32(SegmentCommand32),
    SEG64(SegmentCommand64),
}

#[repr(C)]
pub struct Section32 {
    pub sectname: [u8; 16],
    pub segname: [u8; 16],
    pub addr: u32,
    pub size: u32,
    pub offset: u32,
    pub align: u32,
    pub reloff: u32,
    pub nreloc: u32,
    pub flags: u32,
    pub reserved1: u32,
    pub reserved2: u32,
}

#[repr(C)]
pub struct Section64 {
    pub sectname: [u8; 16],
    pub segname: [u8; 16],
    pub addr: u64,
    pub size: u64,
    pub offset: u32,
    pub align: u32,
    pub reloff: u32,
    pub nreloc: u32,
    pub flags: u32,
    pub reserved1: u32,
    pub reserved2: u32,
    pub reserved3: u32,
}

enum SectionVariant {
    SEC32(Section32),
    SEC64(Section64),
}

#[repr(C)]
pub struct DyldInfo {
    pub cmd: u32,
    pub cmdsize: u32,
    pub rebase_off: u32,
    pub rebase_size: u32,
    pub bind_off: u32,
    pub bind_size: u32,
    pub weak_bind_off: u32,
    pub weak_bind_size: u32,
    pub lazy_bind_off: u32,
    pub lazy_bind_size: u32,
    pub export_off: u32,
    pub export_size: u32,
}

#[repr(C)]
pub struct Symtab {
    pub cmd: u32,
    pub cmdsize: u32,
    pub symoff: u32,
    pub nsyms: u32,
    pub stroff: u32,
    pub strsize: u32,
}

#[repr(C)]
pub struct DynSymtab {
    pub cmd: u32,
    pub cmdsize: u32,
    pub ilocalsym: u32,
    pub nlocalsym: u32,
    pub iextdefsym: u32,
    pub nextdefsym: u32,
    pub iundefsym: u32,
    pub nundefsym: u32,
    pub tocoff: u32,
    pub ntoc: u32,
    pub modtaboff: u32,
    pub nmodtab: u32,
    pub extrefsymoff: u32,
    pub nextrefsyms: u32,
    pub indirectsymoff: u32,
    pub nindirectsyms: u32,
    pub extreloff: u32,
    pub nextrel: u32,
    pub locreloff: u32,
    pub nlocrel: u32,
}

#[repr(C)]
pub struct UuidCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub uuid: [u8; 16],
}

pub fn parse<R: Read>(file: &mut R) {
    let header =  parse_header(file);
    match header {
        Ok(header) => header,
        Err(e) => panic!("Error on header parsing: {}", e),
    };
}

fn parse_header<R: Read>(file: &mut R) -> io::Result<MachHeaderVariant> {
    let magic = file.read_u32::<BigEndian>()?;

    match magic {
        MH_MAGIC => {
            occupy_header32::<R, BigEndian>(file, magic)
        }
        MH_CIGAM => {
            occupy_header32::<R, LittleEndian>(file, magic)
        }
        MH_MAGIC_64 => {
            occupy_header64::<R, BigEndian>(file, magic)
        }
        MH_CIGAM_64 => {
            occupy_header64::<R, LittleEndian>(file, magic)
        }
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid magic number")),
    }
}

fn occupy_header32<R: Read, E: byteorder::ByteOrder>(file: &mut R, magic: u32) -> io::Result<MachHeaderVariant> {
    let header = MachHeader32 {
        magic,
        cputype: file.read_i32::<E>()?,
        cpusubtype: file.read_i32::<E>()?,
        filetype: file.read_u32::<E>()?,
        ncmds: file.read_u32::<E>()?,
        sizeofcmds: file.read_u32::<E>()?,
        flags: file.read_u32::<E>()?,
    };
    Ok(MachHeaderVariant::MH32(header))
}

fn occupy_header64<R: Read, E: byteorder::ByteOrder>(file: &mut R, magic: u32) -> io::Result<MachHeaderVariant> {
    let header = MachHeader64 {
        magic,
        cputype: file.read_i32::<E>()?,
        cpusubtype: file.read_i32::<E>()?,
        filetype: file.read_u32::<E>()?,
        ncmds: file.read_u32::<E>()?,
        sizeofcmds: file.read_u32::<E>()?,
        flags: file.read_u32::<E>()?,
        reserved: file.read_u32::<E>()?,
    };
    Ok(MachHeaderVariant::MH64(header))
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use byteorder::{BigEndian, WriteBytesExt};

    use super::*;

    fn mock_mach_header32() -> Vec<u8> {
        let mut data = vec![];
        data.write_u32::<BigEndian>(MH_MAGIC).unwrap();
        data.write_i32::<BigEndian>(7).unwrap(); // cputype
        data.write_i32::<BigEndian>(3).unwrap(); // cpusubtype
        data.write_u32::<BigEndian>(2).unwrap(); // filetype
        data.write_u32::<BigEndian>(5).unwrap(); // ncmds
        data.write_u32::<BigEndian>(1024).unwrap(); // sizeofcmds
        data.write_u32::<BigEndian>(1).unwrap(); // flags
        data
    }

    fn mock_mach_header64() -> Vec<u8> {
        let mut data = vec![];
        data.write_u32::<BigEndian>(MH_MAGIC_64).unwrap();
        data.write_i32::<BigEndian>(7).unwrap(); // cputype
        data.write_i32::<BigEndian>(3).unwrap(); // cpusubtype
        data.write_u32::<BigEndian>(2).unwrap(); // filetype
        data.write_u32::<BigEndian>(5).unwrap(); // ncmds
        data.write_u32::<BigEndian>(2048).unwrap(); // sizeofcmds
        data.write_u32::<BigEndian>(1).unwrap(); // flags
        data.write_u32::<BigEndian>(0).unwrap(); // reserved
        data
    }

    // Helper function to create mock data for headers
    fn create_mock_header<E: byteorder::ByteOrder>(magic: u32, is_64: bool) -> Vec<u8> {
        let mut data = vec![];
        data.write_u32::<E>(magic).unwrap();
        data.write_i32::<E>(7).unwrap(); // cputype
        data.write_i32::<E>(3).unwrap(); // cpusubtype
        data.write_u32::<E>(2).unwrap(); // filetype
        data.write_u32::<E>(5).unwrap(); // ncmds
        data.write_u32::<E>(1024).unwrap(); // sizeofcmds
        data.write_u32::<E>(1).unwrap(); // flags
        if is_64 {
            data.write_u32::<E>(0).unwrap(); // reserved for 64-bit headers
        }
        data
    }

    #[test]
    fn test_parse_header_32() {
        let data = mock_mach_header32();
        let mut cursor = Cursor::new(data);
        let header = parse_header(&mut cursor).unwrap();
        match header {
            MachHeaderVariant::MH32(header) => {
                assert_eq!(header.magic, MH_MAGIC);
                assert_eq!(header.cputype, 7);
                assert_eq!(header.cpusubtype, 3);
                assert_eq!(header.filetype, 2);
                assert_eq!(header.ncmds, 5);
                assert_eq!(header.sizeofcmds, 1024);
                assert_eq!(header.flags, 1);
            },
            _ => panic!("Expected MachHeader32, found MachHeader64"),
        }
    }

    #[test]
    fn test_parse_header_64() {
        let data = mock_mach_header64();
        let mut cursor = Cursor::new(data);
        let header = parse_header(&mut cursor).unwrap();
        match header {
            MachHeaderVariant::MH64(header) => {
                assert_eq!(header.magic, MH_MAGIC_64);
                assert_eq!(header.cputype, 7);
                assert_eq!(header.cpusubtype, 3);
                assert_eq!(header.filetype, 2);
                assert_eq!(header.ncmds, 5);
                assert_eq!(header.sizeofcmds, 2048);
                assert_eq!(header.flags, 1);
                assert_eq!(header.reserved, 0);
            },
            _ => panic!("Expected MachHeader64, found MachHeader32"),
        }
    }

    #[test]
    fn test_invalid_magic() {
        let data = vec![0u8; 4]; // Invalid magic number
        let mut cursor = Cursor::new(data);
        assert!(parse_header(&mut cursor).is_err());
    }

    #[test]
    fn test_all_possible_magic_numbers() {
        let magics = [
            (MH_MAGIC,  false),
            (MH_CIGAM, false),
            (MH_MAGIC_64, true),
            (MH_CIGAM_64, true),
        ];

        for (magic, is_64) in magics.iter() {
            let data = create_mock_header::<BigEndian>(*magic, *is_64);
            let mut cursor = Cursor::new(data);
            let result = parse_header(&mut cursor);
            assert!(result.is_ok(), "Failed to parse known valid magic number: 0x{:X}", magic);

            // Test with LittleEndian if applicable
            let data = create_mock_header::<LittleEndian>(*magic, *is_64);
            let mut cursor = Cursor::new(data);
            let result = parse_header(&mut cursor);
            assert!(result.is_ok(), "Failed to parse with LittleEndian for magic number: 0x{:X}", magic);

            match result.unwrap() {
                MachHeaderVariant::MH32(_) if !is_64 => (),
                MachHeaderVariant::MH64(_) if *is_64 => (),
                _ => panic!("Header type mismatch for magic number: 0x{:X}", magic),
            }
        }

        // Test an invalid magic number
        let invalid_magic = 0x12345678; // Randomly chosen invalid magic number
        let data = create_mock_header::<BigEndian>(invalid_magic, false);
        let mut cursor = Cursor::new(data);
        let result = parse_header(&mut cursor);
        assert!(result.is_err(), "Parsed an invalid magic number that should have failed");
    }
}
