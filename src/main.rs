extern crate midly;
use std::fs::File;
use std::io::Write;
use std::io::Seek;

use byteorder::{WriteBytesExt};

fn main() -> std::io::Result<()> {
   let mut file = File::create("output.mid")?;

   // Write the track length
   let track_length = file.seek(std::io::SeekFrom::Current(0))? - track_length_pos - 4;
   file.seek(std::io::SeekFrom::Start(track_length_pos))?;
   file.write_u32::<byteorder::BigEndian>(track_length as u32)?;

   Ok(())
}
