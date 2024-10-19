
fn main(){
    let name = String::from("Lovish Goyal");
    let length = get_str_length(name);
    println!("Length of String is {}", length);
}

fn get_str_length(str: String) -> usize {
    return str.chars().count();
}
