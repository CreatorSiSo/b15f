use std::io::{stdin, stdout, Write};

use b15f::B15fDriver;

fn main() -> Result<(), &'static str> {
    let mut driver = B15fDriver::new()?;

    print!("Number of inpus [1..=8]: ");
    stdout().flush().unwrap();

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let num_inputs: u8 = line.trim().parse().unwrap();

    let truth_table: Vec<(u8, u8)> = bool_combinations(num_inputs)
        .into_iter()
        .map(|input| {
            driver.digital_write_0(input);
            (input, driver.digital_read_0())
        })
        .collect();

    println!("   Input | Output");
    println!("---------|---------");
    for (input, output) in truth_table {
        println!("{input:08b} | {output:08b}");
    }

    Ok(())
}

fn bool_combinations(num_inputs: u8) -> Vec<u8> {
    let num_combinations = usize::pow(2, num_inputs as u32);
    let mut combinations = Vec::with_capacity(num_combinations);
    combinations.extend_from_slice(&[0, 1]);

    for _ in 1..num_inputs {
        for combination in &mut combinations {
            *combination <<= 1;
        }
        combinations.extend(
            combinations
                .clone()
                .iter()
                .map(|combination| combination | 1),
        );
    }

    combinations
}

#[test]
fn test_bool_combinations() {
    assert_eq!(bool_combinations(1), vec![0, 1]);
    assert_eq!(bool_combinations(2), vec![00, 10, 01, 11]);
    assert_eq!(
        bool_combinations(3),
        vec![000, 100, 010, 110, 001, 101, 011, 111]
    );
    assert_eq!(
        bool_combinations(4),
        vec![
            0000, 1000, 0100, 1100, 0010, 1010, 0110, 1110, 0001, 1001, 0101, 1101, 0011, 1011,
            0111, 1111
        ]
    );
}
