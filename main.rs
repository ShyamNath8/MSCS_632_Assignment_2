fn main() {
    println!("--- Ownership Example ---");

    // `String` is heap-allocated
    let s1 = String::from("Rust");
    // Ownership moves from s1 to s2
    let s2 = s1;

    // println!("{}", s1); // ❌ error: s1 no longer valid
    println!("s2 owns the string: {}", s2);

    println!("\n--- Borrowing Example ---");
    let s3 = String::from("Borrow me");
    borrow_string(&s3); // &s3 is a reference, no move occurs
    println!("s3 is still valid here: {}", s3);

    println!("\n--- Mutable Borrowing ---");
    let mut s4 = String::from("Hello");
    change_string(&mut s4);
    println!("After change: {}", s4);
}

fn borrow_string(s: &String) {
    println!("I borrowed: {}", s);
}

fn change_string(s: &mut String) {
    s.push_str(", world!");
}
