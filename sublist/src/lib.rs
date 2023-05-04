#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        Comparison::Equal
    } else if _first_list.len() > _second_list.len() {
        let times = _first_list.len() - _second_list.len();
        if _second_list.is_empty() {
            return Comparison::Superlist;
        }
        for t in 0..=times {
            if _second_list == &_first_list[t..t + _second_list.len()] {
                return Comparison::Superlist;
            }
        }
        Comparison::Unequal
    } else if _first_list.len() < _second_list.len() {
        let times = _second_list.len() - _first_list.len();
        if _first_list.is_empty() {
            return Comparison::Sublist;
        }
        for t in 0..=times {
            if _first_list == &_second_list[t..t + _first_list.len()] {
                return Comparison::Sublist;
            }
        }
        Comparison::Unequal
    } else {
        Comparison::Unequal
    }

    // unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
}
