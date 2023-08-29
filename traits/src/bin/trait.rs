
struct Person{
    citizenship: String,
    name: String,
    age: u8,
    gender: char,
    salary: i32
}

struct Student{
    name_std: String,
    age: u8,
    sex: char,
    country: String
}

trait GeneralInformation{
    fn info(&self) -> (&str,u8,char);
    fn country_info(&self) -> &str;
}

impl GeneralInformation for Person{
    fn info(&self) -> (&str,u8,char){
        (&self.name,self.age,self.gender)
    }

    fn country_info(&self) -> &str{
        &self.citizenship
    }
}

impl GeneralInformation for Student{
    fn info(&self) -> (&str,u8,char){
        (&self.name_std,self.age,self.sex)
    }

    fn country_info(&self) -> &str{
        &self.country
    }
}

fn main(){

    let person:Person = Person{
        name:String::from("Bob"),
        citizenship:String::from("usa"),
        age:48,
        gender:'M',
        salary:45000
    };

    let student: Student = Student{
        name_std:String::from("Ben"),
        age:19,
        sex:'M',
        country:String::from("usa")
    };

    println!("person information {:?}",Person::info(&person));
    println!("student information {:?}",Student::info(&student));

}