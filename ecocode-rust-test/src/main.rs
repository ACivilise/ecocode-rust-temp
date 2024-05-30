fn main() {
    // this will trigger a dylint warning
    // @see https://github.com/trailofbits/dylint/examples/restriction/const_path_join
    let target_path = std::path::PathBuf::from("..").join("target");
    println!("{}", target_path.to_str().unwrap());
}
