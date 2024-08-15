pub fn task() {
    let input = 29000000;
    // let input = 10;
    let mut houses = vec![0; input / 10];

    for i in 1..input / 10 {
        let mut delivered = 0;
        for j in (i..input / 10).step_by(i) {
            houses[j] += i * 11;
            delivered += 1;

            if delivered == 50 {
                break;
            }
        }
    }

    let mut first_house_meeting_criteria = 0;
    for i in 1..input / 10 {
        if houses[i] > input {
            first_house_meeting_criteria = i;
            break;
        }
    }

    println!(
        "First house meeting criteria: {}",
        first_house_meeting_criteria
    );
}
