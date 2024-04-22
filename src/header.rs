use std::io;
use std::io::Read;

use byteorder::ReadBytesExt;

pub enum MachHeader {
    MH32(MachHeader32),
    MH64(MachHeader64),
}

// Big endian, 32 bit Mach-O
pub const MH_MAGIC: u32 = 0xfeedface;
// Little endian, 32 bit Mach-O
pub const MH_CIGAM: u32 = 0xcefaedfe;

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

impl MachHeader32 {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, magic: u32) -> io::Result<MachHeader> {
        let header = MachHeader32 {
            magic,
            cputype: file.read_i32::<E>()?,
            cpusubtype: file.read_i32::<E>()?,
            filetype: file.read_u32::<E>()?,
            ncmds: file.read_u32::<E>()?,
            sizeofcmds: file.read_u32::<E>()?,
            flags: file.read_u32::<E>()?,
        };
        Ok(MachHeader::MH32(header))
    }
}

// Big endian, 64 bit Mach-O
pub const MH_MAGIC_64: u32 = 0xfeedfacf;
// Little endian, 64 bit Mach-O
pub const MH_CIGAM_64: u32 = 0xcffaedfe;

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

impl MachHeader64 {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, magic: u32) -> io::Result<MachHeader> {
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
        Ok(MachHeader::MH64(header))
    }
}