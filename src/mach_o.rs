use crate::header::MachHeader;
use crate::load_commands::{LcStr, LoadCommand, Section};

//TODO: use Option<T> to init mach-o struct at the beginning with empty values and then pass it to the parser instead of the header and load_commands at the beginning...

pub struct MachO {
    pub header: MachHeader,
    pub load_commands: (Vec<LoadCommand>, Vec<Vec<Section>>, Vec<LcStr>),
}
