fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let nubmer_list = vec![34, 50, 25, 100, 65];
    let result = largest(&nubmer_list);
    println!("The is {}", result);

    let nubmer_list = vec![34, 50, 25, 100, 65, 6000, 390];
    let result = largest(&nubmer_list);
    println!("The is {}", result);
}
