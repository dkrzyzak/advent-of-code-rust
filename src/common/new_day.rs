use std::{env, fs, io};

pub fn resolve_args(args: &Vec<String>) -> Option<bool> {
    if args.len() < 2 {
        return None;
    }

    // `cargo run -- -new 2022 5`
    if args[1] == "-new" {
        if args.len() < 4 {
            println!("Missing arguments. You need to provide value for year and day: `cargo day 2022 5`");
            return Some(false);
        }

        let year = args[2].parse::<u32>().expect("Invalid year argument");
        let day = args[3].parse::<u32>().expect("Invalid day argument");

        if let Ok(()) = create_new_day(year, day) {
            println!("Created day {} in year {}", day, year);
            return Some(true);
        } else {
            println!("Unable to create new day folder and files...");
            return Some(false);
        }
    }

    return None;
}

pub fn create_new_day(year: u32, day: u32) -> io::Result<()> {
    let mut path = env::current_dir()?;
    path.push("src");
    path.push(format!("y{}", year));
    path.push(format!("day{}", day));

    // Create /src/y2022/day5 folder
    fs::create_dir_all(&path)?;

    let files = ["input", "input_real", "input_simple", "mod.rs"];
    for file in &files {
        let mut file_path = path.clone();
        file_path.push(*file);

        // create empty /src/y2022/day5/mod.rs file
        fs::File::create(file_path)?;
    }

    Ok(())
}
