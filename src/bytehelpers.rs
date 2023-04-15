//helper for writing out those long sequences.
use std::io::Write;

pub fn write_byte_length<W: Write>(mut writer: W, mut value: u32) -> std::io::Result<()> 
{
   let mut buffer: [u8; 4] = [0; 4];
   let mut i = 0;
   loop {
       let mut b = (value & 0x7F) as u8; //set inital value b
       value >>= 7;
       if value > 0 { //check if we want to change
           b |= 0x80;
       }
       buffer[i] = b;
       i += 1;
       if value == 0 {
         //finished
           break;
       }
   }
   writer.write_all(&buffer[..i])
}
