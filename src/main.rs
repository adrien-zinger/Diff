use std::vec::Vec;

fn levenstein<T: std::cmp::PartialEq>(s: &[T], t: &[T]) -> Vec::<Vec::<u8>> {
    let mut distance = vec![vec![0; t.len()]; s.len()];
    let mut operation = vec![vec![0_u8; t.len()]; s.len()];
    for i in 1..s.len() { distance[i][0] = i; }
    for j in 1..t.len() { distance[0][j] = j; }
    for j in 1..s.len() - 1 {
        for i in 1..t.len() - 1 {
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

fn print_raw(ret: &mut Vec::<String>, raw: &str, raw_type: u32, pos: usize, sub: &str) {
    // Todo dump size of the raw and get the substitution values 
    if let Some(v) = match raw_type {
        0 => Some(format!("±{} {} {} {}", pos, raw.len(), raw.to_owned(), sub.to_owned())),
        1 => Some(format!("+{} {}", pos, raw.to_owned())),
        2 => Some(format!("-{} {}", pos, raw.to_owned())),
        _ => { None }
    } { ret.push(v); }
}

fn diff<T: std::cmp::PartialEq + std::fmt::Display>(source: &[T], target: &[T]) -> String {
    let o = levenstein(source, target);
    let mut i = source.len() - 1;
    let mut j = target.len() - 1;
    let mut ret = Vec::new();
    let mut raw = String::new();
    let mut raw_sub = String::new();
    let mut raw_type = 3;
    while i > 0 && j > 0 {
        if o[i][j] == 0 {
            if source[i] != target[j] { // Substitute
                if raw_type != 0 {
                    print_raw(&mut ret, &raw, raw_type, i + 1, &raw_sub);
                    raw = String::new();
                    raw_sub = String::new();
                    raw_type = 0;
                }
                raw.push_str(&source[i].to_string());
                raw_sub.push_str(&target[j].to_string());
            }
            j -= 1;
            i -= 1;
        } else if o[i][j] == 1 { // insert
            if raw_type != 1 {
                print_raw(&mut ret, &raw, raw_type, i + 1, &raw_sub);
                raw = String::new();
                raw_sub = String::new();
                raw_type = 1;
            }
            raw.push_str(&target[j].to_string());
            i -= 1;
        } else if o[i][j] == 2 { // delete
            if raw_type != 2 {
                print_raw(&mut ret, &raw, raw_type, i + 1, &raw_sub);
                raw = String::new();
                raw_sub = String::new();
                raw_type = 2;
            }
            raw.push_str(&target[j].to_string());
            j -= 1;
        }
    }
    print_raw(&mut ret, &raw, raw_type, i + 1, &raw_sub);
    ret.join("\n")
}

fn main() {
    let s = vec!['_', 'a', 'b', 'c', 'n'];
    let t = vec!['_', 'a', 'd', 'h', 'c', 'n'];
    let d = diff(&s, &t);
    println!("{}", d);
}