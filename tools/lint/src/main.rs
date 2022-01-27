use xshell::cmd;

fn main() {
    cmd!("cargo fmt --all -- --check")
        .run()
        .expect("Please run 'cargo fmt --all' to format your code.");
    cmd!("cargo clippy --workspace --all-targets --all-features -- -D warnings -A clippy::type_complexity -W clippy::doc_markdown")
        .run()
        .expect("Please fix clippy errors in output above.");
}
