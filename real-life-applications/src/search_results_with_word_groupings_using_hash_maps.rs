//----------------------------------
//      correct search results using word grouping 
//          - Description
//              - given a list of words, group the words that are anagrams
// 
//          - Tools
//              - Hashmaps, Nested Loops
//----------------------------------

// 'a' as u32 => ascii value for a => 65
// 'b' as u32 => ascii value for b => 66
// 'c' as u32 => ascii value for c => 67

use std::collections::HashMap;

fn word_groupings(words_list:Vec<String>) -> Vec<Vec<String>>{
    let mut word_hash = HashMap::new();
    let mut char_freq = vec![0;26];

    for current_word in words_list {
        println!("current_word {}",current_word);
        for c in current_word.to_lowercase().chars(){
            println!("char {}",c);

            char_freq[(c as u32 - 'a' as u32) as usize] +=1;
        }

        let key = char_freq.clone().into_iter().map(|i| i.to_string()).collect::<String>();
        println!("char_freq: {:?}",char_freq);
        println!("char_freq.into_iter(): {:?}",char_freq.clone().into_iter());
        println!("char_freq.into_iter().map(|i| i.to_string()) {:?}",char_freq.into_iter().map(|i| i.to_string()));
        
        println!("key {:?}",key);
        word_hash.entry(key).or_insert(Vec::new()).push(current_word);

        char_freq = vec![0;26];

    }
    println!("{:?}",word_hash);
    

    for (key,value) in &word_hash{
        println!("Key # {:?} value {:?}",key,value);
    }

    let result = word_hash.into_iter().map(|(_,v)| v).collect();
    return result;

}


fn main() {
    let words = vec!["the".to_string(),"teh".to_string(),"het".to_string(),
    "stupid".to_string(),"stupdi".to_string(),"apple".to_string(),"appel".to_string()];


    let grouping = word_groupings(words); 

    println!("{:?}",grouping); // [["apple", "appel"], ["stupid", "stupdi"], ["the", "teh", "het"]]

    let input_word = String::from("teh");
    for i in grouping.into_iter(){
        if i.contains(&input_word){
            println!("The group of the word is {:?}",i);
        }
    }

}
