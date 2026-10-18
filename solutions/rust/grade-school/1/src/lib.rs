use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
// struct Student {
//     name: String,
//     grade: u8,
// }

// impl Hash for Student {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.name.hash(state);
//     }
// }

pub struct School {
    students: HashMap<String, u32>,
}

impl School {
    pub fn new() -> School {
        Self {
            students: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.entry(student.to_string()).or_insert(grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        let g: HashSet<u32> = HashSet::from_iter(self.students.values().copied());
        let mut v = g.into_iter().collect::<Vec<_>>();
        v.sort();

        v
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students: Vec<String> = self
            .students
            .iter()
            .filter_map(|(k, v)| (grade == *v).then_some(k.clone()))
            .collect();
        students.sort();

        students
    }
}
