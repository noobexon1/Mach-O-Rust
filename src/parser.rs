use std::io;
use std::io::Read;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

// TODO: Maybe implement a trait or a function to read an array of 16 bytes because it creates boilerplate in structs...
// TODO: Consider refactoring load command by making an enum of load commands because they are all connected in some way...

enum MachHeader {
    MH32(MachHeader32),
    MH64(MachHeader64),
}

const MH_MAGIC: u32 = 0xfeedface; // Big endian, 32 bit Mach-O
const MH_CIGAM: u32 = 0xcefaedfe; // Little endian, 32 bit Mach-O

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

const MH_MAGIC_64: u32 = 0xfeedfacf; // Big endian, 64 bit Mach-O
const MH_CIGAM_64: u32 = 0xcffaedfe; // Little endian, 64 bit Mach-O

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

#[repr(C)]
pub struct LoadCommand {
    pub cmd: u32,
    pub cmdsize: u32,
}

impl LoadCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R) -> io::Result<LoadCommand> {
        let load_command = LoadCommand {
            cmd: file.read_u32::<E>()?,
            cmdsize: file.read_u32::<E>()?,
        };
        Ok(load_command)
    }
}

#[repr(C)]
pub union LcStr {
    pub offset: u32,
    pub ptr: *const u8,
}

enum SegmentCommand {
    SEG32(SegmentCommand32),
    SEG64(SegmentCommand64),
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

impl SegmentCommand32 {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<SegmentCommand> {
                let segment_command = SegmentCommand32 {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            segname: Self::read_segname(file)?,
            vmaddr: file.read_u32::<E>()?,
            vmsize: file.read_u32::<E>()?,
            fileoff: file.read_u32::<E>()?,
            filesize: file.read_u32::<E>()?,
            maxprot: file.read_i32::<E>()?,
            initprot: file.read_i32::<E>()?,
            nsects: file.read_u32::<E>()?,
            flags: file.read_u32::<E>()?,
        };

        Ok(SegmentCommand::SEG32(segment_command))
    }

    fn read_segname<R: Read>(file: &mut R) -> io::Result<[u8; 16]> {
        let mut segname = [0u8; 16];
        file.read_exact(&mut segname)?;
        Ok(segname)
    }
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

impl SegmentCommand64 {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<SegmentCommand> {
        let segment_command = SegmentCommand64 {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            segname: Self::read_segname(file)?,
            vmaddr: file.read_u64::<E>()?,
            vmsize: file.read_u64::<E>()?,
            fileoff: file.read_u64::<E>()?,
            filesize: file.read_u64::<E>()?,
            maxprot: file.read_i32::<E>()?,
            initprot: file.read_i32::<E>()?,
            nsects: file.read_u32::<E>()?,
            flags: file.read_u32::<E>()?,
        };

        Ok(SegmentCommand::SEG64(segment_command))
    }

    fn read_segname<R: Read>(file: &mut R) -> io::Result<[u8; 16]> {
        let mut segname = [0u8; 16];
        file.read_exact(&mut segname)?;
        Ok(segname)
    }
}

enum Section {
    SEC32(Section32),
    SEC64(Section64),
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

impl Section32 {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R) -> io::Result<Section32> {
        let section = Section32 {
            sectname: Self::read_sectname_or_segname(file)?,
            segname: Self::read_sectname_or_segname(file)?,
            addr: file.read_u32::<E>()?,
            size: file.read_u32::<E>()?,
            offset: file.read_u32::<E>()?,
            align: file.read_u32::<E>()?,
            reloff: file.read_u32::<E>()?,
            nreloc: file.read_u32::<E>()?,
            flags: file.read_u32::<E>()?,
            reserved1: file.read_u32::<E>()?,
            reserved2: file.read_u32::<E>()?,
        };
        Ok(section)
    }

    fn read_sectname_or_segname<R: Read>(file: &mut R) -> io::Result<[u8; 16]> {
        let mut sectname_or_segname = [0u8; 16];
        file.read_exact(&mut sectname_or_segname)?;
        Ok(sectname_or_segname)
    }
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

impl Section64 {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R) -> io::Result<Section64> {
        let section = Section64 {
            sectname: Self::read_sectname_or_segname(file)?,
            segname: Self::read_sectname_or_segname(file)?,
            addr: file.read_u64::<E>()?,
            size: file.read_u64::<E>()?,
            offset: file.read_u32::<E>()?,
            align: file.read_u32::<E>()?,
            reloff: file.read_u32::<E>()?,
            nreloc: file.read_u32::<E>()?,
            flags: file.read_u32::<E>()?,
            reserved1: file.read_u32::<E>()?,
            reserved2: file.read_u32::<E>()?,
            reserved3: file.read_u32::<E>()?,
        };
        Ok(section)
    }

    fn read_sectname_or_segname<R: Read>(file: &mut R) -> io::Result<[u8; 16]> {
        let mut sectname_or_segname = [0u8; 16];
        file.read_exact(&mut sectname_or_segname)?;
        Ok(sectname_or_segname)
    }
}

#[repr(C)]
pub struct Dylib {
    pub name: LcStr,
    pub timestamp: u32,
    pub current_version: u32,
    pub compatibility_version: u32,
}

impl Dylib {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R) -> io::Result<Dylib> {
        let dylib = Dylib {
            name: LcStr { offset: file.read_u32::<E>()? },
            timestamp: file.read_u32::<E>()?,
            current_version: file.read_u32::<E>()?,
            compatibility_version: file.read_u32::<E>()?,
        };
        Ok(dylib)
    }
}

#[repr(C)]
pub struct DylibCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub dylib: Dylib,
}

impl DylibCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<DylibCommand> {
        let dylib_command = DylibCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            dylib: Dylib::from_file::<R, E>(file)?,
        };
        Ok(dylib_command)
    }
}

#[repr(C)]
pub struct SubFrameWorkCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub umbrella: LcStr,
}

impl SubFrameWorkCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<SubFrameWorkCommand> {
        let sub_framework_command = SubFrameWorkCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            umbrella: LcStr { offset: file.read_u32::<E>()? },
        };
        Ok(sub_framework_command)
    }
}

#[repr(C)]
pub struct SubClientCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub client: LcStr,
}

impl SubClientCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<SubClientCommand> {
        let sub_client_command = SubClientCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            client: LcStr { offset: file.read_u32::<E>()? },
        };
        Ok(sub_client_command)
    }
}

#[repr(C)]
pub struct SubUmbrellaCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub sub_umbrella: LcStr,
}

impl SubUmbrellaCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<SubUmbrellaCommand> {
        let sub_umbrella_command = SubUmbrellaCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            sub_umbrella: LcStr { offset: file.read_u32::<E>()? },
        };
        Ok(sub_umbrella_command)
    }
}

#[repr(C)]
pub struct SubLibraryCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub sub_library: LcStr,
}

impl SubLibraryCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<SubLibraryCommand> {
        let sub_library_command = SubLibraryCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            sub_library: LcStr { offset: file.read_u32::<E>()? },
        };
        Ok(sub_library_command)
    }
}

#[repr(C)]
pub struct PreboundDylibCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub name: LcStr,
    pub nmodules: u32,
    pub linked_modules: LcStr,
}

impl PreboundDylibCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<PreboundDylibCommand> {
        let preboound_dylib_command = PreboundDylibCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            name: LcStr { offset: file.read_u32::<E>()? },
            nmodules: file.read_u32::<E>()?,
            linked_modules: LcStr { offset: file.read_u32::<E>()? },
        };
        Ok(preboound_dylib_command)
    }
}

#[repr(C)]
pub struct DylinkerCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub name: LcStr,
}

impl DylinkerCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<DylinkerCommand> {
        let dylinker_command = DylinkerCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            name: LcStr { offset: file.read_u32::<E>()? },
        };
        Ok(dylinker_command)
    }
}

#[repr(C)]
pub struct ThreadCommand {
    pub cmd: u32,
    pub cmdsize: u32,
}

impl ThreadCommand {
    pub fn from_file<E: byteorder::ByteOrder>(load_command: &LoadCommand) -> io::Result<ThreadCommand> {
        let thread_command = ThreadCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
        };
        Ok(thread_command)
    }
}

enum RoutinesCommand {
    RTN32(RoutinesCommand32),
    RTN64(RoutinesCommand64),
}

#[repr(C)]
pub struct RoutinesCommand32 {
    pub cmd: u32,
    pub cmdsize: u32,
    pub init_address: u32,
    pub init_module: u32,
    pub reserved1: u32,
    pub reserved2: u32,
    pub reserved3: u32,
    pub reserved4: u32,
    pub reserved5: u32,
    pub reserved6: u32,
}

impl RoutinesCommand32 {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<RoutinesCommand> {
        let routines_command = RoutinesCommand32 {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            init_address: file.read_u32::<E>()?,
            init_module: file.read_u32::<E>()?,
            reserved1: file.read_u32::<E>()?,
            reserved2: file.read_u32::<E>()?,
            reserved3: file.read_u32::<E>()?,
            reserved4: file.read_u32::<E>()?,
            reserved5: file.read_u32::<E>()?,
            reserved6: file.read_u32::<E>()?,
        };
        Ok(RoutinesCommand::RTN32(routines_command))
    }
}

#[repr(C)]
pub struct RoutinesCommand64 {
    pub cmd: u32,
    pub cmdsize: u32,
    pub init_address: u64,
    pub init_module: u64,
    pub reserved1: u64,
    pub reserved2: u64,
    pub reserved3: u64,
    pub reserved4: u64,
    pub reserved5: u64,
    pub reserved6: u64,
}

impl RoutinesCommand64 {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<RoutinesCommand> {
        let routines_command = RoutinesCommand64 {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            init_address: file.read_u64::<E>()?,
            init_module: file.read_u64::<E>()?,
            reserved1: file.read_u64::<E>()?,
            reserved2: file.read_u64::<E>()?,
            reserved3: file.read_u64::<E>()?,
            reserved4: file.read_u64::<E>()?,
            reserved5: file.read_u64::<E>()?,
            reserved6: file.read_u64::<E>()?,
        };
        Ok(RoutinesCommand::RTN64(routines_command))
    }
}

#[repr(C)]
pub struct SymtabCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub symoff: u32,
    pub nsyms: u32,
    pub stroff: u32,
    pub strsize: u32,
}

impl SymtabCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<SymtabCommand> {
        let symtab_command = SymtabCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            symoff: file.read_u32::<E>()?,
            nsyms: file.read_u32::<E>()?,
            stroff: file.read_u32::<E>()?,
            strsize: file.read_u32::<E>()?,
        };
        Ok(symtab_command)
    }
}

#[repr(C)]
pub struct DynSymtabCommand {
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

impl DynSymtabCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<DynSymtabCommand> {
        let dyn_symtab_command = DynSymtabCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            ilocalsym: file.read_u32::<E>()?,
            nlocalsym: file.read_u32::<E>()?,
            iextdefsym: file.read_u32::<E>()?,
            nextdefsym: file.read_u32::<E>()?,
            iundefsym: file.read_u32::<E>()?,
            nundefsym: file.read_u32::<E>()?,
            tocoff: file.read_u32::<E>()?,
            ntoc: file.read_u32::<E>()?,
            modtaboff: file.read_u32::<E>()?,
            nmodtab: file.read_u32::<E>()?,
            extrefsymoff: file.read_u32::<E>()?,
            nextrefsyms: file.read_u32::<E>()?,
            indirectsymoff: file.read_u32::<E>()?,
            nindirectsyms: file.read_u32::<E>()?,
            extreloff: file.read_u32::<E>()?,
            nextrel: file.read_u32::<E>()?,
            locreloff: file.read_u32::<E>()?,
            nlocrel: file.read_u32::<E>()?,
        };
        Ok(dyn_symtab_command)
    }
}

#[repr(C)]
pub struct DylibTableOfContents {
    pub symbol_index: u32,
    pub module_index: u32,
}

impl DylibTableOfContents {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R) -> io::Result<DylibTableOfContents> {
        let dylib_table_of_contents = DylibTableOfContents {
            symbol_index: file.read_u32::<E>()?,
            module_index: file.read_u32::<E>()?,
        };
        Ok(dylib_table_of_contents)
    }
}

enum DylibModule {
    DMD32(DylibModule32),
    DMD64(DylibModule64),
}

#[repr(C)]
pub struct DylibModule32 {
    pub module_name: u32,
    pub iextdefsym: u32,
    pub nextdefsym: u32,
    pub irefsym: u32,
    pub nrefsym: u32,
    pub ilocalsym: u32,
    pub nlocalsym: u32,
    pub iextrel: u32,
    pub nextrel: u32,
    pub iinit_iterm: u32,
    pub ninit_nterm: u32,
    pub objc_module_info_addr: u32,
    pub objc_module_info_size: u32,
}

impl DylibModule32 {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R) -> io::Result<DylibModule> {
        let dylib_module = DylibModule32 {
            module_name: file.read_u32::<E>()?,
            iextdefsym: file.read_u32::<E>()?,
            nextdefsym: file.read_u32::<E>()?,
            irefsym: file.read_u32::<E>()?,
            nrefsym: file.read_u32::<E>()?,
            ilocalsym: file.read_u32::<E>()?,
            nlocalsym: file.read_u32::<E>()?,
            iextrel: file.read_u32::<E>()?,
            nextrel: file.read_u32::<E>()?,
            iinit_iterm: file.read_u32::<E>()?,
            ninit_nterm: file.read_u32::<E>()?,
            objc_module_info_addr: file.read_u32::<E>()?,
            objc_module_info_size: file.read_u32::<E>()?,
        };
        Ok(DylibModule::DMD32(dylib_module))
    }
}

#[repr(C)]
pub struct DylibModule64 {
    pub module_name: u32,
    pub iextdefsym: u32,
    pub nextdefsym: u32,
    pub irefsym: u32,
    pub nrefsym: u32,
    pub ilocalsym: u32,
    pub nlocalsym: u32,
    pub iextrel: u32,
    pub nextrel: u32,
    pub iinit_iterm: u32,
    pub ninit_nterm: u32,
    pub objc_module_info_size: u32,
    pub objc_module_info_addr: u64,
}

impl DylibModule64 {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R) -> io::Result<DylibModule> {
        let dylib_module = DylibModule64 {
            module_name: file.read_u32::<E>()?,
            iextdefsym: file.read_u32::<E>()?,
            nextdefsym: file.read_u32::<E>()?,
            irefsym: file.read_u32::<E>()?,
            nrefsym: file.read_u32::<E>()?,
            ilocalsym: file.read_u32::<E>()?,
            nlocalsym: file.read_u32::<E>()?,
            iextrel: file.read_u32::<E>()?,
            nextrel: file.read_u32::<E>()?,
            iinit_iterm: file.read_u32::<E>()?,
            ninit_nterm: file.read_u32::<E>()?,
            objc_module_info_size: file.read_u32::<E>()?,
            objc_module_info_addr: file.read_u64::<E>()?,
        };
        Ok(DylibModule::DMD64(dylib_module))
    }
}

#[repr(C)]
pub struct DylibReference {
    pub isym: u32,
    pub flags: u8,
}

impl DylibReference {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R) -> io::Result<DylibReference> {
        let dylib_reference = DylibReference {
            isym: file.read_u32::<E>()?,
            flags: file.read_u8()?,
        };
        Ok(dylib_reference)
    }
}

#[repr(C)]
pub struct TwoLevelHintsCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub offset: u32,
    pub nhints: u32,
}

impl TwoLevelHintsCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<TwoLevelHintsCommand> {
        let two_level_hints_command = TwoLevelHintsCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            offset: file.read_u32::<E>()?,
            nhints: file.read_u32::<E>()?,
        };
        Ok(two_level_hints_command)
    }
}

#[repr(C)]
pub struct TwoLevelHint {
    pub isub_image: u8,
    pub itoc: u32,
}

impl TwoLevelHint {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R) -> io::Result<TwoLevelHint> {
        let two_level_hint = TwoLevelHint {
            isub_image: file.read_u8()?,
            itoc: file.read_u32::<E>()?,
        };
        Ok(two_level_hint)
    }
}

#[repr(C)]
pub struct PrebindCksumCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub cksum: u32,
}

impl PrebindCksumCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<PrebindCksumCommand> {
        let prebind_checksum_command = PrebindCksumCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            cksum: file.read_u32::<E>()?,
        };
        Ok(prebind_checksum_command)
    }
}

#[repr(C)]
pub struct UuidCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub uuid: [u8; 16],
}

impl UuidCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<UuidCommand> {
        let uuid_command = UuidCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            uuid: Self::read_uuid(file)?,
        };
        Ok(uuid_command)
    }

    fn read_uuid<R: Read>(file: &mut R) -> io::Result<[u8; 16]> {
        let mut uuid = [0u8; 16];
        file.read_exact(&mut uuid)?;
        Ok(uuid)
    }
}

#[repr(C)]
pub struct RpathCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub path: LcStr,
}

impl RpathCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<RpathCommand> {
        let rpath_command = RpathCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            path: LcStr { offset: file.read_u32::<E>()? },
        };
        Ok(rpath_command)
    }
}

#[repr(C)]
pub struct LinkeditDataCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub dataoff: u32,
    pub datasize: u32,
}

impl LinkeditDataCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<LinkeditDataCommand> {
        let linkedit_data_command = LinkeditDataCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            dataoff: file.read_u32::<E>()?,
            datasize: file.read_u32::<E>()?,
        };
        Ok(linkedit_data_command)
    }
}

enum EncryptionInfoCommand {
    ENI32(EncryptionInfoCommand32),
    ENI64(EncryptionInfoCommand64),
}

#[repr(C)]
pub struct EncryptionInfoCommand32 {
    pub cmd: u32,
    pub cmdsize: u32,
    pub cryptoff: u32,
    pub cryptsize: u32,
    pub cryptid: u32,
}

impl EncryptionInfoCommand32 {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<EncryptionInfoCommand> {
        let encryption_info_command = EncryptionInfoCommand32 {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            cryptoff: file.read_u32::<E>()?,
            cryptsize: file.read_u32::<E>()?,
            cryptid: file.read_u32::<E>()?,
        };
        Ok(EncryptionInfoCommand::ENI32(encryption_info_command))
    }
}

#[repr(C)]
pub struct EncryptionInfoCommand64 {
    pub cmd: u32,
    pub cmdsize: u32,
    pub cryptoff: u32,
    pub cryptsize: u32,
    pub cryptid: u32,
    pub pad: u32,
}

impl EncryptionInfoCommand64 {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<EncryptionInfoCommand> {
        let encryption_info_command = EncryptionInfoCommand64 {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            cryptoff: file.read_u32::<E>()?,
            cryptsize: file.read_u32::<E>()?,
            cryptid: file.read_u32::<E>()?,
            pad: file.read_u32::<E>()?,  // Additional padding field
        };
        Ok(EncryptionInfoCommand::ENI64(encryption_info_command))
    }
}

#[repr(C)]
pub struct VersionMinCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub version: u32,
    pub sdk: u32,
}

impl VersionMinCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<VersionMinCommand> {
        let version_min_command = VersionMinCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            version: file.read_u32::<E>()?,
            sdk: file.read_u32::<E>()?,
        };
        Ok(version_min_command)
    }
}

#[repr(C)]
pub struct BuildVersionCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub platform: u32,
    pub minos: u32,
    pub sdk: u32,
    pub ntools: u32,
}

impl BuildVersionCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<BuildVersionCommand> {
        let build_version_command = BuildVersionCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            platform: file.read_u32::<E>()?,
            minos: file.read_u32::<E>()?,
            sdk: file.read_u32::<E>()?,
            ntools: file.read_u32::<E>()?,
        };
        Ok(build_version_command)
    }
}

#[repr(C)]
pub struct DyldInfoCommand {
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

impl DyldInfoCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<DyldInfoCommand> {
        let dyld_info_command = DyldInfoCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            rebase_off: file.read_u32::<E>()?,
            rebase_size: file.read_u32::<E>()?,
            bind_off: file.read_u32::<E>()?,
            bind_size: file.read_u32::<E>()?,
            weak_bind_off: file.read_u32::<E>()?,
            weak_bind_size: file.read_u32::<E>()?,
            lazy_bind_off: file.read_u32::<E>()?,
            lazy_bind_size: file.read_u32::<E>()?,
            export_off: file.read_u32::<E>()?,
            export_size: file.read_u32::<E>()?,
        };
        Ok(dyld_info_command)
    }
}

#[repr(C)]
pub struct BuildToolVersion {
    pub tool: u32,
    pub version: u32,
}

impl BuildToolVersion {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R) -> io::Result<BuildToolVersion> {
        let build_toool_version = BuildToolVersion {
            tool: file.read_u32::<E>()?,
            version: file.read_u32::<E>()?,
        };
        Ok(build_toool_version)
    }
}

#[repr(C)]
pub struct LinkerOptionCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub count: u32,
}

impl LinkerOptionCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<LinkerOptionCommand> {
        let linker_option_command = LinkerOptionCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            count: file.read_u32::<E>()?,
        };
        Ok(linker_option_command)
    }
}

#[repr(C)]
pub struct SymsegCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub offset: u32,
    pub size: u32,
}

impl SymsegCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<SymsegCommand> {
        let symseg_command = SymsegCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            offset: file.read_u32::<E>()?,
            size: file.read_u32::<E>()?,
        };
        Ok(symseg_command)
    }
}

#[repr(C)]
pub struct IdentCommand {
    pub cmd: u32,
    pub cmdsize: u32,
}

impl IdentCommand {
    pub fn from_file<E: byteorder::ByteOrder>(load_command: &LoadCommand) -> io::Result<IdentCommand> {
        let ident_command = IdentCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
        };
        Ok(ident_command)
    }
}

#[repr(C)]
pub struct EntryPointCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub entryoff: u64,
    pub stacksize: u64,
}

impl EntryPointCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<EntryPointCommand> {
        let entry_point_command = EntryPointCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            entryoff: file.read_u64::<E>()?,
            stacksize: file.read_u64::<E>()?,
        };
        Ok(entry_point_command)
    }
}

#[repr(C)]
pub struct SourceVersionCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub version: u64,
}

impl SourceVersionCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<SourceVersionCommand> {
        let source_version_command = SourceVersionCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            version: file.read_u64::<E>()?,
        };
        Ok(source_version_command)
    }
}

#[repr(C)]
pub struct DataInCodeEntry {
    pub offset: u32,
    pub length: u16,
    pub kind: u16,
}

impl DataInCodeEntry {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R) -> io::Result<DataInCodeEntry> {
        let data_in_code_entry = DataInCodeEntry {
            offset: file.read_u32::<E>()?,
            length: file.read_u16::<E>()?,
            kind: file.read_u16::<E>()?,
        };
        Ok(data_in_code_entry)
    }
}

#[repr(C)]
pub struct TlvDescriptor {
    pub thunk: extern "C" fn(&mut TlvDescriptor),
    pub key: usize,
    pub offset: usize,
}

#[repr(C)]
pub struct NoteCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub data_owner: [u8; 16],
    pub offset: u64,
    pub size: u64,
}

impl NoteCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(file: &mut R, load_command: &LoadCommand) -> io::Result<NoteCommand> {
        let note_command = NoteCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            data_owner: Self::read_data_owner(file)?,
            offset: file.read_u64::<E>()?,
            size: file.read_u64::<E>()?,
        };
        Ok(note_command)
    }

    fn read_data_owner<R: Read>(file: &mut R) -> io::Result<[u8; 16]> {
        let mut data_owner = [0u8; 16];
        file.read_exact(&mut data_owner)?;
        Ok(data_owner)
    }
}

pub fn parse<R: Read>(file: &mut R) {
    let header =  parse_header(file);
    match header {
        Ok(header) => header,
        Err(e) => panic!("Error on header parsing: {}", e),
    };
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
            MachHeader::MH64(header) => {
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
