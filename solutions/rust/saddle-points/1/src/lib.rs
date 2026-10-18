pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    // Find largest in row
    let mut largest_indices_for_row: Vec<Vec<usize>> = vec![];
    for row in input {
        let max_height = row.iter().max();
        if let Some(max_height) = max_height {
            let mut row_indices = vec![];
            for (idx, height) in row.iter().enumerate() {
                if height == max_height {
                    row_indices.push(idx);
                }
            }
            largest_indices_for_row.push(row_indices);
        } else {
            // Empty row
            largest_indices_for_row.push(vec![]);
        }
    }

    let mut good_spots = vec![];

    // Find smallest in column
    for (i, _) in input.iter().enumerate() {
        'outer: for j in largest_indices_for_row[i].iter().copied() {
            for k in 0..input.len() {
                if i != k && input[k][j] < input[i][j] {
                    // Not smallest
                    continue 'outer;
                }
            }
            // Smallest
            good_spots.push((i, j));
        }
    }

    good_spots
}
