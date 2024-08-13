use std::{fs::File, io::Read, path::Path};

// pub fn read_input_file(year: &str, subfolder_name: &str) -> Vec<String> {
//     let pathname = format!("src/{year}/{subfolder_name}/input");
//     let file_path = Path::new(&pathname);
//     let mut file = File::open(file_path).expect("Cant open file");

//     let mut content = String::new();
//     file.read_to_string(&mut content)
//         .expect("Cant read to string");

//     let lines = content.lines().map(String::from).collect();

//     lines
// }

pub fn read_input_file(folder_path: &Path) -> Vec<String> {
    let pathname = Path::new(folder_path).join("input");
    let mut file = File::open(pathname).expect("There is no input file in current directory");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Cant read to string");

    let lines = content.lines().map(String::from).collect();

    lines
}
