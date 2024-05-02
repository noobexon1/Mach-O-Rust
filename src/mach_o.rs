use crate::header::MachHeader;
use crate::load_commands::LoadCommand;

// todo: add load_commands_string (LCstr)

pub struct MachO {
    pub header: MachHeader,
    pub load_commands: Vec<LoadCommand>,
}
