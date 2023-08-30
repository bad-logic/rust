

#[derive(Debug,Default)]
pub struct Student{
    id:u8,
    pub name: String,
    pub age: u8
}


impl Student {
    // pub fn new(std_name:String) -> Self{
    //     Self {
    //         id: 0,
    //         name: std_name,
    //         age: 29
    //     }
    // }

    pub fn new(std_name: String) -> Result<Self, String> {
        if std_name.to_lowercase().chars().all(|x| matches!(x,'a'..='z')){
            Ok(Self {
                id: 0,
                name: std_name,
                age: 29
            })
        }else{
            Err("The name is invalid".to_string())
        }
    }
}

// impl  Default for Student {
//     fn default() -> Self {
//         Self {
//             id: 0,
//             name: "unknown".to_string(),
//             age: 20
//         }
//     }
// }