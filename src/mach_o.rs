use crate::header::MachHeader;
use crate::load_commands::{LcStr, LoadCommand, Section};

pub struct MachO {
    pub header: MachHeader,
    pub load_commands: (Vec<LoadCommand>, Vec<Vec<Section>>, Vec<LcStr>),
}
