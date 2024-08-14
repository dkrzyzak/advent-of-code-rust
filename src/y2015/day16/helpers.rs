use regex::Regex;

#[derive(Debug)]
pub struct Aunt {
    pub id: u16,
    pub children: Option<u8>,
    pub cats: Option<u8>,
    pub samoyeds: Option<u8>,
    pub pomeranians: Option<u8>,
    pub akitas: Option<u8>,
    pub vizslas: Option<u8>,
    pub goldfish: Option<u8>,
    pub trees: Option<u8>,
    pub cars: Option<u8>,
    pub perfumes: Option<u8>,
}

impl Default for Aunt {
    fn default() -> Self {
        Aunt {
            id: 0,
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        }
    }
}

pub fn parse_aunt(s: &str) -> Aunt {
    let re = Regex::new(r"Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)").unwrap();
    let captured = re.captures(s).unwrap();
    let id = captured[1].parse::<u16>().unwrap();
    let param1 = captured[2].parse::<String>().unwrap();
    let val1 = captured[3].parse::<u8>().unwrap();
    let param2 = captured[4].parse::<String>().unwrap();
    let val2 = captured[5].parse::<u8>().unwrap();
    let param3 = captured[6].parse::<String>().unwrap();
    let val3 = captured[7].parse::<u8>().unwrap();

    let params = vec![(param1, val1), (param2, val2), (param3, val3)];
    let mut aunt = Aunt::default();
    aunt.id = id;

    for (param_name, param_value) in params {
        match param_name.as_str() {
            "children" => aunt.children = Some(param_value),
            "cats" => aunt.cats = Some(param_value),
            "samoyeds" => aunt.samoyeds = Some(param_value),
            "pomeranians" => aunt.pomeranians = Some(param_value),
            "akitas" => aunt.akitas = Some(param_value),
            "vizslas" => aunt.vizslas = Some(param_value),
            "goldfish" => aunt.goldfish = Some(param_value),
            "trees" => aunt.trees = Some(param_value),
            "cars" => aunt.cars = Some(param_value),
            "perfumes" => aunt.perfumes = Some(param_value),
            _ => {
                println!("invalid key: {param_name}")
            }
        }
    }

    aunt
}
