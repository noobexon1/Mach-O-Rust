use crate::header::MachHeader;
use crate::load_commands::{LcStr, LoadCommand};

pub struct MachO {
    pub header: MachHeader,
    pub load_commands: (Vec<LoadCommand>, Vec<LcStr>),
}
