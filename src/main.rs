use std::{collections::HashMap, io};

fn main() {
    let mut v = vec![8, 8, 9, 2, 234, 10, 234, 81, 3, 3, 3];
    v.sort();
    println!("The middle of {v:?} is {}", v[v.len() / 2]);
    let mut h = HashMap::new();
    for n in v {
        h.entry(n).and_modify(|count| *count += 1).or_insert(1);
    }
    println!(
        "{h:?}, {}",
        match h.iter().max_by_key(|&(_, v)| v).map(|(k, _)| *k) {
            Some(x) => x,
            None => return,
        }
    );

    let mut s = String::from("example");
    let vocals = ['a', 'e', 'i', 'o', 'u'];

    if s.starts_with(vocals) {
        s.push_str("-hay");
        println!("{}", s);
    }
    let mut s = String::from("first");
    let c1 = s.remove(0);
    // let finals = format!("-{c1}ay");
    // s.push_str(&finals);
    println!("{}", format_args!("{s}-{c1}ay"));

    interface_add_employee();
}

fn interface_add_employee() {
    println!(
        "Please insert the command as follows \"Add <Name of employee> to <name of the department>\""
    );

    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");

    let s_split: Vec<&str> = s.split_whitespace().collect();
    let mut h = HashMap::new();

    if s_split.len() >= 4 && s_split[0] == "Add" {
        println!("{}", s_split[0]);
        h.entry(s_split[1]).or_insert(match s_split.get(3) {
            Some(x) => x,
            None => return,
        });
    } else {
        println!("Please insert the command given as above with the correct spaces");
    }
    println!("{:?}", h);
}
