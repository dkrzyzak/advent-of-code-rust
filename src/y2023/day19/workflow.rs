use crate::parse_captures;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref WORKFLOW_REG: Regex = Regex::new(r"(\w+)\{([^}]+)\}").unwrap();
    static ref CONDITION_REG: Regex = Regex::new(r"([xmas])([<>])(\d+):(\w+)").unwrap();
}

#[derive(Debug)]
pub struct Workflow {
    pub name: String,
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub enum Rule {
    Accept(),
    Reject(),
    Send(String),
    Condition(String, String, usize, Box<Rule>),
}

impl Workflow {
    pub fn parse(from: &String) -> Workflow {
        let (name, rules_all) = parse_captures!(WORKFLOW_REG, from, 1 String, 2 String);
        let rules = rules_all.split(",").map(|s| parse_rule(s)).collect();

        Workflow { name, rules }
    }
}

pub fn parse_rule(rule_str: &str) -> Rule {
    if rule_str == "A" {
        return Rule::Accept();
    }

    if rule_str == "R" {
        return Rule::Reject();
    }

    if !rule_str.contains(":") {
        return Rule::Send(rule_str.to_string());
    }

    let (field, operation, compared_to, action) =
        parse_captures!(CONDITION_REG, rule_str, 1 String, 2 String, 3 usize, 4 String);

    let parsed_action = match action.as_str() {
        "A" => Rule::Accept(),
        "R" => Rule::Reject(),
        _ => Rule::Send(action),
    };

    Rule::Condition(field, operation, compared_to, Box::new(parsed_action))
}
