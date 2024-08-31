use std::collections::HashMap;

use crate::{common::print_by_line::PrintByLine, parse_input};

mod part;
mod workflow;
use part::*;
use workflow::*;

pub fn task() {
    let lines = parse_input!();
    let (workflows, parts) = extract_input(&lines);

    // workflows.print_by_line();
    // parts.print_by_line();

    let total = parts.iter().map(|part| {
        process_part(part, workflows.get("in").unwrap(), &workflows)
    }).sum::<usize>();

    println!("Total: {total}");
}

pub fn extract_input(lines: &Vec<String>) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();
    let mut iter = lines.iter();

    // parsing workflows
    while let Some(line) = iter.next() {
        if line == "" {
            // empty line means we are done with workflows
            break;
        }

        let workflow = Workflow::parse(line);
        workflows.insert(workflow.name.clone(), workflow);
    }

    // parsing parts
    while let Some(line) = iter.next() {
        let part = Part::parse(line);
        parts.push(part);
    }

    (workflows, parts)
}

pub fn process_part(
    part: &Part,
    workflow: &Workflow,
    workflows: &HashMap<String, Workflow>,
) -> usize {
    for rule in workflow.rules.iter() {
        match rule {
            Rule::Accept() => return part.sum(),
            Rule::Reject() => return 0,
            Rule::Send(name) => return process_part(part, workflows.get(name).unwrap(), workflows),
            Rule::Condition(field_name, operation, compared_to, action) => {
                let value = part.get_key(&field_name);
                let result = if operation == ">" {
                    value > *compared_to
                } else {
                    value < *compared_to
                };

                if result {
                    match &**action {
                        Rule::Accept() => return part.sum(),
                        Rule::Reject() => return 0,
                        Rule::Send(name) => {
                            println!("{}, {:?}", name, action);
                            return process_part(part, workflows.get(name).unwrap(), workflows)
                        },
                        _ => panic!("Can't process this action")
                    }
                }
            }
        }
    }

    0
}
