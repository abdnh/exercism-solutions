#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let first_bigger = first_list.len() >= second_list.len();
    let (list1, list2) = if first_bigger {
        (first_list, second_list)
    } else {
        (second_list, first_list)
    };
    for i in 0..list1.len() {
        let mut j = i;
        let mut k = 0;
        let mut equal = true;
        while j < list1.len() && k < list2.len() {
            if list1[j] != list2[k] {
                equal = false;
                break;
            }
            j += 1;
            k += 1;
        }
        if !equal {
            continue;
        }
        if j == list1.len() && k == list2.len() {
            if i == 0 {
                return Comparison::Equal;
            } else if first_bigger {
                return Comparison::Superlist;
            } else {
                return Comparison::Sublist;
            }
        }
        if k == list2.len() {
            if first_bigger {
                return Comparison::Superlist;
            } else {
                return Comparison::Sublist;
            };
        }
    }

    if first_list.is_empty() && second_list.is_empty() {
        Comparison::Equal
    } else {
        Comparison::Unequal
    }
}
