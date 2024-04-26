use std::io::{self, Read, Seek, SeekFrom};

pub fn make_reader_static<R: Read + Seek>(file: &mut R, size: usize) -> io::Result<Vec<u8>>  {
    let current_pos = file.stream_position()?;

    println!("current pos is {}", file.stream_position()?);

    let mut buffer = vec![0; size];
    file.read(&mut buffer)?;

    println!("current pos is {}", file.stream_position()?);

    file.seek(SeekFrom::Start(current_pos))?;

    println!("current pos is {}", file.stream_position()?);


    Ok(buffer)
}