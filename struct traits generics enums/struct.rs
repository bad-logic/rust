struct Person {
    citizenship: String,
    name: String,
    age: i32,
    gender: char,
    salary: i32
}

impl Person{
    // fn new() -> Self {
    //     Person{
    //         name: String::from("xyz"),
    //         citizenship: String::from("America"),
    //         age: 40,
    //         gender:'M',
    //         salary: 50_000
    //     }
    // }

    fn new() -> Person {
        Self {
            name: String::from("xyz"),
            citizenship: String::from("America"),
            age: 40,
            gender:'M',
            salary: 50_000
        }
    }

    fn compute_taxes(&self) -> f32{
        (self.salary as f32 / 3.0) * 0.5
    }
}

struct Point(i32,i32);

impl Point{
    fn greater(&self) -> i32{
        if self.0 >= self.1 {
            self.0
        }else{
            self.1
        }
    }
    fn lesser(&self) -> i32 {
        if self.0 < self.1 {self.0} else {self.1}
    }
}

fn main(){

    let person1: Person = Person{
        name: String::from("Bob Marley"),
        citizenship: String::from("America"),
        age: 40,
        gender: 'M',
        salary: 40_000
    };

    println!("The structure values for person1 are {} {} {} {} {}",person1.name,person1.citizenship,person1.age,person1.gender,person1.salary);
    
    println!("Taxes: {}",person1.compute_taxes());
    println!("Taxes: {}",Person::compute_taxes(&person1));

    let person2 = Person::new();
    println!("The structure values for person2 are {} {} {} {} {}",person2.name,person2.citizenship,person2.age,person2.gender,person2.salary);
    println!("Taxes: {}",Person::compute_taxes(&person2));

    let person3: Person = Person {
        age: 50,
        name: String::from("Pam"),
        ..person1
    };

    println!("The structure values for person3 are {} {} {} {} {}",person3.name,person3.citizenship,person3.age,person3.gender,person3.salary);


    let mut person4:Person = Person::new();
    println!("The default name of person4 is {}",person4.name);
    person4.name=String::from("new name");
    println!("The updated name of person4 is {}",person4.name);


    let some_nums: Point = Point(32,16);
    // println!("{:?}",some_nums);
    println!("The points are ({},{})",some_nums.0,some_nums.1);
    println!("The greater value in some_nums is {}",Point::greater(&some_nums));
    println!("The lesser value in some_nums is {}",Point::lesser(&some_nums));


}