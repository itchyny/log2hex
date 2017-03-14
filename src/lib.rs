// log(2) = sum [ 1 / (i * (2 ^ i)), i, 1, Infinity ]
// therefore
// (log(2) * (2 ^ d)) % 1 = sum [ ((2 ^ (d - i)) % i) / i, i, 1, d ] % 1
//                        + sum [ (2 ^ (d - i)) / i, i, d + 1, Infinity ] % 1
pub fn log2hex(place: u32) -> String {
    let d: u32 = 4 * place;
    let fraction1: f64 = (1..d + 1)
        .map(|i| (powmod(2, d - i, i as u64) as f64) / (i as f64))
        .sum();
    let fraction2: f64 = (d + 1..)
        .map(|i| 2.0_f64.powi(d as i32 - i as i32) / (i as f64))
        .take_while(|&x| x > 1e-10_f64)
        .sum();
    (0..4)
        .scan(fraction1 + fraction2, |x, _| {
            *x = (*x - x.floor()) * 16.0;
            Some(format!("{:x}", x.floor() as u32))
        })
        .fold(String::new(), |s, t| s + &t)
}

fn powmod(n: u64, m: u32, d: u64) -> u64 {
    if m == 0 {
        1 % d
    } else if m == 1 {
        n % d
    } else {
        let k: u64 = powmod(n, m / 2, d);
        if m % 2 == 0 {
            (k * k) % d
        } else {
            (k * k * n) % d
        }
    }
}
