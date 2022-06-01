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

    let mut bracket_stack = Vec::with_capacity(s_len * 3);
    let mut range_counter = RangeCounter::default();
    for (symbol, next_idx) in triple_s.chars().zip(1usize..) {
        match symbol {
            ')' => {
                if bracket_stack.pop() != Some(PendingBracket::Round) {
                    range_counter.on_brace_mismatch(next_idx);
                    continue;
                }
            }
            ']' => {
                if bracket_stack.pop() != Some(PendingBracket::Square) {
                    range_counter.on_brace_mismatch(next_idx);
                    continue;
                }
            }
            '}' => {
                if bracket_stack.pop() != Some(PendingBracket::Curly) {
                    range_counter.on_brace_mismatch(next_idx);
                    continue;
                }
            }
            '(' => bracket_stack.push(PendingBracket::Round),
            '[' => bracket_stack.push(PendingBracket::Square),
            '{' => bracket_stack.push(PendingBracket::Curly),
            _ => {}
        }
        if bracket_stack.is_empty() {
            range_counter.shift_end(next_idx);
            range_counter.update_best_range()
        }
    }

    let best_substring_range = range_counter.get_best_range();
    let result = if best_substring_range.len() < s_len {
        &triple_s[best_substring_range]
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
