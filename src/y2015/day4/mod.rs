use md5::{Digest, Md5};

pub fn task() {
    let input = "yzbqklnj";
    let mut i: u32 = 1;

    loop {
        let mut hasher = Md5::new();
        let phrase = format!("{input}{i}");
        hasher.update(phrase);

        let result = hasher.finalize();

        let formatted = format!("{:x}", result);

        let first_six = &formatted[0..6];

        if first_six == "000000" {
            println!("{formatted}");
            println!("i: {i}");
            break;
        }

        i += 1;
    }
}
