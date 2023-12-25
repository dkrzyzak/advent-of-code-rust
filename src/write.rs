use std::{fs::File, io::BufWriter};
use std::io::prelude::*;

pub fn write_to_file(data: Vec<Vec<char>>, filename: &str) -> std::io::Result<()> {
   let file = File::create(filename)?;
   let mut writer = BufWriter::new(file);

   for line in data {
      let s: String = line.into_iter().collect();
      writer.write_all(s.as_bytes())?;
      writer.write_all(b"\n")?;
   }

   Ok(())
}