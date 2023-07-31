fn main() {
    let pr = "知恵は武器よりも価値がある。";

    let sub3: String = pr.chars().take(2).collect();
    println!("{}", sub3);

    let pr_chars: Vec<char> = pr.chars().collect();

    let sub_chars = &pr_chars[3..5];

    let sub4: String = sub_chars.into_iter().collect();
    println!("{}", sub4)
}
