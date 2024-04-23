use std::io;
use std::io::Read;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

use crate::header::*;

pub struct MachO {
    header: MachHeader,
}

impl MachO {
    pub fn get_header(&self) -> &MachHeader {
        &self.header
    }
}

// TODO: Maybe implement a trait or a function to read an array of 16 bytes because it creates boilerplate in structs...
// TODO: Consider refactoring load command by making an enum of load commands because they are all connected in some way...


pub fn parse<R: Read>(file: &mut R) -> MachO {
    let header = match parse_header(file) {
        Ok(header) => header,
        Err(e) => panic!("Error on header parsing: {}", e),
    };

    MachO {
        header,
    }
}

fn parse_header<R: Read>(file: &mut R) -> io::Result<MachHeader> {
    let magic = file.read_u32::<BigEndian>()?;

    match magic {
        MH_MAGIC => MachHeader32::from_file::<R, BigEndian>(file, magic),
        MH_CIGAM => MachHeader32::from_file::<R, LittleEndian>(file, magic),
        MH_MAGIC_64 => MachHeader64::from_file::<R, BigEndian>(file, magic),
        MH_CIGAM_64 => MachHeader64::from_file::<R, LittleEndian>(file, magic),
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid magic number")),
    }
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
            MachHeader::MH32(header) => {
                assert_eq!(header.magic, MH_MAGIC);
                assert_eq!(header.cputype, 7);
                assert_eq!(header.cpusubtype, 3);
                assert_eq!(header.filetype, 2);
                assert_eq!(header.ncmds, 5);
                assert_eq!(header.sizeofcmds, 1024);
                assert_eq!(header.flags, 1);
            }
            _ => panic!("Expected MachHeader32, found MachHeader64"),
        }
    }

    #[test]
    fn test_parse_header_64() {
        let data = mock_mach_header64();
        let mut cursor = Cursor::new(data);
        let header = parse_header(&mut cursor).unwrap();
        match header {
            MachHeader::MH64(header) => {
                assert_eq!(header.magic, MH_MAGIC_64);
                assert_eq!(header.cputype, 7);
                assert_eq!(header.cpusubtype, 3);
                assert_eq!(header.filetype, 2);
                assert_eq!(header.ncmds, 5);
                assert_eq!(header.sizeofcmds, 2048);
                assert_eq!(header.flags, 1);
                assert_eq!(header.reserved, 0);
            }
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
            (MH_MAGIC, false),
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
                MachHeader::MH32(_) if !is_64 => (),
                MachHeader::MH64(_) if *is_64 => (),
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
