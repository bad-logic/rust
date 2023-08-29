//----------------------------------
//      Popularity Scores 
//          - Description
//              - given some products along with its respective popularity scores,
//                  we want to determine if the popularity is fluctuating, increasing
//                  or decreasing
// 
//          - Tools
//              - Hashmaps, loops, conditional if
//----------------------------------

use std::collections::HashMap;


fn popularity_analysis(scores: Vec<i32>)-> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..scores.len()-2 {
        if scores[i] > scores[i+1]{
            increasing = false;
        }
        if scores[i] < scores[i+1]{
            decreasing = false;
        }
    }

    return increasing || decreasing;
}


fn main() {
    let mut products = HashMap::new();

    products.insert("p1", vec![1,2,2,3]);
    products.insert("p2", vec![4,5,6,3,4]);
    products.insert("p3", vec![8,8,7,6,5,4,4,1]);


    println!("{:?}",products);

    for ( name, popularity) in products{
        let popularity = popularity_analysis(popularity);
        if popularity {
            println!("popularity of {} is increasing or decreasing",name);
        }else{
            println!("popularity of {} is fluctuating",name);
        }
    }

}
