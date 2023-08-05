//------------------------------------
//          Rust Modules
//------------------------------------

mod lib;

mod custom_module {
    // items in the modules default to private visibility
    fn private_fn(){
        println!("called `custom_module::private_fn()`");
    }

    // use pub modifier to override default visibility
    pub fn public_fn(){
        println!("called `custom_module::public_fn()`");
    }

    pub fn indirect_access(){
        println!("called `custom_module::indirect_access()`, that ");
        private_fn();
    }
}

fn main(){
    custom_module::public_fn();
    custom_module::indirect_access();
    lib::print_status();
    lib::file_1::print_status();
    lib::file_2::print_status();
}