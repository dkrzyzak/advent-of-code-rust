use crate::read::read_input_file;

pub fn task() {
   let lines = read_input_file("y2015", "day1");
   let parens = lines.first().unwrap();

   let mut current_floor: i32 = 0;
   let mut char_index: u32 = 1;

   for paren in parens.chars() {
      if paren == '(' {
         current_floor += 1;
      }

      if paren == ')' {
         current_floor -= 1;
      }

      if current_floor == -1 {
         println!("Basement at char: {char_index}");
      }

      char_index += 1;
   }

   println!("Floor: {current_floor}");
}
