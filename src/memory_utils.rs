use std::io::{self, Cursor, Read, Seek, SeekFrom};

pub fn get_memory_slice<R: Read + Seek>(file: &mut R, size: usize, advance_seek: bool) -> io::Result<Cursor<Vec<u8>>> {
    match advance_seek {
        true => {
            let mut buffer = vec![0; size];
            file.read(&mut buffer)?;
            Ok(Cursor::new(buffer))
        }
        false => {
            let current_pos = file.stream_position()?;
            let mut buffer = vec![0; size];
            file.read(&mut buffer)?;
            file.seek(SeekFrom::Start(current_pos))?;
            Ok(Cursor::new(buffer))
        }
    }
}

pub fn get_file_offset<R: Read + Seek>(file: &mut R) -> io::Result<u64> {
    file.stream_position()
}

pub fn advance_to_next_load_command<R: Read + Seek>(file: &mut R, offset: u64, cmdsize: u64) -> io::Result<u64> {
    file.seek(SeekFrom::Start(offset + cmdsize))
}

