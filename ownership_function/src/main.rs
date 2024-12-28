fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    //ここではもうsは使えない

    let x = 5;

    makes_copy(x);

    //数値型は所有権を持たないので別に使える

    let y = x;

    makes_copy(y);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}
