#[derive(Debug)]
pub struct Rule(pub u32, pub u32);

impl Rule {
    pub fn is_valid_update(&self, update: &Vec<u32>) -> bool {
        let &Self(first, second) = self;

        if let (Some(position_first), Some(position_second)) = (
            update.iter().position(|&el| el == first),
            update.iter().position(|&el| el == second),
        ) {
            return position_first < position_second;
        } else {
            return true;
        }
    }
}

pub fn extract_input(lines: &Vec<String>) -> (Vec<Rule>, Vec<Vec<u32>>) {
    let mut iter = lines.iter();

    let mut ordering_rules = Vec::new();

    while let Some(line) = iter.next() {
        if line == "" {
            break;
        }

        let rule = line
            .split("|")
            .map(|item| item.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        ordering_rules.push(Rule(rule[0], rule[1]));
    }

    let mut updates = Vec::new();

    while let Some(line) = iter.next() {
        let update = line
            .split(",")
            .map(|item| item.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        updates.push(update);
    }

    (ordering_rules, updates)
}
