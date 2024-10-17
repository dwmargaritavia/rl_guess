use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=10);
    loop {
        println!("Какое число было мною загадано от 1 до 10?");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Не удалось прочитать ввод");

        let guess: u32 = guess.trim().parse().expect("Введите число.");

        println!("Вы предположили: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("меньше"),
            Ordering::Greater => println!("больше"),
            Ordering::Equal => {
                println!("Совпало");
                break;
            }
        }
    }
}
