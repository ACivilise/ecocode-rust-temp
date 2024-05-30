use std::fs::File;

fn foo() -> std::io::Result<()> {
    let _ = File::open("/dev/null")?;
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
