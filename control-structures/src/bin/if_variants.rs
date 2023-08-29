//------------------------------------
//  conditional if and its variants
//-----------------------------------

fn main(){

    let some_number:i32 = 40;

    if some_number < 50 {
        println!("The number is lesser than 50");
    }

    println!("This line will execute irrespective of the condition");

    let marks:i32 = 65;
    let grade:char;

    if marks > 70 && marks <=100{
        grade = 'A';
    }else if marks >= 60 && marks <=70{
        grade = 'B';
    }else{
        grade = 'C';
    }

    println!("The obtained grade is {}",grade);


}