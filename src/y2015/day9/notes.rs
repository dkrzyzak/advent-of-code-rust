// BRUTE-FORCE APPROACH
    let mut shortest_route = u32::MAX;

    let all_cities: Vec<&String> = distances.keys().clone().collect();
    let mut all_permutations: Vec<Vec<&String>> = Vec::new();

    let mut routes: Vec<Vec<&String>> = distances
        .keys()
        .map(|city| {
            let mut route: Vec<&String> = Vec::new();
            route.push(city);
            return route;
        })
        .collect();

    for mut route in routes {
        calculate_permutations(&mut route, &mut all_permutations, &all_cities);
    }


fn calculate_permutations(
    route: &mut Vec<&String>,
    all_permutations: &mut Vec<Vec<&String>>,
    all_cities: &Vec<&String>,
) {
    if route.len() == all_cities.len() {
        all_permutations.push(route);
    } else {
        let unvisited_city = all_cities.iter().find(|city| route.contains(city)).unwrap();
        route.push(unvisited_city);
        calculate_permutations(route, all_permutations, all_cities);
    }
}