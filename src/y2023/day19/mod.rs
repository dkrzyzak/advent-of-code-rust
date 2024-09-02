use std::collections::HashMap;

use crate::parse_input;

mod part;
mod range;
mod workflow;
use part::*;
use range::*;
use workflow::*;

pub fn task() {
    let lines = parse_input!();
    let (workflows, _) = extract_input(&lines);
    let parts_range = PartRange::new();

    // let range = Range::new();
    // let (range1, range2) = range.split(500, "<");
    // println!("range1: {:?}, range2: {:?}", range1.bounds, range2.bounds);

    let mut total: usize = 0;
    run_pipeline(
        parts_range,
        workflows.get("in").unwrap(),
        &workflows,
        &mut total,
    );
    println!("Calculated total: {}", total);
}

pub fn run_pipeline(
    parts_range: PartRange,
    workflow: &Workflow,
    workflows: &HashMap<String, Workflow>,
    total: &mut usize,
) {
    let mut processed_range = parts_range;

    for rule in workflow.rules.iter() {
        match rule {
            Rule::Accept() => {
                *total += processed_range.total();
            }
            Rule::Reject() => {}
            Rule::Send(name) => {
                run_pipeline(
                    processed_range.clone(),
                    workflows.get(name).unwrap(),
                    workflows,
                    total,
                );
            }
            Rule::Condition(field_name, operation, compared_to, action) => {
                let (range_ok, range_bad) = processed_range.split(field_name, *compared_to, operation);
                processed_range = range_bad;

                match &**action {
                    Rule::Accept() => {
                        *total += range_ok.total();
                    }
                    Rule::Reject() => {}
                    Rule::Send(name) => {
                        run_pipeline(range_ok, workflows.get(name).unwrap(), workflows, total);
                    }
                    _ => unreachable!("Can't process this action"),
                }
            }
        }
    }
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

pub fn part1() {
    let lines = parse_input!();
    let (workflows, parts) = extract_input(&lines);

    let total = parts
        .iter()
        .map(|part| process_part(part, workflows.get("in").unwrap(), &workflows))
        .sum::<usize>();

    println!("Total: {total}");
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
                            return process_part(part, workflows.get(name).unwrap(), workflows)
                        }
                        _ => panic!("Can't process this action"),
                    }
                }
            }
        }
    }

    0
}
