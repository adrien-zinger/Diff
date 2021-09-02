use std::vec::Vec;

fn levenstein<T: std::cmp::PartialEq>(s: &Vec<T>, t: &Vec<T>) -> Vec::<Vec::<u8>> {
    let mut d = vec![vec![0; t.len()]; s.len()];
    let mut o = vec![vec![0 as u8; t.len()]; s.len()];
    let m = s.len();
    let n = t.len();
    for i in 1..m { d[i][0] = i; }
    for j in 1..n { d[0][j] = j; }
    for j in 1..m {
        for i in 1..m {
            let substitution_cost = if s[i] == t[j] { 0 } else { 1 };
            let deletion = d[i - 1][j] + 1;
            let insertion = d[i][j - 1] + 1;
            let substitution = d[i - 1][j - 1] + substitution_cost;
            d[i][j] = if substitution <= insertion && substitution <= deletion {
                o[i][j] = 0;
                substitution
            } else if insertion < substitution && insertion < deletion {
                o[i][j] = 1;
                insertion
            } else if deletion < substitution && deletion < insertion {
                o[i][j] = 2;
                deletion
            } else {
                o[i][j] = 0;
                substitution
            }
        }
    }
    o
}

fn print_raw(ret: &mut Vec::<String>, raw: &str, raw_type: u32, pos: usize) {
    // Todo dump size of the raw and get the substitution values 
    if let Some(v) = match raw_type {
        0 => Some(format!("±{} {}", pos, raw.to_owned())),
        1 => Some(format!("+{} {}", pos, raw.to_owned())),
        2 => Some(format!("-{} {}", pos, raw.to_owned())),
        _ => { None }
    } { ret.push(v); }
}

fn diff<T: std::cmp::PartialEq + std::fmt::Display>(s: &Vec<T>, t: &Vec<T>) -> String {
    let o = levenstein(s, t);
    let mut i = s.len() - 1;
    let mut j = t.len() - 1;
    let mut ret = Vec::new();
    let mut raw = String::new();
    let mut raw_type = 3;
    while i > 0 && j > 0 {
        if o[i][j] == 0 {
            if s[i] != t[j] { // Substitute
                // Make this part generic
                if raw_type != 0 {
                    print_raw(&mut ret, &raw, raw_type, i);
                    raw = String::new();
                    raw_type = 0;
                }
                raw.push_str(&s[i].to_string());
            }
            j -= 1;
            i -= 1;
        } else if o[i][j] == 1 { // insert
            if raw_type != 1 {
                print_raw(&mut ret, &raw, raw_type, i);
                raw = String::new();
                raw_type = 1;
            }
            raw.push_str(&t[j].to_string());
            i -= 1;
        } else if o[i][j] == 2 { // delete
            if raw_type != 2 {
                print_raw(&mut ret, &raw, raw_type, i);
                raw = String::new();
                raw_type = 2;
            }
            raw.push_str(&t[j].to_string());
            j -= 1;
        }
    }
    print_raw(&mut ret, &raw, raw_type, i);
    ret.join("\n").to_string()
}

fn main() {
    let s = vec!['_', 'a', 'b', 'c', 'n'];
    let t = vec!['_', 'a', 'd', 'h', 'c', 'n'];
    let d = diff(&s, &t);
    println!("{}", d);
}