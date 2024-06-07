fn reverse_string(s: &mut String) {
    let len = s.len();
    let mut chars = s.chars().collect::<Vec<char>>();
    
    for i in 0..len / 2 {
        chars.swap(i, len - 1 - i);
    }
    
    *s = chars.into_iter().collect();
}

fn main() {
    let mut s = String::from("QuadB Tech Intern");
    reverse_string(&mut s);
    println!("Reversed string: {}", s);
}
