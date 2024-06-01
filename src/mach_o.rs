use crate::header::MachHeader;
use crate::load_commands::{LcStr, LoadCommand, Section};

// TODO: make it more generic. we shoudl make a pair of (Vec<LoadCommand>, Vec<extra_memory>).
// TODO: we encapsulte each extra_memory with its type by making and extrra_memory an enum (what to do with it?) and then we iterate and resolve! :D

// 1. create an enum for extra_memory and add all the possible options. extra_memory subtype should be a vector of u8.
// 2. make every load command that has extra memory after it read the chunk to a new extra_memory and append it to Vec<extra_memory>
// 3. after reading the load commands we immideatly start going over the chunks and parse them according to each type :D

pub struct MachO {
    pub header: MachHeader,
    pub load_commands: (Vec<LoadCommand>, Vec<Vec<Section>>, Vec<LcStr>),
}
