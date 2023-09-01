//----------------------------------
// 
//      Builder Pattern
// 
// 
// 
//----------------------------------

#[derive(Debug,Clone)]
enum MembershipType {
    new,
    casual,
    loyal
}

impl Default for MembershipType {
    
    fn default() -> Self {
        Self::new
    }
    
}

#[derive(Debug,Default,Clone)]
struct Customer{
    name: String,
    username: String,
    membership: MembershipType,
    gender: char,
    country: String,
    age: u8
}

impl  Customer {

    fn new(name: String) -> CustomerBuilder{
        CustomerBuilder { name, username: None, membership: None, gender: None, country: None, age: None }
    }
  
}

// builder pattern
struct CustomerBuilder{
    name: String,
    username: Option<String>,
    membership: Option<MembershipType>,
    gender: Option<char>,
    country: Option<String>,
    age: Option<u8>
}

impl CustomerBuilder {

    fn username(&mut self,username: String) -> &mut Self{
        self.username = Some(username);
        self
    }

    fn membership(&mut self,membership: MembershipType) -> &mut Self{
        self.membership = Some(membership);
        self
    }

    fn gender(&mut self,gender: char) -> &mut Self{
        self.gender = Some(gender);
        self
    }

    fn country(&mut self,country: String) -> &mut Self{
        self.country = Some(country);
        self
    }

    fn age(&mut self,age: u8) -> &mut Self{
        self.age = Some(age);
        self
    }

    fn build(&mut self) -> Customer{
        Customer { 
            name: self.name.clone(), 
            username: self.username.clone().unwrap_or_default(), 
            membership: self.membership.clone().unwrap_or_default(), 
            gender: self.gender.unwrap_or_default(), 
            country: self.country.clone().unwrap_or_default(), 
            age: self.age.unwrap_or_default()
        }
    }
    
}

fn main(){

    let cust1 = Customer::new("Bob".to_string()).build();

    let cust2 = Customer::new("Bob".to_string())
                            .username("Bob123".to_string())
                            .build();
    
    let cust3 = Customer::new("Bob".to_string())
                            .username("Bob123".to_string())
                            .membership(MembershipType::casual)
                            .build();

    println!("{:#?}",cust1);
    println!("{:#?}",cust2);
    println!("{:#?}",cust3);

}