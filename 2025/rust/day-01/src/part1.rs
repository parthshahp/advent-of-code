use std::io::Error;

pub fn process(_input: &str) -> Result<String, Error> {
    let mut current = 50;
    let instructions: Vec<&str> = _input.split_whitespace().collect();
    let mut count = 0;

    for mut inst in instructions {
        let direction = inst.chars().nth(0).expect("Doesn't have direction");
        inst = inst.get(1..).expect("Has more");
        let amount: i32 = inst.parse().expect("Not a number");

        if direction == 'L' {
            current -= amount;
        } else if direction == 'R' {
            current += amount;
        }
        current %= 100;
        if current == 0 {
            count += 1;
        }
    }

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!("3", process(input).unwrap());
    }
}
