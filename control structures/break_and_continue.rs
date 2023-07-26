//-------------------------------
// break - for stopping a loop
// continue - skips the current iteration of loop and continues loops next iteration
//---------------------------------

fn main(){

    let mut var:i32 = 100;
    loop{
        var = var - 1;
        if var % 13 == 0 {
            break;
        } 
    }
    println!("The highest number lesser than the given number divisible by 13 is {}",var);

    let mut var: i32 = 0;
    let mut count = 0;

    let number:i32 = loop{
        var += 1;
        if var % 5 == 0 && var % 3 == 0 {
            println!("The number which is divisible by both 3 and 5 is {} \n",var);
            count += 1;
            if count == 3 {
                break var;
            }else{
                continue;
            }
        }
        println!("{}",var);
    };

    println!("The 3rd number divisible by both 3 and 5 is {}",number);
}