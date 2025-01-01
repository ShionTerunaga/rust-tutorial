fn main() {
    process_1();

    process_2();
}

fn process_1() {
    let s1 = String::from("hello");

    let len1 = calculate_length(&s1);

    println!("The length of '{}' is  {}.", &s1, len1);

    let mut s2 = s1.clone();

    println!("Next String");

    change(&mut s2);

    let len1 = calculate_length(&s2);

    println!("The length of '{}' is  {}.", s2, len1);
}

fn process_2() {
    let mut s1 = String::from("hello");

    let len1 = calculate_length(&s1);

    println!("The length of '{}' is  {}.", &s1, len1);

    println!("Next String");

    change(&mut s1);

    let len1 = calculate_length(&s1);

    println!("The length of '{}' is  {}.", s1, len1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
問題

process_1とprocess_2はどちらを使うことが好ましいでしょうか？
*/
