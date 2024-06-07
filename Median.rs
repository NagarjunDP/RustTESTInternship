fn find_median(arr: &[i32]) -> f64 {
    let n = arr.len();
    let mid = n / 2;

    if n % 2 == 0 {
        
        let median = (arr[mid - 1] + arr[mid]) as f64 / 2.0;
        median
    } else {
       
        let median = arr[mid] as f64;
        median
    }
}

fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    let median = find_median(&arr);
    println!("Median: {}", median);
}
