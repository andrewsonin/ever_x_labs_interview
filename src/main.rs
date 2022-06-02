use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::iter::once;
use std::ops::Range;

#[derive(Default)]
struct RangeCounter {
    best_start: usize,
    best_end: usize,
    cur_start: usize,
    cur_end: usize,
}

impl RangeCounter {
    fn on_brace_mismatch(&mut self, next_idx: usize) {
        self.update_best_range();
        self.cur_start = next_idx;
        self.cur_end = next_idx;
    }

    fn update_best_range(&mut self) {
        if self.cur_end - self.cur_start > self.best_end - self.best_start {
            self.best_start = self.cur_start;
            self.best_end = self.cur_end;
        }
    }

    fn shift_end(&mut self, next_idx: usize) {
        self.cur_end = next_idx;
    }

    fn get_best_range(&self) -> Range<usize> {
        self.best_start..self.best_end
    }
}

#[derive(Eq, PartialEq)]
enum PendingBracket {
    Round,
    Square,
    Curly,
}

fn get_longest_correct_cyclic_substring(s: impl AsRef<str>) -> String
{
    let s = s.as_ref();
    let s_len = s.len();
    if s_len != s.chars().count() {
        panic!("Input string should contain 1-byte symbols only. Got: {s}")
    }
    let triple_s = format!("{s}{s}{s}");

    let (mut round_sum, mut square_sum, mut curly_sum) = (0, 0, 0);
    let mut min_max_occurrences = HashMap::<(i64, i64, i64), (usize, usize)>::with_capacity(
        s_len * 3
    );
    min_max_occurrences.insert((0, 0, 0), (0, 0));
    for (char, idx) in triple_s.chars().zip(1usize..) {
        match char {
            '(' => round_sum += 1,
            ')' => round_sum -= 1,
            '[' => square_sum += 1,
            ']' => square_sum -= 1,
            '{' => curly_sum += 1,
            '}' => curly_sum -= 1,
            _ => {}
        }
        match min_max_occurrences.entry((round_sum, square_sum, curly_sum)) {
            Entry::Occupied(mut entry) => {
                let (_min_idx, max_idx) = entry.get_mut();
                *max_idx = idx
            }
            Entry::Vacant(entry) => { entry.insert((idx, idx)); }
        }
    }
    let (best_dist, best_left, best_right) = min_max_occurrences.values().fold(
        (0, 0, 0),
        |(mut best_dist, mut best_left, mut best_right), &(min_idx, max_idx)| {
            let cur_dist = max_idx - min_idx;
            if cur_dist > best_dist {
                (best_dist, best_left, best_right) = (cur_dist, min_idx, max_idx)
            }
            (best_dist, best_left, best_right)
        },
    );

    let result = if best_dist < s_len {
        &triple_s[best_left..best_right]
    } else {
        "Infinite"
    };
    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::get_longest_correct_cyclic_substring;

    #[test]
    fn example_01() {
        assert_eq!(get_longest_correct_cyclic_substring("}](){"), "(){}")
    }

    #[test]
    fn example_02() {
        assert_eq!(get_longest_correct_cyclic_substring("sh(dh)}"), "sh(dh)")
    }

    #[test]
    fn example_03() {
        assert_eq!(get_longest_correct_cyclic_substring("]h({hdb}b)["), "Infinite")
    }

    #[test]
    fn example_04() {
        assert_eq!(get_longest_correct_cyclic_substring("]h({hdb}b)[)"), "h({hdb}b)")
    }

    #[test]
    fn example_05() {
        assert_eq!(get_longest_correct_cyclic_substring(""), "Infinite")
    }

    #[test]
    fn example_06() {
        assert_eq!(get_longest_correct_cyclic_substring("aawfvwjnvdskHIUAWUndjea"), "Infinite")
    }

    #[test]
    fn example_07() {
        assert_eq!(get_longest_correct_cyclic_substring("dh()wed(())"), "Infinite")
    }

    #[test]
    fn example_08() {
        assert_eq!(get_longest_correct_cyclic_substring("dh()wed(())]"), "dh()wed(())")
    }

    #[test]
    fn example_09() {
        assert_eq!(get_longest_correct_cyclic_substring("[dh()wed(())[]"), "dh()wed(())[]")
    }
}

fn main() {
    println!("{}", get_longest_correct_cyclic_substring("}](){"));
    println!("{}", get_longest_correct_cyclic_substring("sh(dh)}"));
    println!("{}", get_longest_correct_cyclic_substring("]h({hdb}b)["));
}
