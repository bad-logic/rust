use std::rc::Rc;


fn make_rc() -> Rc<String> {
    let s1 = Rc::new(String::from("Hello"));
    println!("count when the pointer is created {}",Rc::strong_count(&s1));

    let s2 = s1.clone();
    println!("count after the clone is created for the pointer {}",Rc::strong_count(&s1));

    return s2;

}

fn main(){
    let s2 = make_rc();
    println!("References count after function call {}",Rc::strong_count(&s2));
}