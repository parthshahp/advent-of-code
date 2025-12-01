use std::io::{Error, ErrorKind};

#[derive(PartialEq)]
enum Turn {
    Left,
    Right,
}

impl TryFrom<char> for Turn {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value == 'L' {
            return Ok(Turn::Left);
        } else if value == 'R' {
            return Ok(Turn::Right);
        }
        Err(())
    }
}

struct Instruction {
    dir: Turn,
    steps: i32,
}

pub fn process(_input: &str) -> Result<String, Error> {
    let mut current: i32 = 50;
    let instructions: Vec<Instruction> = _input
        .lines()
        .map(parse_instructions)
        .collect::<Result<Vec<_>, _>>()?;
    let mut count = 0;

    for inst in instructions {
        if inst.dir == Turn::Left {
            if current == 0 {
                count -= 1
            }
            current -= inst.steps;
        } else if inst.dir == Turn::Right {
            current += inst.steps;
        }
        if current <= 0 {
            count += -current / 100 + 1;
        } else {
            count += current / 100;
        }
        current = current.rem_euclid(100);
    }

    Ok(count.to_string())
}

fn parse_instructions(line: &str) -> Result<Instruction, Error> {
    let mut chars = line.chars();
    let dir_char = chars
        .next()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "missing direction"))?;
    let direction = Turn::try_from(dir_char)
        .map_err(|_| Error::new(ErrorKind::InvalidData, "invalid direction"))?;
    let amount: i32 = chars
        .as_str()
        .parse()
        .map_err(|_| Error::new(ErrorKind::InvalidData, "invalid distance"))?;

    Ok(Instruction {
        dir: direction,
        steps: amount,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2() {
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
        assert_eq!("6", process(input).unwrap());
    }
}
