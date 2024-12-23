use rand::Rng;

fn judge_return_number(num: u32) {
    let secret_number = rand::thread_rng().gen_range(1..100);

    if num < secret_number {
        println!("秘密の数字は50より小さいです");
    } else {
        println!("秘密の数字は50より大きいです");
    }
}

fn main() {
    judge_return_number(50);
}
