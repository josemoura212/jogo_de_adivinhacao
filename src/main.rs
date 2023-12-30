use std::{cmp::Ordering, io};

use rand::Rng;
extern crate rand;

fn main() {
    println!("Advinhe o numero!");

    let numero_screto = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Digite o seu palpite.");

        let mut palpite = String::new();

        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler entrada");

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você disse: {}", palpite);

        match palpite.cmp(&numero_screto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }
}
