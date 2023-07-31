fn main() {
    let banana = ("banana", 300);
    let apple = ("apple", 200);

    let total = banana.1 + apple.1;

    print_tupple(&banana);
    print_tupple(&apple);
    println!("合計{}円です", total);
}

fn print_tupple(item: &(&str, i64)) {
    println!("{}を{}円で購入", item.0, item.1);
}
