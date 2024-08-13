use regex::Regex;
use serde_json::Value;

use crate::parse_input;

pub fn task() {
    let lines = parse_input!();
    let json_str = lines.first().unwrap();

    let reg = Regex::new(r"(\-?\d+)").unwrap();
    let total_1: i32 = reg
        .captures_iter(json_str)
        .filter_map(|capture| {
            let captured = capture.get(1).unwrap();
            let number = captured.as_str().parse::<i32>().ok();

            number
        })
        .sum();

    println!("Total: {total_1}");

    let parsed_json: Value = serde_json::from_str(json_str).unwrap();
    let mut total2: i64 = 0;
    sum_nums(&parsed_json, &mut total2);
    println!("Total without red: {total2}");

}

fn sum_nums(value: &Value, total: &mut i64)  {
    match value {
        Value::Number(num) => {
            if let Some(t) = num.as_i64() {
                *total += t;
            }
        }
        Value::Array(arr) => {
            for item in arr {
                sum_nums(item, total);
            }
        }
        Value::Object(obj) => {
            let mut has_red = false;
            for item in obj.values() {
                match item {
                    Value::String(s) => {
                        if s == "red" {
                            has_red = true;
                        }
                    }
                    _ => {}
                }
            }

            if !has_red {
                for item in obj.values() {
                    sum_nums(item, total);
                }
            }

            
            
        }
        _ => {}
    }
}
