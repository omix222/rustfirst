extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    // ちょっと待って下さい、既に guess を定義してありますよね? してあります、が、
    // Rustでは以前の guess の定義を新しいもので「隠す」ことが出来ます
    // (訳注: このように隠すことをシャドーイングといいます)。 まさにこのように、
    // 最初 String であった guess を u32 に変換したい、というような状況でよく使われます。 
    // シャドーイングのおかげで guess_str と
    // guess のように別々の名前を考える必要はなくなり、 guess の名前を再利用出来ます。
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => println!("You win!"),
    }
}