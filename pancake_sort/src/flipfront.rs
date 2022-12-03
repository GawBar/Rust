#[allow(dead_code)]

pub fn flipfront(array: Vec<i32>, n: usize) -> Vec<i32>{
    if n < 2 && n > array.len() {
        return vec![];
    }

    let mut result = vec![];

    let mut i: i32 = n as i32 - 1;
    while i >= 0 {
        println!("i: {}", i);
        result.push(array[i as usize]);
        i -= 1;
    }

    for elem in array[n..].iter() {
        result.push(*elem);
    }

    return result;
}