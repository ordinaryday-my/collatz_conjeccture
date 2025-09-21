pub fn generate_collatz_sequence(start: usize) -> Result<(Vec<usize>, usize), String> {
    #[derive(Clone, Copy)]
    struct State {
        current: usize,
        one_count: usize,
    }

    let mut seq_iter = std::iter::from_fn({
        let mut state: State = State {
            current: start,
            one_count: 0,
        };
        let mut overflowed = false;

        move || {
            if overflowed {
                return None;
            }

            let next_val = if state.current % 2 != 0 {
                3usize
                    .checked_mul(state.current)
                    .and_then(|x| x.checked_add(1))
            } else {
                Some(state.current / 2)
            };

            match next_val {
                Some(val) => {
                    let new_count_1 = if val == 1 {
                        state.one_count + 1
                    } else {
                        state.one_count
                    };
                    if new_count_1 > 40 {
                        None
                    } else {
                        state.one_count = new_count_1;
                        state.current = val;
                        Some(Ok(val))
                    }
                }
                None => {
                    overflowed = true;
                    Some(Err("Overflow occurred".to_string()))
                },
            }
        }
    });

    seq_iter.try_fold((Vec::new(), 0usize), |(mut vec, count), val| {
        match val {
            Ok(v) => {
                vec.push(v);
                Ok((vec, count + 1))
            }
            Err(e) => Err(e),
        }
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_collatz_sequence() {
        let (seq, steps) = super::generate_collatz_sequence(6).unwrap();
        assert_eq!(steps, 8);
        assert_eq!(seq, vec![3, 10, 5, 16, 8, 4, 2, 1]);
    }
}