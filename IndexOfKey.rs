use std::io;

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 7, 8];

    println!("Enter the key:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let key: i32 = input.trim().parse().expect("Invalid input");

    let mut low = 0;
    let mut high = arr.len() as i32 - 1;
    let mut pos = -1;

    while low <= high {
        let mid = (low + high) / 2;
        
        if arr[mid as usize] == key {
            pos = mid;
            high = mid - 1;  
        } else if arr[mid as usize] < key {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    if pos == -1 {
        println!("Not found!");
    } else {
        println!("{}", pos);
    }
}
