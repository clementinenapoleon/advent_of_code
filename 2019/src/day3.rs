type InputType = String;

enum WirePath {
    U(usize),
    D(usize),
    L(usize),
    R(usize)
}

fn convert(wire_location: InputType) -> WirePath {
    WirePath::U(1)
}

/// What is the Manhattan distance from the central port to the closest intersection?
fn manhattan_distance_from_central_port_to_nearest_intersection(wire_locations: Vec<InputType>) -> usize {
    let wire_paths: Vec<WirePath> = wire_locations.iter().map(|x| {convert(x.clone())}).collect();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(manhattan_distance_from_central_port_to_nearest_intersection(simple_data()), 0)
    }

    fn simple_data() -> Vec<InputType> {
        let input: Vec<InputType> = vec![];
        return input;
    }

    #[test]
    fn f() {
        assert_eq!(manhattan_distance_from_central_port_to_nearest_intersection(data()), 0)
    }

    fn data() -> Vec<InputType> {
        let input: Vec<InputType> = vec![];
        return input;
    }
}
