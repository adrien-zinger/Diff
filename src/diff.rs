use std::vec::Vec;
type Diff<T> = Vec<(u8, u32, u32, Vec<T>, Vec<T>)>;

fn levenstein_algo<T: std::cmp::PartialEq>(s: &[T], t: &[T]) -> Vec<Vec<u8>> {
    let mut distance = vec![vec![0; t.len()]; s.len()];
    let mut operation = vec![vec![0_u8; t.len()]; s.len()];
    (1..s.len()).for_each(|i| {
        distance[i][0] = i;
    }); 
    for j in 1..t.len() {
        distance[0][j] = j;
    }
    (1..t.len()).for_each(|j| {
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
    });
    operation
}

fn get_diff<T: std::cmp::PartialEq + std::clone::Clone + std::marker::Copy>(
    source: &[T],
    target: &[T],
) -> Diff<T> {
    let o = levenstein_algo(source, target);
    let mut i = source.len() - 1;
    let mut j = target.len() - 1;
    let mut ret = Vec::new();
    let mut raw = Vec::new();
    let mut raw_sub = Vec::new();
    let mut raw_type = o[i][j];
    while i > 0 && j > 0 {
        if raw_type != o[i][j] {
            if !(raw.is_empty() && raw_sub.is_empty()) {
                ret.push((
                    raw_type,
                    (i + 1) as u32,
                    raw.len() as u32,
                    raw.drain(..).collect(),
                    raw_sub.drain(..).collect(),
                ));
            }
            raw_type = o[i][j];
        }
        if o[i][j] == 0 {
            if source[i] != target[j] {
                raw_type = o[i][j];
                raw.push(source[i]);
                raw_sub.push(target[j]);
            } else if !(raw.is_empty() && raw_sub.is_empty()) {
                ret.push((
                    raw_type,
                    (i + 1) as u32,
                    raw.len() as u32,
                    raw.drain(..).collect(),
                    raw_sub.drain(..).collect(),
                ));
            }
            j -= 1;
            i -= 1;
        } else if o[i][j] == 1 {
            // insert
            raw_type = o[i][j];
            raw.push(target[j]);
            j -= 1;
        } else if o[i][j] == 2 {
            // delete
            raw_type = o[i][j];
            raw.push(source[i]);
            i -= 1;
        }
    }
    if !(raw.is_empty() && raw_sub.is_empty()) {
        ret.push((raw_type, (i + 1) as u32, raw.len() as u32, raw, raw_sub));
    }
    ret
}

pub fn diff<T: std::cmp::PartialEq + std::clone::Clone + std::marker::Copy>(
    source: &[T],
    target: &[T],
) -> Diff<T> {
    get_diff(source, target)
}
