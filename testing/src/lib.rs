
struct Circle{
    radius: f32,
}

impl Circle {
    fn area(&self)-> f32{
        3.14 * self.radius * self.radius
    }

    fn perimeter(&self)->f32{
        2.0 * 3.14 * self.radius
    }

    fn contains(&self, other: &Circle)-> bool{
        self.radius > other.radius
    }
    
}

fn division(divident:f64,divisor:f64)->Option<f64>{
    match divisor {
        0.0 => None,
        _ => Some(divident/divisor)   
    }
}

fn division_err(divident:f64,divisor:f64)->Result<f64,String>{
    match divisor {
        0.0 => Err(String::from("Error: Division by zero")),
        _ => Ok(divident/divisor)   
    }
}

struct Person{
    name: String,
    age: i32,
    salary: f64
}

impl Person {
    fn salary_range(&self){
        if self.salary <= 10_000.0{
            panic!("The salary must be greater than 10,000");
        }else if self.salary >= 30_000.0{
            panic!("The salary must be less than 30,000");
        }
    }
    
}

fn square(x:i32)-> i32{
    println!("The square of the number is {}",x * x);
    x * x
}

#[cfg(test)]
mod tests{
    // bring everything to tests module
    use super::*; // refers to the parent module

    #[test]
    fn larger_circle_should_contain_smaller_circle(){
        let c1 = Circle{
            radius:5.0
        };
        let c2 = Circle{
            radius: 6.0
        };
        assert!(c2.contains(&c1));
    }

    #[test]
    fn smaller_circle_should_not_contain_larger_circle(){
        let c1 = Circle{
            radius:5.0
        };
        let c2 = Circle{
            radius: 6.0
        };
        assert!(!c1.contains(&c2));
    }

    #[test]
    fn larger_circle_should_have_more_area(){
        let c1 = Circle{
            radius:5.0
        };
        let c2 = Circle{
            radius: 6.0
        };
        assert!(c2.area() > c1.area());
    }

    #[test]
    fn larger_circle_should_have_more_perimeter(){
        let c1 = Circle{
            radius:5.0
        };
        let c2 = Circle{
            radius: 6.0
        };
        assert!(c2.perimeter() > c1.perimeter());
    }

    #[test]
    fn test_1(){
        let divident = 10.0;
        let divisor = 0.0;

        let result = division(divident, divisor);
        assert!(result!=None,"This is because division by zero, the divident in this case
        was {} and the divisor was {}",divident,divisor);
    }

    #[test]
    #[should_panic(expected="The salary must be less than 30,000")]
    fn out_of_range(){
        let p1  = Person{
            name: "Bob".to_string(),
            age:33,
            salary:32_000.00
        };
        p1.salary_range();
    }

    #[test]
    fn test_ok(){
        assert!(division_err(15.0,3.0).is_ok());
    }

    #[test]
    fn test()->Result<(),String>{
        if 2+2 == 4 {
            Ok(())
        }else{
            Err(String::from("the two values are not equal"))
        }
    }

    #[test]
    fn square_of_two(){
        assert_eq!(square(2),4);
    }

    #[test]
    fn square_of_three(){
        assert_ne!(9,square(3));
    }

    #[test]
    fn square_of_four(){
        assert_eq!(16,square(4));
    }

    #[test]
    #[ignore]
    fn expensive_test(){
        
    }
}