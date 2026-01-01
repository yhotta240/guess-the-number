use rand::Rng;
use rand::thread_rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("数当てゲーム!");

    let secret_number: u32 = thread_rng().gen_range(1..101);
    
    let mut attempts: u32 = 0; //試行回数をカウントする変数

    loop {
        println!("1から100までの数字を予想してください:");
    
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("読み込みに失敗しました");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("あなたの予想: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("小さすぎ！");
                attempts += 1;
            },  //小さすぎ！
            Ordering::Greater => {
                println!("大きすぎ！");
                attempts += 1;
            }, //大きすぎ！
            Ordering::Equal => {
                println!("やったね！ あなたの試行回数は {} 回です。", attempts + 1);
                break;
            } //やったね！
        }
    }
}
