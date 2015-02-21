use std::thread;

fn main() {
    let _guard = thread::scoped(|| {
        println!("Hello from a thread!");
    });
}
