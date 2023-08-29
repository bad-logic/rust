// context
// there is an online shopping business who recently held lucky draw contest
// the ones selected from the contest have been given s $50 shopping card gift
// which can be used to buy items from the online store

// there is however a restriction from the business that the customers can only buy two
// products at most. now we want to help the customers by suggesting a list of products 
// that contains products worth $50 exactly so that they are not worried from paying
// the extra from their own pockets.
// we want to suggest the packages deals containing two products that sum up to $50


//----------------------------------
//      Suggesting Item for Special Shopping Card
//          - Description
//              - Given a list of prices, return a couple of items with their sum matching
//                the gift card $50
//              
// 
//          - Tools
//              - HashSet, Vectors
//----------------------------------

use std::collections::HashSet;

fn prod_suggestions(product_prices: Vec<i32>,amount: i32) ->Vec<Vec<i32>>{
    let mut prices_hash = HashSet::new();
    let mut offers = Vec::new();

    for i in product_prices{
        let diff = amount - i;
        println!("prices hash: {:?}",prices_hash);
        
        println!("i: {}, diff: {}",i,diff);
        if prices_hash.get(&diff).is_none(){
            prices_hash.insert(i);
        }else{
            offers.push(vec![i,diff]);
        }
    }

    println!("{:?}",prices_hash);


    offers
}

fn main() {
    // assuming the prices do not repeat
    let product_prices = vec![11,30,55,34,45,10,19,20,60,5,23];

    let suggestions = prod_suggestions(product_prices,50);
    println!("{:?}",suggestions);
    
}
