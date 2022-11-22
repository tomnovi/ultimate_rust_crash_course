pub fn inspect(str: &String) {
    if str.ends_with("s") {
        println!("plural");
    } else {
        println!("singular");
    }
}

pub fn change(str: &mut String){
    if !str.ends_with("s") {str.push_str("s")}
}

pub fn eat(str: String) -> bool {
    str.starts_with("b") && str.contains("a")
}

pub fn bedazzle(str: &mut String) {
    (*str) = "sparkly".to_string();
    // (*str) = String::from("sparkly");
}
