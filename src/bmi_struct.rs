struct BmiRange {
    min: f64,
    max: f64,
    label: &'static str,
}

fn main() {
    let height_cm = input("HEIGHT(cm)?");
    let weight = input("WEGHT(kg)?");

    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);

    let bmi_list = vec![
        BmiRange { min: 0.0, max: 18.5, label: "LOW" },
        BmiRange { min: 18.5, max: 25.0, label: "NORMAL" },
        BmiRange { min: 25.0, max: 30.0, label: "FAT1" },
        BmiRange { min: 30.0, max: 35.0, label: "FAT2" },
        BmiRange { min: 35.0, max: 40.0, label: "FAT3" },
        BmiRange { min: 40.0, max: 99.0, label: "FAT4" }
    ];

    let mut result = "UNKNOWN";
    for range in bmi_list {
        if range.min <= bmi && bmi < range.max {
            result = range.label;
            break;
        }
    }

    println!("BMI={:.1}, RESULT={}", bmi, result)
}

fn input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("INPUT ERR");
    s.trim().parse().expect("CONVERT TYPE ERR")
}
