fn main() {
    if let Err(error) = harmony_app::main_entry() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
