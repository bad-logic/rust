//-----------------------------------------------------
// 
//          Efficient Storage and Retrieval of words
// 
//              - Trie Data Structure ( type of k-array search tree )
//                  - a.k.a digital tree or prefix tree.
//  
//-----------------------------------------------------

use std::collections::HashMap;

#[derive(Default,Debug,PartialEq,Eq,Clone)]
struct Node{
    children: HashMap<char,Node>,
    is_word:bool
}

impl  Node {
    fn new()->Self{
        Node { children: HashMap::new(), is_word: false }
    }
}

#[derive(Default,Debug,PartialEq,Eq,Clone)]
struct WordDictionary{
    root:Node
}

impl WordDictionary {
    fn new()-> Self{
        Self::default()
    }

    fn insert(&mut self, word: &String){
        let mut current = &mut self.root;
        for l in word.chars(){
            current = current.children.entry(l).or_insert( Node::new());
        }

        // if !current.is_word {
            current.is_word = true;
        // }
    }

    fn search(&self,word:&String)-> bool{
        let mut current = &self.root;
        let mut exists = false;
        for w in word.chars(){
            if current.children.get(&w).is_some(){
                current = current.children.get(&w).unwrap();
                exists = current.is_word;
            }else{
                exists = false;
                break;
            }
        }
        exists
    }
    
}

fn main(){

    let words = vec!["the","a","there","any","by","bye","their","abc"]
    .into_iter().map(|x| String::from(x)).collect::<Vec<String>>();

    let mut dictionary = WordDictionary::new();

    for word in words{
        dictionary.insert(&word);
    }

    // println!("Dictionary: {:#?}",dictionary);

    println!("Searching \'the\' in the dictionary. Entry found: {}",dictionary.search(&String::from("the")));
    println!("Searching \'that\' in the dictionary. Entry found: {}",dictionary.search(&String::from("that")));
}