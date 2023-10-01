

fn main(){

    let json ="{
        \"name\": \"Bob\",
        \"age\": 24,
        \"gender\": \"male\"
    }";

    let json_literal =r#"{
        "name": "Bob",
        "age": 24,
        "gender": "male"
    }"#;

    println!("{:#?}", json);
    println!("{:#?}", json_literal);
}