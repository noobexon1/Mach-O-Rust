use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

// TODO: the problem is that i need SEEK as well as Read. we will see how to so that.
pub fn read_static(file: &mut File, size: usize) -> io::Result<Vec<u8>>  {
    let current_pos = file.stream_position()?;

    println!("current pos is {}", file.stream_position()?);

    let mut buffer = vec![0; size];
    file.read(&mut buffer)?;

    println!("current pos is {}", file.stream_position()?);

    file.seek(SeekFrom::Start(current_pos))?;

    println!("current pos is {}", file.stream_position()?);


    Ok(buffer)
}