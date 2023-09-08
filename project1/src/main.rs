fn main() {
    // Read a filename or crash
    let content = std::fs::read_to_string("Readmee.md").unwrap();
    content.bytes();

    // Count all characters that are numeric or ascii graphics
    let count: usize = unimplemented!();

    println!("Found {} math characters", count);
}

/// Count only those characters where calling `action(c)` returns true
fn count_characters(in_content: String, action: impl Fn(char) -> bool) -> usize {
    unimplemented!()
}
