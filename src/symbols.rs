

#[derive(Debug)]
pub struct SymTab {
    sym_tbl_offset: u32,
    nsyms: u32,
    symtab: Vec<Nlist>,
    str_tbl_offset: u32,
    str_tbl_size: u32
}


#[derive(Debug)]
pub enum Nlist {
    NL32(Nlist32),
    NL64(Nlist64),
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

#[derive(Debug)]
#[repr(C)]
pub struct Nlist64 {
    pub n_strx: u32,
    pub n_type: u8,
    pub n_sect: u8,
    pub n_desc: u16,
    pub n_value: u64,
}