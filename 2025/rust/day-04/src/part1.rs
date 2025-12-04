use crate::Result;

#[derive(Debug)]
struct Point(i32, i32);

pub fn process(input: &str) -> Result<String> {
    let mut total = 0;
    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] != '@' {
                continue;
            }
            let p = Point(i.try_into()?, j.try_into()?);
            if check_directions(&p, &matrix)? {
                total += 1;
            }
        }
    }

    return Ok(total.to_string());
}

fn check_directions(p: &Point, m: &Vec<Vec<char>>) -> Result<bool> {
    let directions = vec![
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    let mut count = 0;
    for d in directions {
        let new_point = Point(p.0 + d.0, p.1 + d.1);
        if invalid_pos(&new_point, &m)? {
            continue;
        }
        let x: usize = new_point.0.try_into()?;
        let y: usize = new_point.1.try_into()?;
        if m[x][y] == '@' {
            count += 1;
        }
    }

    return Ok(count < 4);
}

fn invalid_pos(p: &Point, m: &Vec<Vec<char>>) -> Result<bool> {
    if p.0 >= m[0].len().try_into()? || p.1 >= m.len().try_into()? || p.0 < 0 || p.1 < 0 {
        return Ok(true);
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!("13", process(input).unwrap());
    }
}
