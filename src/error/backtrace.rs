pub(super) fn enable_backtrace() -> bool {
    let var = std::env::var("RUST_BACKTRACE");
    var.is_ok() && var.unwrap() != "0"
}
