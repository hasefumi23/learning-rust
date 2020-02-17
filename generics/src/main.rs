fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn largest1<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let wont_work = Point { x: 1, y: 4.0 };
}

fn main2() {
    let nubmer_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&nubmer_list);
    println!("The is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The is {}", result);
}

fn main1() {
    let nubmer_list = vec![34, 50, 25, 100, 65];
    let result = largest(&nubmer_list);
    println!("The is {}", result);

    let nubmer_list = vec![34, 50, 25, 100, 65, 6000, 390];
    let result = largest(&nubmer_list);
    println!("The is {}", result);
}
