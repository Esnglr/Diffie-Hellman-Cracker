pub fn solve_discrete_log_brute_force(g: u64, target: u64, p: u64) -> Option<u64> {
    let mut current: u64 = 1; // g^0 mod p

    for x in 0..p {
        if current == target {
            return Some(x);
        }
        current = ((current as u128 * g as u128) % p as u128) as u64;
    }

    None
}