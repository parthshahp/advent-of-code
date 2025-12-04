use crate::Result;

#[derive(Debug)]
struct Point(isize, isize);

const DIRECTIONS: [Point; 8] = [
    Point(-1, 0),
    Point(1, 0),
    Point(0, -1),
    Point(0, 1),
    Point(-1, -1),
    Point(-1, 1),
    Point(1, -1),
    Point(1, 1),
];

pub fn process(input: &str) -> Result<String> {
    let mut matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut total = 0;
    loop {
        let temp = remove(&mut matrix);
        if temp == 0 {
            break;
        }
        total += temp;
    }

    Ok(total.to_string())
}

fn remove(m: &mut Vec<Vec<char>>) -> i32 {
    let mut total = 0;
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] != '@' {
                continue;
            }
            let p = Point(i as isize, j as isize);
            if check_directions(&p, &m) {
                total += 1;
                m[i][j] = '.';
            }
        }
    }
    total
}

fn check_directions(p: &Point, m: &Vec<Vec<char>>) -> bool {
    let mut count = 0;
    for d in DIRECTIONS {
        let new_point = Point(p.0 + d.0, p.1 + d.1);
        if invalid_pos(&new_point, &m) {
            continue;
        }

        // Already checked the bounds in invalid_pos
        let x: usize = new_point.0 as usize;
        let y: usize = new_point.1 as usize;

        if m[x][y] == '@' {
            count += 1;
        }
    }

    count < 4
}

fn invalid_pos(p: &Point, m: &Vec<Vec<char>>) -> bool {
    if p.0 < 0 || p.1 < 0 {
        return true;
    }

    let x = p.0 as usize;
    let y = p.1 as usize;

    if x > m.len() {
        return true;
    }

    if y > m[x].len() {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2() {
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
