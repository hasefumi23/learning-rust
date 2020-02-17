fn main() {
    let nubmer_list = vec![34, 50, 25, 100, 65];
    let mut largest = nubmer_list[0];
    for number in nubmer_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The is {}", largest);
}
