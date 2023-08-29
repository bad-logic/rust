
/*
 _______________
|               |
|               |
|     HEAP      |
|               |
|_______________|
|               |
|               |
|     STACK     |
|               |
|_______________|
|               |
|               |
| Static/Global |
|               |
|_______________|
|               |
|               |
|   Code/Text   |
|               |
|_______________|



Heap: stores data with unknown size at compile time or changeable size 

Stack: store all the information regarding the function calls and the local variables defined 
inside these functions. all the premitive type variables.
stores data with a known fixed size.

Static/Global: store all the static or global variables. 
these variable have the whole lifetime of a program application and are accessible 
anywhere during the whole lifecycle of the program as long as it is in execution. 
They are declared before the start of the main function at the top of the program 
and are typically constants.

Code/Text: store program instructions and code

The amount of memory that is set aside for code/Text, Static/Global and stack
segments does not grow while the application is running.

*/

const MAX_VALUE:i32 = 40_000; 


fn main(){
    let (x,y) = (2,4); 
    let sum_value = square_sum(x,y);
    println!("The value of Square of sum ={}", sum_value);

}

fn square_sum(num1:i32,num2:i32) -> i32{
    square(num1 + num2)
}

fn square(num:i32) -> i32{
    num * num
}


/*
 ___________________________
|                           |
|           HEAP            |
|                           |
|                           |
|                           |
|                           |
|                           |
|                           |
|___________________________|
|                           |
|           STACK           |
|                           |
|    ___________________    |
|   |                   |   |
|   |    Square()       |   |
|   |      num          |   |
|   |                   |   |
|   |                   |   |
|   |___________________|   |
|                           |
|    ___________________    |
|   |                   |   |
|   |    Square_sum()   |   |
|   |     num1, num2    |   |
|   |                   |   |
|   |                   |   |
|   |___________________|   |
|                           |
|    ___________________    |
|   |                   |   |
|   |      Main()       |   |
|   |       x,y         |   |
|   |                   |   |
|   |                   |   |
|   |___________________|   |
|                           |
|___________________________|
|                           |
|      GLOBAL/STATIC        |
|         MAX_VALUE         |
|                           |
|                           |
|                           |
|___________________________|
|                           |
|        Code/Text          |
|                           |
|___________________________|

*/