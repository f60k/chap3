fn main() {
    let g1 = String::from("aaaaa");
    show_message(&g1);
    println!("{}", g1);
}

fn show_message(message: &String) {
    println!("{}", message);
}
