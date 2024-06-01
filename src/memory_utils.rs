use std::io::{self, Read, Seek, SeekFrom};

pub fn get_file_offset<R: Read + Seek>(file: &mut R) -> io::Result<u64> {
    file.stream_position()
}

pub fn advance_to_next_load_command<R: Read + Seek>(file: &mut R, offset: u64, cmdsize: u64) -> io::Result<u64> {
    file.seek(SeekFrom::Start(offset + cmdsize))
}

