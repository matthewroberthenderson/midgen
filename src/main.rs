extern crate midly;
use std::fs::File;
use std::io::Write;
use std::io::Seek;

use byteorder::{WriteBytesExt};

mod bytehelpers;
use bytehelpers::{write_byte_length};


fn main() -> std::io::Result<()> {
   let mut file = File::create("output.mid")?;

   // Write the MIDI header
   file.write_all(b"MThd")?; // Chunk type MThd think SOF but for midi.
   file.write_u32::<byteorder::BigEndian>(6)?; // Chunk length
   file.write_u16::<byteorder::BigEndian>(1)?; // MIDI file type (single track)
   file.write_u16::<byteorder::BigEndian>(1)?; // Number of tracks
   file.write_u16::<byteorder::BigEndian>(480)?; // Ticks per quarter note

   // Write the track header
   file.write_all(b"MTrk")?; // Chunk type
   let track_length_pos = file.seek(std::io::SeekFrom::Current(0))?;
   file.write_u32::<byteorder::BigEndian>(0)?; // Placeholder for track length

   // Write some MIDI events
   write_byte_length(&file,0)?; // Delta time
   file.write_u8(0xC0)?; // Program change message
   file.write_u8(0)?; // Program number
   write_byte_length(&file,0)?;// Delta time
   file.write_u8(0x90)?; // Note on message
   file.write_u8(60)?; // Note number
   file.write_u8(100)?; // Velocity

   write_byte_length(&file,480)?; // Delta time
   file.write_u8(0x80)?; // Note off message
   file.write_u8(60)?; // Note number
   file.write_u8(0)?; // Velocity

   // Write the track length
   let track_length = file.seek(std::io::SeekFrom::Current(0))? - track_length_pos - 4;
   file.seek(std::io::SeekFrom::Start(track_length_pos))?;
   file.write_u32::<byteorder::BigEndian>(track_length as u32)?;

   Ok(())
}
