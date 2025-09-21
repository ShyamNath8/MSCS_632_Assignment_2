// rust_memory.rs
fn take_ownership(s: String) { // takes ownership
    println!("Owned string: {}", s);
} // s dropped here (memory freed)

fn borrow_string(s: &String) { // borrows (immutable)
    println!("Borrowed string: {}", s);
}

fn main() {
    let s1 = String::from("hello");   // s1 owns the string
    borrow_string(&s1);               // borrow: no transfer of ownership
    take_ownership(s1);               // ownership moved; s1 no longer valid here

    // After move, s1 cannot be used: compiler error if we try to use it.
    // Rust enforces these rules at compile-time.
}
