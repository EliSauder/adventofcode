fn main() -> std::io::Result<()> {
    let mut buf = String::new();

    while std::io::stdin().read_line(&mut buf)? != 0 {}

    Ok(())
}
