fn main() {
    let str = String::from("hello");

    //あんまり良くないやり方
    let first_char = &str[0..1];
    println!("First character is {}", first_char);

    //こっちの方がいい
    let first_char = str.chars().nth(0);
    println!("String is {}", &str);
    println!("First character is {:?}", &first_char);

    if let Some(c) = first_char {
        println!("Character removing the Some is {}", c);
    }

    for (i, c) in str.chars().enumerate() {
        println!("{}. {}", i + 1, c);
    }
}
