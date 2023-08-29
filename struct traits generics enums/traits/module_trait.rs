pub mod Basic{

    pub trait PersonalInfo{
        fn how_old(self)->u8;
    }

    pub struct Person{
        pub name: String,
        pub age: u8
    }
    
    impl PersonalInfo for Person{
        fn how_old(self) -> u8{
            self.age
        }
    }

}

// pub fn some_fn(){
//     use self::Basic::{PersonalInfo,Person};

//     let p1 = Person{
//         name: "Bob".to_string(),
//         age: 40
//     };

//     println!("{}",p1.how_old());
// }