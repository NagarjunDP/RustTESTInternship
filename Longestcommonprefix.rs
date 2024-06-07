fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    
    let mut prefix = String::new();
    let first_str = &strings[0];
    
    for (i, c) in first_str.chars().enumerate() {
        for s in &strings[1..] {
            if let Some(sc) = s.chars().nth(i) {
                if sc != c {
                    return prefix;
                }
            } else {
                return prefix;
            }
        }
        prefix.push(c);
    }
    
    prefix
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    
    let longest_prefix = longest_common_prefix(&strings);
    println!("Longest Common Prefix: {}", longest_prefix);
}
