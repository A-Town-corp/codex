//! Child process used only by the native launcher integration tests.

fn main() {
    for argument in std::env::args_os().skip(1) {
        println!("{}", argument.to_string_lossy());
    }
    std::process::exit(42);
}
