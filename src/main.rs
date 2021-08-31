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
                o[i][j] = if s[i] == t[j] { 3 } else { 0 };
                substitution
            } else if insertion < substitution && insertion < deletion {
                o[i][j] = 1;
                insertion
            } else if deletion < substitution && deletion < insertion {
                o[i][j] = 2;
                deletion
            } else {
                o[i][j] = if s[i] == t[j] { 3 } else { 0 };
                substitution
            }
        }
    }
    o
}

fn diff<T: std::cmp::PartialEq + std::fmt::Display>(s: &Vec<T>, t: &Vec<T>) {
    let o = levenstein(s, t);
    let mut i = s.len() - 1;
    let mut j = t.len() - 1;
    let mut res: Vec::<String> = Vec::new();
    while i > 0 && j > 0 {
        if o[i][j] == 3 {
            res.push(format!("Nothing {} {}", s[i], t[j]));
            j -= 1;
            i -= 1;
        } else if o[i][j] == 0 {
            res.push(format!("Substitute {} {}", s[i], t[j]));
            j -= 1;
            i -= 1;
        } else if o[i][j] == 1 {
            res.push(format!("Insert {}", t[j]));
            i -= 1;
        } else if o[i][j] == 2 {
            res.push(format!("Delete {}", s[i]));
            j -= 1;
        }
    }
    res.reverse();
    println!("{}", res.join("\n"));
}

fn main() {
    let s = vec!['_', 'a', 'b', 'c', 'n'];
    let t = vec!['_', 'a', 'd', 'c', 'n'];
    diff(&s, &t);
}