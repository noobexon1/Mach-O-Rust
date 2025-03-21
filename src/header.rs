use std::io::Read;

use byteorder::ReadBytesExt;
use crate::error::AppError;

#[derive(Debug)]
pub enum MachHeader {
    MH32(MachHeader32),
    MH64(MachHeader64),
}

impl MachHeader {
    pub fn ncmds(&self) -> u32 {
        match self {
            MachHeader::MH32(header) => header.ncmds,
            MachHeader::MH64(header) => header.ncmds,
        }
    }
}

#[derive(Debug)]
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
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, magic: u32) -> Result<MachHeader, AppError> {
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

#[derive(Debug)]
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
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, magic: u32) -> Result<MachHeader, AppError> {
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
