use std::collections::HashMap;

pub fn solve_shanks(g: u64, target: u64, p: u64) -> Option<u64> {
    let m = ((p - 1) as f64).sqrt().ceil() as u64;

    // baby-step table
    let mut table: HashMap<u64, u64> = HashMap::new();
    let mut current: u64 = target;

    for j in 0..m {
        table.insert(current, j);
        current = ((current as u128 * g as u128) % p as u128) as u64;
    }

    let mut factor= 1u64;
    for _ in 0..m {
        factor = ((factor as u128 * g as u128) % p as u128) as u64;
    }

    let mut current_giant = factor;

    for i in 1..=m{
        if let Some(&j) = table.get(&current_giant) {
            let x = i * m - j;
            return Some(x);
        }

        current_giant = ((current_giant as u128 * factor as u128) % p as u128) as u64;
    }

    None

}