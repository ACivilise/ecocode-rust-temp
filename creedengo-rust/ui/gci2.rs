fn multiple_if_else_bad(x: i32) -> String {
    if x == 1 {
        "one".to_string()
    } else if x == 2 {
        "two".to_string()
    } else if x == 3 {
        "three".to_string()
    } else {
        "other".to_string()
    }
}

fn multiple_if_else_worse(x: i32) -> String {
    if x == 1 {
        "one".to_string()
    } else if x == 2 {
        "two".to_string()
    } else if x == 3 {
        "three".to_string()
    } else if x == 4 {
        "four".to_string()
    } else if x == 5 {
        "five".to_string()
    } else {
        "other".to_string()
    }
}

fn get_status_code(code: u16) -> &'static str {
    if code == 200 {
        "OK"
    } else if code == 404 {
        "Not Found"
    } else if code == 500 {
        "Internal Server Error"
    } else if code == 403 {
        "Forbidden"
    } else {
        "Unknown"
    }
}

fn single_if_else_ok(x: i32) -> String {
    if x == 1 {
        "one".to_string()
    } else {
        "other".to_string()
    }
}

fn using_match_good(x: i32) -> String {
    match x {
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        _ => "other".to_string(),
    }
}

fn main() {
    println!("{}", multiple_if_else_bad(1));
    println!("{}", multiple_if_else_worse(1));
    println!("{}", get_status_code(404));
    println!("{}", single_if_else_ok(1));
    println!("{}", using_match_good(1));
}
