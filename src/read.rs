use std::{path::Path, fs::File, io::Read};

pub fn read_input_file(year: &str, subfolder_name: &str) -> Vec<String> {
   let pathname = format!("src/{year}/{subfolder_name}/input");
   let file_path = Path::new(&pathname);
    let mut file = File::open(file_path).expect("Cant open file");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Cant read to string");

    let lines = content.lines().map(String::from).collect();

    lines
}
