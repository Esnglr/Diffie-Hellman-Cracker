mod dlp;

use dlp::brute_force::solve_discrete_log_brute_force as solve_brute_force;
use dlp::shanks::solve_shanks;

fn main() {
    // Alice's shared key: A = 6 (3^15 mod 17)
    // Bob's shared key: B = 12 (3^13 mod 17)
    let g = 3u64;
    let p = 17u64;

    let alice_pub = 6u64;
    let bob_pub = 12u64;

    println!("1. Brute Force Method");
    println!("Searching for Alice's secret key.");
    match solve_brute_force(g, alice_pub, p) {
        Some(x) => println!("[+] Alice's secret key (x): {}", x),
        None => println!("[-] No secret key!"),
    }

    println!("Searching for Bob's secret key.");
    match solve_brute_force(g, bob_pub, p) {
        Some(y) => println!("[+] Bob's secret key (y): {}", y),
        None => println!("[-] No secret key!"),
    }


    println!("2. Shanks (Baby-step Giant-step) Method");
    println!("Searching for Alice's secret key.");
    match solve_shanks(g, alice_pub, p) {
        Some(x) => println!("[+] Alice's secret key (x): {}", x),
        None => println!("[-] No secret key found!"),
    }

    println!("Searching for Bob's secret key...");
    match solve_shanks(g, bob_pub, p) {
        Some(y) => println!("[+] Bob's secret key (y): {}", y),
        None => println!("[-] No secret key found!"),
    }


    println!("3. ");

}