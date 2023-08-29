fn main(){
    // infinite loop
    // loop{
    //     println!("This is an infinite loop");
    // }

    // while loop
    let my_number:u8 = 5;
    let mut guess: bool = false;

    println!("Guess my number which is 1 and 20");

    while !guess {
        let mut number: String = String::new();
        std::io::stdin().read_line(&mut number).expect("failed to read input");

        let number: u8 = number.trim().parse().expect("invalid input");
        if number == my_number {
            println!("Your guess is correct");
            guess = true;
        }else{
            println!("Try Again");
        }
    }
}