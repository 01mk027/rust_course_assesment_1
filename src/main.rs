fn main() {
    let string1 = String::from("String 1 ");
    let string2 = String::from("String 2");
    let new_result = concatenate_strings(&string1, &string2);
    println!("{}", new_result);
}

fn concatenate_strings(s1: &str, s2: &str) -> String{
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    return result;
}
