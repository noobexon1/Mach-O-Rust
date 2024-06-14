use crate::header::MachHeader;
use crate::load_commands::{LcStr, LoadCommand, Section};

pub struct MachO {
    pub header: Option<MachHeader>,
    pub load_commands: Option<(Vec<LoadCommand>, Vec<Vec<Section>>, Vec<LcStr>)>,
}

impl MachO {
    pub fn new() -> Self {
        MachO {
            header: None,
            load_commands: None,
        }
    }
}
