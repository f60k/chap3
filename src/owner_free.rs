fn main() {
    {
        let s1 = 10; //String::from("aaaaa");
        let s3 = String::from("bbbbb");
        {
            let s2 = s1;
            println!("{}", s2);
            println!("{}", s1);
        }
        println!("{}", s3);
    }
    // println!("{}", s3);
}
