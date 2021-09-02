use std::vec::Vec;

fn levenstein<T: std::cmp::PartialEq>(s: &[T], t: &[T]) -> Vec<Vec<u8>> {
    let mut distance = vec![vec![0; t.len()]; s.len()];
    let mut operation = vec![vec![0_u8; t.len()]; s.len()];
    for i in 1..s.len() {
        distance[i][0] = i;
    }
    for j in 1..t.len() {
        distance[0][j] = j;
    }
    for j in 1..t.len() {
        for i in 1..s.len() {
            let substitution_cost = if s[i] == t[j] { 0 } else { 1 };
            let deletion = distance[i - 1][j] + 1;
            let insertion = distance[i][j - 1] + 1;
            let substitution = distance[i - 1][j - 1] + substitution_cost;
            distance[i][j] = if substitution <= insertion && substitution <= deletion {
                operation[i][j] = 0;
                substitution
            } else if insertion < substitution && insertion < deletion {
                operation[i][j] = 1;
                insertion
            } else if deletion < substitution && deletion < insertion {
                operation[i][j] = 2;
                deletion
            } else {
                operation[i][j] = 0;
                substitution
            }
        }
    }
    operation
}

pub fn diff<T: std::cmp::PartialEq + std::clone::Clone + std::marker::Copy + std::fmt::Debug>(
    source: &[T],
    target: &[T],
) -> Vec<(u8, usize, Vec<T>, Vec<T>)> {
    let o = levenstein(source, target);
    let mut i = source.len() - 1;
    let mut j = target.len() - 1;
    let mut ret = Vec::new();
    let mut raw = Vec::new();
    let mut raw_sub = Vec::new();
    let mut raw_type = 3_u8;
    while i > 0 && j > 0 {
        if o[i][j] == 0 {
            if source[i] != target[j] {
                // Substitute
                if raw_type != 0 {
                    if !(raw.is_empty() && raw_sub.is_empty()) {
                        ret.push((
                            raw_type,
                            i + 1,
                            raw.drain(..).collect(),
                            raw_sub.drain(..).collect(),
                        ));
                    }
                    raw_type = 0;
                }
                raw.push(source[i]);
                raw_sub.push(target[j]);
            }
            j -= 1;
            i -= 1;
        } else if o[i][j] == 1 {
            // insert
            if raw_type != 1 {
                if !(raw.is_empty() && raw_sub.is_empty()) {
                    ret.push((
                        raw_type,
                        i + 1,
                        raw.drain(..).collect(),
                        raw_sub.drain(..).collect(),
                    ));
                }
                raw.drain(..);
                raw_sub.drain(..);
                raw_type = 1;
            }
            raw.push(target[j]);
            i -= 1;
        } else if o[i][j] == 2 {
            // delete
            if raw_type != 2 {
                if !(raw.is_empty() && raw_sub.is_empty()) {
                    ret.push((
                        raw_type,
                        i + 1,
                        raw.drain(..).collect(),
                        raw_sub.drain(..).collect(),
                    ));
                }
                raw.drain(..);
                raw_sub.drain(..);
                raw_type = 2;
            }
            raw.push(target[j]);
            j -= 1;
        }
    }
    if !(raw.is_empty() && raw_sub.is_empty()) {
        ret.push((raw_type, i + 1, raw, raw_sub));
    }
    ret
}