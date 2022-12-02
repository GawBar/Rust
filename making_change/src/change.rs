#[allow(dead_code)]

pub fn change(mut money: i32) -> i32{
    let units = [500, 100, 25, 10, 5, 1];

    let mut coins = 0;
    let mut i = 0;
    
    while money > 0 {
        if money >= units[i] {
            coins += 1;
            money -= units[i];
        } else {
            i += 1;
        }
    }

    return coins;
}