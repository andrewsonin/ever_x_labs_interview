fn get_idx_and_max<T: Ord + Copy>(values: &[T]) -> Option<(usize, T)> {
    values
        .iter()
        .enumerate()
        .max_by_key(|(_idx, &val)| val)
        .map(|(idx, val)| (idx, *val))
}

fn get_longest_correct_cyclic_substring(s: &str) -> String
{
    let s_len = s.len();
    if s_len != s.chars().count() {
        panic!("Input string should contain 1-byte symbols only. Got: {s}")
    }
    let double_s = format!("{s}{s}");

    let mut num_correct_by_index = vec![0; s_len * 2];
    if let Some(s) = double_s.chars().nth(0) {
        if s.is_alphabetic() {
            num_correct_by_index[0] = 1;
        }
    }
    for (idx, symbol) in double_s.chars().enumerate().skip(1) {
        if symbol.is_alphabetic() {
            num_correct_by_index[idx] = num_correct_by_index[idx - 1] + 1
        } else {
            let matching_brace = match symbol {
                '(' | '[' | '{' => continue,
                ')' => '(',
                ']' => '[',
                '}' => '{',
                _ => panic!("Does not expect such a symbol: {symbol}")
            };
            let prev_idx = idx - 1;
            let prev_num_correct = num_correct_by_index[prev_idx];
            if prev_idx < prev_num_correct {
                continue;
            }
            let idx_to_compare = prev_idx - prev_num_correct;
            if double_s.as_bytes()[idx_to_compare] as char == matching_brace {
                num_correct_by_index[idx] = 2 + prev_num_correct + num_correct_by_index[idx_to_compare.saturating_sub(1)];
            }
        }
    }

    let (idx, max) = get_idx_and_max(&num_correct_by_index).unwrap_or_default();

    let best_substring_range = (idx + 1 - max)..(idx + 1);
    let result = if best_substring_range.len() < s_len {
        &double_s[best_substring_range]
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

    #[test]
    fn example_10() {
        assert_eq!(get_longest_correct_cyclic_substring("[dh()wEd(([))[]"), "dh()wEd")
    }

    #[test]
    fn example_11() {
        assert_eq!(get_longest_correct_cyclic_substring("a"), "Infinite")
    }

    #[test]
    fn example_12() {
        assert_eq!(get_longest_correct_cyclic_substring("[dh()([))[]ewvrewwedwe"), "[]ewvrewwedwe")
    }

    #[test]
    fn example_13() {
        assert_eq!(get_longest_correct_cyclic_substring("][dh[))[]ewvrewwedwe["), "[]ewvrewwedwe[]")
    }

    #[test]
    fn example_14() {
        assert_eq!(get_longest_correct_cyclic_substring("()(({}[](][{[()]}]{})))("), "[{[()]}]{}")
    }

    #[test]
    fn example_15() {
        assert_eq!(get_longest_correct_cyclic_substring("()(({}[](][{[()]}]{})))("), "[{[()]}]{}")
    }

    #[test]
    fn example_16() {
        assert_eq!(get_longest_correct_cyclic_substring("()(({}[]([{[()]}]{})))("), "()(({}[]([{[()]}]{})))")
    }

    #[test]
    fn example_17() {
        assert_eq!(get_longest_correct_cyclic_substring("{}[()()()()()()()]"), "Infinite")
    }

    #[test]
    fn example_18() {
        assert_eq!(get_longest_correct_cyclic_substring("a()[[[[a()()a()[[[[a"), "a()()a()")
    }

    #[test]
    fn example_19() {
        assert_eq!(get_longest_correct_cyclic_substring("a()[[[[a()()a()[[[[arrfff"), "arrfffa()")
    }

    #[test]
    fn example_20() {
        assert_eq!(get_longest_correct_cyclic_substring("acb)()jn)"), "()jn")
    }

    #[test]
    fn example_21() {
        assert_eq!(get_longest_correct_cyclic_substring("a()(({}[]([{[()]}]{})))("), "a()(({}[]([{[()]}]{})))")
    }

    #[test]
    fn example_22() {
        assert_eq!(get_longest_correct_cyclic_substring("()(({}[]([{[()]}]{})))(b"), "b()(({}[]([{[()]}]{})))")
    }
}

fn main() {}