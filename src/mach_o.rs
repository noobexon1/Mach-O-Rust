use crate::header::MachHeader;
use crate::load_commands::{LcStr, LoadCommand, Section};
use crate::symbols::Symtab;

pub struct MachO {
    pub header: Option<MachHeader>,
    pub load_commands: Option<(Vec<LoadCommand>, Vec<Vec<Section>>, Vec<LcStr>)>,
    pub symtab: Option<Symtab>
}

impl MachO {
    pub fn new() -> Self {
        MachO {
            header: None,
            load_commands: None,
            symtab: None,
        }
    }
}
