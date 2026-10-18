pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut annotated_garden: Vec<String> = vec![];
    for (i, row) in garden.iter().copied().enumerate() {
        let row = row.as_bytes();
        let mut annotated_row: Vec<_> = row.into();
        for (j, chr) in row.iter().copied().enumerate() {
            if chr != b' ' {
                annotated_row[j] = chr;
                continue;
            }
            let mut count = 0;
            if j != 0 && row[j - 1] != b' ' {
                count += 1;
            }
            if j != row.len() - 1 && row[j + 1] != b' ' {
                count += 1;
            }
            if i != 0 {
                let previous_row = garden[i - 1].as_bytes();
                if previous_row[j] != b' ' {
                    count += 1;
                }
                if j != 0 && previous_row[j - 1] != b' ' {
                    count += 1;
                }
                if j != row.len() - 1 && previous_row[j + 1] != b' ' {
                    count += 1;
                }
            }
            if i != garden.len() - 1 {
                let next_row = garden[i + 1].as_bytes();
                if next_row[j] != b' ' {
                    count += 1;
                }

                if j != 0 && next_row[j - 1] != b' ' {
                    count += 1;
                }
                if j != row.len() - 1 && next_row[j + 1] != b' ' {
                    count += 1;
                }
            }
            if count != 0 {
                annotated_row[j] = b'0' + count;
            }
        }

        annotated_garden.push(String::from_utf8_lossy(&annotated_row).into());
    }

    annotated_garden
}
