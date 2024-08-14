// names: &Vec<String>
pub fn print_happiness_matrix(matrix: &Vec<Vec<i16>>, names: &Vec<String>) {
    let longest_name = names.iter().map(|n| n.len()).max().unwrap();
    let total_line_len = longest_name + 2 + 6 * names.len();
    
    let mut header_line = String::new();
    header_line += &" ".repeat(longest_name + 1);
    header_line += "|";

    for name in names {
        let first_letter = name.chars().nth(0).unwrap();
        let column_header = format!("{:^5}|", first_letter);
        header_line += &column_header;
    }

    println!("{header_line}");

    for i in 0..names.len() {
        let row = &matrix[i];
        let name = &names[i];

        let name = format!("{:width$}", name, width = longest_name);
        let points: String = row.iter().map(|value| format!("{:>4} |", value)).collect();
        println!("{name} |{points}");
    }

    println!("{}\n", "-".repeat(total_line_len));
}
