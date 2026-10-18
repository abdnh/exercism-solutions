static STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let Some(position) = STUDENTS.iter().position(|s| *s == student) else {
        panic!("unexpected student name: {}", student);
    };
    let mut plants: Vec<&'static str> = vec![];
    for line in diagram.lines() {
        for (j, chr) in line.chars().enumerate() {
            if j / 2 == position {
                let plant = match chr {
                    'C' => "clover",
                    'G' => "grass",
                    'R' => "radishes",
                    'V' => "violets",
                    _ => panic!("invalid diagram: {}", chr),
                };
                plants.push(plant);
            }
        }
    }

    plants
}
