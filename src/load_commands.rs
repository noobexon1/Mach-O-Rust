use std::io;
use std::io::Read;

use byteorder::ReadBytesExt;

pub enum LoadCommand {
    SegmentCommand(SegmentCommand),
    DylibCommand(DylibCommand),
    SubFrameWorkCommand(SubFrameWorkCommand),
    SubClientCommand(SubClientCommand),
    SubUmbrellaCommand(SubUmbrellaCommand),
    SubLibraryCommand(SubLibraryCommand),
    PreboundDylibCommand(PreboundDylibCommand),
    DylinkerCommand(DylinkerCommand),
    ThreadCommand(ThreadCommand),
    RoutinesCommand(RoutinesCommand),
    SymtabCommand(SymtabCommand),
    DynSymtabCommand(DynSymtabCommand),
    TwoLevelHintsCommand(TwoLevelHintsCommand),
    PrebindCksumCommand(PrebindCksumCommand),
    UuidCommand(UuidCommand),
    RpathCommand(RpathCommand),
    LinkeditDataCommand(LinkeditDataCommand),
    EncryptionInfoCommand(EncryptionInfoCommand),
    VersionMinCommand(VersionMinCommand),
    BuildVersionCommand(BuildVersionCommand),
    DyldInfoCommand(DyldInfoCommand),
    LinkerOptionCommand(LinkerOptionCommand),
    SymsegCommand(SymsegCommand),
    IdentCommand(IdentCommand),
    EntryPointCommand(EntryPointCommand),
    SourceVersionCommand(SourceVersionCommand),
    NoteCommand(NoteCommand),
}

#[derive(Debug)]
#[repr(C)]
pub struct LoadCommandPrefix {
    pub cmd: u32,
    pub cmdsize: u32,
}

impl LoadCommandPrefix {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
    ) -> io::Result<LoadCommandPrefix> {
        let load_command = LoadCommandPrefix {
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

#[derive(Debug)]
pub enum SegmentCommand {
    SEG32(SegmentCommand32),
    SEG64(SegmentCommand64),
}

#[derive(Debug)]
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
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
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

        Ok(LoadCommand::SegmentCommand(SegmentCommand::SEG32(
            segment_command,
        )))
    }

    fn read_segname<R: Read>(file: &mut R) -> io::Result<[u8; 16]> {
        let mut segname = [0u8; 16];
        file.read_exact(&mut segname)?;
        Ok(segname)
    }
}

#[derive(Debug)]
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
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
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

        Ok(LoadCommand::SegmentCommand(SegmentCommand::SEG64(
            segment_command,
        )))
    }

    fn read_segname<R: Read>(file: &mut R) -> io::Result<[u8; 16]> {
        let mut segname = [0u8; 16];
        file.read_exact(&mut segname)?;
        Ok(segname)
    }
}

#[derive(Debug)]
pub enum Section {
    SEC32(Section32),
    SEC64(Section64),
}

#[derive(Debug)]
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

#[derive(Debug)]
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
            name: LcStr {
                offset: file.read_u32::<E>()?,
            },
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
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let dylib_command = DylibCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            dylib: Dylib::from_file::<R, E>(file)?,
        };
        Ok(LoadCommand::DylibCommand(dylib_command))
    }
}

#[repr(C)]
pub struct SubFrameWorkCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub umbrella: LcStr,
}

impl SubFrameWorkCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let sub_framework_command = SubFrameWorkCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            umbrella: LcStr {
                offset: file.read_u32::<E>()?,
            },
        };
        Ok(LoadCommand::SubFrameWorkCommand(sub_framework_command))
    }
}

#[repr(C)]
pub struct SubClientCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub client: LcStr,
}

impl SubClientCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let sub_client_command = SubClientCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            client: LcStr {
                offset: file.read_u32::<E>()?,
            },
        };
        Ok(LoadCommand::SubClientCommand(sub_client_command))
    }
}

#[repr(C)]
pub struct SubUmbrellaCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub sub_umbrella: LcStr,
}

impl SubUmbrellaCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let sub_umbrella_command = SubUmbrellaCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            sub_umbrella: LcStr {
                offset: file.read_u32::<E>()?,
            },
        };
        Ok(LoadCommand::SubUmbrellaCommand(sub_umbrella_command))
    }
}

#[repr(C)]
pub struct SubLibraryCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub sub_library: LcStr,
}

impl SubLibraryCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let sub_library_command = SubLibraryCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            sub_library: LcStr {
                offset: file.read_u32::<E>()?,
            },
        };
        Ok(LoadCommand::SubLibraryCommand(sub_library_command))
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
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let preboound_dylib_command = PreboundDylibCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            name: LcStr {
                offset: file.read_u32::<E>()?,
            },
            nmodules: file.read_u32::<E>()?,
            linked_modules: LcStr {
                offset: file.read_u32::<E>()?,
            },
        };
        Ok(LoadCommand::PreboundDylibCommand(preboound_dylib_command))
    }
}

#[repr(C)]
pub struct DylinkerCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub name: LcStr,
}

impl DylinkerCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let dylinker_command = DylinkerCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            name: LcStr {
                offset: file.read_u32::<E>()?,
            },
        };
        Ok(LoadCommand::DylinkerCommand(dylinker_command))
    }
}

#[derive(Debug)]
#[repr(C)]
//TODO: modify this according to the instructions here: https://opensource.apple.com/source/xnu/xnu-4903.221.2/EXTERNAL_HEADERS/mach-o/loader.h.auto.html
pub struct ThreadCommand {
    pub cmd: u32,
    pub cmdsize: u32,
}

impl ThreadCommand {
    pub fn from_file<E: byteorder::ByteOrder>(
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let thread_command = ThreadCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
        };
        Ok(LoadCommand::ThreadCommand(thread_command))
    }
}

#[derive(Debug)]
pub enum RoutinesCommand {
    RTN32(RoutinesCommand32),
    RTN64(RoutinesCommand64),
}

#[derive(Debug)]
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
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
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
        Ok(LoadCommand::RoutinesCommand(RoutinesCommand::RTN32(
            routines_command,
        )))
    }
}

#[derive(Debug)]
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
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
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
        Ok(LoadCommand::RoutinesCommand(RoutinesCommand::RTN64(
            routines_command,
        )))
    }
}

#[derive(Debug)]
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
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let symtab_command = SymtabCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            symoff: file.read_u32::<E>()?,
            nsyms: file.read_u32::<E>()?,
            stroff: file.read_u32::<E>()?,
            strsize: file.read_u32::<E>()?,
        };
        Ok(LoadCommand::SymtabCommand(symtab_command))
    }
}

#[derive(Debug)]
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
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
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
        Ok(LoadCommand::DynSymtabCommand(dyn_symtab_command))
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct DylibTableOfContents {
    pub symbol_index: u32,
    pub module_index: u32,
}

impl DylibTableOfContents {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
    ) -> io::Result<DylibTableOfContents> {
        let dylib_table_of_contents = DylibTableOfContents {
            symbol_index: file.read_u32::<E>()?,
            module_index: file.read_u32::<E>()?,
        };
        Ok(dylib_table_of_contents)
    }
}

#[derive(Debug)]
pub enum DylibModule {
    DMD32(DylibModule32),
    DMD64(DylibModule64),
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
#[repr(C)]
pub struct TwoLevelHintsCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub offset: u32,
    pub nhints: u32,
}

impl TwoLevelHintsCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let two_level_hints_command = TwoLevelHintsCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            offset: file.read_u32::<E>()?,
            nhints: file.read_u32::<E>()?,
        };
        Ok(LoadCommand::TwoLevelHintsCommand(two_level_hints_command))
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
#[repr(C)]
pub struct PrebindCksumCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub cksum: u32,
}

impl PrebindCksumCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let prebind_checksum_command = PrebindCksumCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            cksum: file.read_u32::<E>()?,
        };
        Ok(LoadCommand::PrebindCksumCommand(prebind_checksum_command))
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct UuidCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub uuid: [u8; 16],
}

impl UuidCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let uuid_command = UuidCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            uuid: Self::read_uuid(file)?,
        };
        Ok(LoadCommand::UuidCommand(uuid_command))
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
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let rpath_command = RpathCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            path: LcStr {
                offset: file.read_u32::<E>()?,
            },
        };
        Ok(LoadCommand::RpathCommand(rpath_command))
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct LinkeditDataCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub dataoff: u32,
    pub datasize: u32,
}

impl LinkeditDataCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let linkedit_data_command = LinkeditDataCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            dataoff: file.read_u32::<E>()?,
            datasize: file.read_u32::<E>()?,
        };
        Ok(LoadCommand::LinkeditDataCommand(linkedit_data_command))
    }
}

#[derive(Debug)]
pub enum EncryptionInfoCommand {
    ENI32(EncryptionInfoCommand32),
    ENI64(EncryptionInfoCommand64),
}

#[derive(Debug)]
#[repr(C)]
pub struct EncryptionInfoCommand32 {
    pub cmd: u32,
    pub cmdsize: u32,
    pub cryptoff: u32,
    pub cryptsize: u32,
    pub cryptid: u32,
}

impl EncryptionInfoCommand32 {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let encryption_info_command = EncryptionInfoCommand32 {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            cryptoff: file.read_u32::<E>()?,
            cryptsize: file.read_u32::<E>()?,
            cryptid: file.read_u32::<E>()?,
        };
        Ok(LoadCommand::EncryptionInfoCommand(
            EncryptionInfoCommand::ENI32(encryption_info_command),
        ))
    }
}

#[derive(Debug)]
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
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let encryption_info_command = EncryptionInfoCommand64 {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            cryptoff: file.read_u32::<E>()?,
            cryptsize: file.read_u32::<E>()?,
            cryptid: file.read_u32::<E>()?,
            pad: file.read_u32::<E>()?, // Additional padding field
        };
        Ok(LoadCommand::EncryptionInfoCommand(
            EncryptionInfoCommand::ENI64(encryption_info_command),
        ))
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct VersionMinCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub version: u32,
    pub sdk: u32,
}

impl VersionMinCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let version_min_command = VersionMinCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            version: file.read_u32::<E>()?,
            sdk: file.read_u32::<E>()?,
        };
        Ok(LoadCommand::VersionMinCommand(version_min_command))
    }
}

#[derive(Debug)]
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
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let build_version_command = BuildVersionCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            platform: file.read_u32::<E>()?,
            minos: file.read_u32::<E>()?,
            sdk: file.read_u32::<E>()?,
            ntools: file.read_u32::<E>()?,
        };
        Ok(LoadCommand::BuildVersionCommand(build_version_command))
    }
}

#[derive(Debug)]
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
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
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
        Ok(LoadCommand::DyldInfoCommand(dyld_info_command))
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct BuildToolVersion {
    pub tool: u32,
    pub version: u32,
}

impl BuildToolVersion {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
    ) -> io::Result<BuildToolVersion> {
        let build_toool_version = BuildToolVersion {
            tool: file.read_u32::<E>()?,
            version: file.read_u32::<E>()?,
        };
        Ok(build_toool_version)
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct LinkerOptionCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub count: u32,
}

impl LinkerOptionCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let linker_option_command = LinkerOptionCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            count: file.read_u32::<E>()?,
        };
        Ok(LoadCommand::LinkerOptionCommand(linker_option_command))
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct SymsegCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub offset: u32,
    pub size: u32,
}

impl SymsegCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let symseg_command = SymsegCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            offset: file.read_u32::<E>()?,
            size: file.read_u32::<E>()?,
        };
        Ok(LoadCommand::SymsegCommand(symseg_command))
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct IdentCommand {
    pub cmd: u32,
    pub cmdsize: u32,
}

impl IdentCommand {
    pub fn from_file<E: byteorder::ByteOrder>(
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let ident_command = IdentCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
        };
        Ok(LoadCommand::IdentCommand(ident_command))
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct EntryPointCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub entryoff: u64,
    pub stacksize: u64,
}

impl EntryPointCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let entry_point_command = EntryPointCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            entryoff: file.read_u64::<E>()?,
            stacksize: file.read_u64::<E>()?,
        };
        Ok(LoadCommand::EntryPointCommand(entry_point_command))
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct SourceVersionCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub version: u64,
}

impl SourceVersionCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let source_version_command = SourceVersionCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            version: file.read_u64::<E>()?,
        };
        Ok(LoadCommand::SourceVersionCommand(source_version_command))
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct DataInCodeEntry {
    pub offset: u32,
    pub length: u16,
    pub kind: u16,
}

impl DataInCodeEntry {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
    ) -> io::Result<DataInCodeEntry> {
        let data_in_code_entry = DataInCodeEntry {
            offset: file.read_u32::<E>()?,
            length: file.read_u16::<E>()?,
            kind: file.read_u16::<E>()?,
        };
        Ok(data_in_code_entry)
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct TlvDescriptor {
    pub thunk: extern "C" fn(&mut TlvDescriptor),
    pub key: usize,
    pub offset: usize,
}

#[derive(Debug)]
#[repr(C)]
pub struct NoteCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub data_owner: [u8; 16],
    pub offset: u64,
    pub size: u64,
}

impl NoteCommand {
    pub fn from_file<R: Read, E: byteorder::ByteOrder>(
        file: &mut R,
        load_command: &LoadCommandPrefix,
    ) -> io::Result<LoadCommand> {
        let note_command = NoteCommand {
            cmd: load_command.cmd,
            cmdsize: load_command.cmdsize,
            data_owner: Self::read_data_owner(file)?,
            offset: file.read_u64::<E>()?,
            size: file.read_u64::<E>()?,
        };
        Ok(LoadCommand::NoteCommand(note_command))
    }

    fn read_data_owner<R: Read>(file: &mut R) -> io::Result<[u8; 16]> {
        let mut data_owner = [0u8; 16];
        file.read_exact(&mut data_owner)?;
        Ok(data_owner)
    }
}
