#[derive(Debug)]
enum Direction {
    Right,
    Bottom,
    Left,
    Top,
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix: Vec<Vec<u32>> = (0..size)
        .map(|_| {
            let mut v: Vec<u32> = (0..size).collect();
            v.fill(0);
            v
        })
        .collect();
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut counter = 1;
    let mut direction = Direction::Right;
    let size = size as usize;
    loop {
        if i >= size || j >= size {
            break;
        }
        matrix[i][j] = counter;
        if counter == (size * size) as u32 {
            break;
        }
        counter += 1;
        match direction {
            Direction::Right => {
                j += 1;
                if j == size || matrix[i][j] != 0 {
                    direction = Direction::Bottom;
                    j -= 1;
                    i += 1;
                }
            }
            Direction::Bottom => {
                i += 1;
                if i == size || matrix[i][j] != 0 {
                    direction = Direction::Left;
                    i -= 1;
                    j -= 1;
                }
            }
            Direction::Left => {
                if j == 0 || matrix[i][j - 1] != 0 {
                    direction = Direction::Top;
                    i -= 1;
                } else {
                    j -= 1;
                }
            }
            Direction::Top => {
                if i == 0 || matrix[i - 1][j] != 0 {
                    direction = Direction::Right;
                    j += 1;
                } else {
                    i -= 1;
                }
            }
        }
    }

    matrix
}
