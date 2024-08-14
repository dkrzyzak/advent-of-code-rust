use regex::Regex;

pub fn extract_matrix_from_input(input: &Vec<String>, people_count: usize) -> (Vec<Vec<i16>>, Vec<String>) {
    let mut names: Vec<String> = Vec::new();
    let mut happiness_matrix: Vec<Vec<i16>> = Vec::new();
    
    let mut current_name = String::new();
    let mut current_index: isize = -1;

    let re = Regex::new(r"^(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).").unwrap();
    for line in input.iter() {
        let captured = re.captures(line).unwrap();
        let name = captured[1].parse::<String>().unwrap();
        let gain_or_lose = captured[2].parse::<String>().unwrap();
        let absolute_points = captured[3].parse::<i16>().unwrap();
        let neighbour = captured[4].parse::<String>().unwrap();

        if !names.contains(&name) {
            names.push(name.clone());
        }

        if !names.contains(&neighbour) {
            names.push(neighbour.clone());
        }

        if current_name != name {
            current_name = name;
            current_index += 1;
            let happiness_vec: Vec<i16> = vec![0; people_count];
            happiness_matrix.push(happiness_vec);
        }

        let neighbour_index = names.iter().position(|n| *n == neighbour).unwrap();
        let points = if gain_or_lose == "gain" { absolute_points } else { -absolute_points };
        happiness_matrix[current_index as usize][neighbour_index] = points;
    }

    (happiness_matrix, names)
}