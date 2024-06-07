fn main() {
    let words = "Hey I am Intern at QuadB Tech Company ";
    let word: Vec<&str> = words.trim().split_whitespace().collect();
    let mut shortest = word[0];

    for i in &word[1..] {
        if i.len() < shortest.len() {
            shortest = i;
        }
    }

    println!("{}", shortest);
}
