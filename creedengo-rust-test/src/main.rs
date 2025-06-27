fn main() {
    let value = 2;
    
    // This should trigger the GCI2 lint
    let result = if value == 1 {
        "one"
    } else if value == 2 {
        "two"
    } else if value == 3 {
        "three"
    } else {
        "other"
    };
    
    println!("Result: {}", result);
    
    // Another example that should trigger the lint
    let status = get_status_code(404);
    println!("Status: {}", status);
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
