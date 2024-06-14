use std::io::Read;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use crate::constants::{MH_CIGAM, MH_CIGAM_64, MH_MAGIC, MH_MAGIC_64};
use crate::error::AppError;

pub type Symtab = Vec<Nlist>;

#[derive(Debug)]
pub enum Nlist {
    NL32(Nlist32),
    NL64(Nlist64),
}

impl Nlist {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, magic: u32) -> Result<Nlist, AppError> {
        match magic {
            MH_CIGAM_64 | MH_MAGIC_64 => Nlist64::from_file::<R, E>(file),
            MH_CIGAM | MH_MAGIC => Nlist32::from_file::<R, E>(file),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Nlist32 {
    pub n_strx: u32,
    pub n_type: u8,
    pub n_sect: u8,
    pub n_desc: i16,
    pub n_value: u32,
}

impl Nlist32 {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R) -> Result<Nlist, AppError> {
        let nlist32 = Nlist32 {
            n_strx: file.read_u32::<E>()?,
            n_type: file.read_u8()?,
            n_sect: file.read_u8()?,
            n_desc: file.read_i16::<E>()?,
            n_value: file.read_u32::<E>()?,
        };
        Ok(Nlist::NL32(nlist32))
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Nlist64 {
    pub n_strx: u32,
    pub n_type: u8,
    pub n_sect: u8,
    pub n_desc: u16,
    pub n_value: u64,
}

impl Nlist64 {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R) -> Result<Nlist, AppError> {
        let nlist64 = Nlist64 {
            n_strx: file.read_u32::<E>()?,
            n_type: file.read_u8()?,
            n_sect: file.read_u8()?,
            n_desc: file.read_u16::<E>()?,
            n_value: file.read_u64::<E>()?,
        };
        Ok(Nlist::NL64(nlist64))
    }
}