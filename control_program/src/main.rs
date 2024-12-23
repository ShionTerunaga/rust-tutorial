use rand::Rng;
fn main() {
    let rand=rand::thread_rng().gen_range(1..100);
    //本当はそういうことはしないけど
    let judge= if rand%2==0 {"偶数だ！"} else {"奇数だ！"};

    println!("{}!!これは{}",rand,judge);
}
