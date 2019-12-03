

fn calculate_total_fuel(input: Vec<isize>) -> isize {
    let mut sum = 0;

    for i in input {
        let mut prefuel: isize = i;

        let mut postfuel: isize = calculate_fuel(prefuel);

        while postfuel > 0 {
            sum += postfuel;
            prefuel = postfuel;
            postfuel = calculate_fuel(prefuel);
        }
    }

    println!("Required fuel is {}", sum);
    return sum;
}

fn calculate_fuel(a: isize) -> isize {
    let fuel = (a / 3) - 2;
    if fuel > 0 {
        return fuel;
    } else {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(super::calculate_total_fuel(data()), 5010664);
    }

    fn data() -> Vec<isize> {
        let input: Vec<isize> = vec![126317,
                                     64620,
                                     139485,
                                     77772,
                                     104110,
                                     103781,
                                     62566,
                                     76265,
                                     122125,
                                     54244,
                                     113039,
                                     142451,
                                     118677,
                                     54302,
                                     143001,
                                     81938,
                                     110142,
                                     115486,
                                     128100,
                                     81258,
                                     126461,
                                     81557,
                                     147850,
                                     138259,
                                     73839,
                                     96284,
                                     149078,
                                     59289,
                                     125691,
                                     102718,
                                     142591,
                                     110725,
                                     56164,
                                     76729,
                                     133956,
                                     140321,
                                     57104,
                                     125483,
                                     115962,
                                     52370,
                                     74447,
                                     121430,
                                     96347,
                                     116793,
                                     76514,
                                     60089,
                                     113431,
                                     66670,
                                     120534,
                                     117547,
                                     113552,
                                     131513,
                                     118405,
                                     85212,
                                     57049,
                                     118644,
                                     54743,
                                     95142,
                                     58559,
                                     85522,
                                     73832,
                                     141441,
                                     97836,
                                     98818,
                                     104272,
                                     100048,
                                     99266,
                                     97766,
                                     115778,
                                     51066,
                                     132499,
                                     129931,
                                     119368,
                                     91101,
                                     139165,
                                     106488,
                                     105597,
                                     66166,
                                     117561,
                                     94670,
                                     123877,
                                     63389,
                                     70293,
                                     79754,
                                     105288,
                                     128328,
                                     130873,
                                     54200,
                                     120704,
                                     57043,
                                     71478,
                                     133049,
                                     102096,
                                     82797,
                                     62972,
                                     121906,
                                     77277,
                                     97183,
                                     112739,
                                     135590];
        return input;
    }
}
