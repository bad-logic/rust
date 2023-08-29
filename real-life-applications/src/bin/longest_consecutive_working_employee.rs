//----------------------------------
//      Longest Non Stop work
//          - Description
//              - Given time slots numbers, we want to determine the longest consecutive
//                  time slots
//              
// 
//          - Tools
//              - HashSet, Vectors, Loops
//----------------------------------

// find the longest consecutive slot that an employee has worked

use std::collections::HashSet;

#[derive(Debug)]
struct EmployeeSchedule{
    name: String,
    slots: Vec<u8>, // slots number that the employee has worked in any given day
    longest_consecutive_slots: u8
}

impl  EmployeeSchedule {
    fn new(name:String,slots:Vec<u8>)->Self{
        EmployeeSchedule { name, slots,longest_consecutive_slots:0 }
    }

    fn set_longest_consecutive_slots(&mut self){
        let mut longest_consecutive_busy_period = 0;
        let slot_set = self.slots.clone().into_iter().collect::<HashSet<_>>();

        for &slot in &slot_set{
            if !slot_set.contains(&(slot -1)){
                // meaning slot is a small value whose slot - 1 does not exists in slots array
                let mut current_slot = slot.to_owned();
                let mut current_consecutive_slot = 1;

                // check if consecutive greater values exists
                // if exists move through the array and increase consecutive count
                while slot_set.contains(&(current_slot+1)){
                    current_slot += 1;
                    current_consecutive_slot += 1;
                }
                // if any consecutive slots found is greater than previous longest
                // set the previous longest to current longest
                if current_consecutive_slot > longest_consecutive_busy_period {
                    longest_consecutive_busy_period = current_consecutive_slot;
                }
            }
        }
        self.longest_consecutive_slots = longest_consecutive_busy_period;

    }
}

fn main() {
    let emp1 = EmployeeSchedule::new(String::from("emp1"),vec![4,1,2,5,6,8,10,11]);
    let emp2 = EmployeeSchedule::new(String::from("emp2"),vec![3,1,2,5,7,10,11,14]);
    let emp3 = EmployeeSchedule::new(String::from("emp3"),vec![3,1,15,13,12,10,14,15,16,17,18,8,9]);
    
    let mut schedules = vec![emp1,emp2,emp3];
    let mut longest_consecutive_slot:u8 = 0;
    let mut longest_working_emp_index : usize =0;

    for (index,schedule) in schedules.iter_mut().enumerate(){
        schedule.set_longest_consecutive_slots();
        if longest_consecutive_slot < schedule.longest_consecutive_slots {
            longest_consecutive_slot = schedule.longest_consecutive_slots;
            longest_working_emp_index = index;
        }
    }

    println!("schedule: {:?}",schedules);
    println!("longest consecutive slot count: {}",longest_consecutive_slot);
    println!("longest_working_employee: {:?}",schedules[longest_working_emp_index]);
    
}
