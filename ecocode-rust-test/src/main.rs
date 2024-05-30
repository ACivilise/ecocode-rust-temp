use std::path::PathBuf;

fn main() {
    // this will trigger a dylint warning
    // https://github.com/trailofbits/dylint", pattern = "examples/restriction/const_path_join
    let target_path = PathBuf::from("..").join("target");
    let number = 3;

    if number == 2 {
    } else if number == 3 {
    } else if number == 4 {
    }
} 