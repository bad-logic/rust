//----------------------------------
//      Employee with meeting
//          - Description
//              - Given meeting schedule of employees, we want to determine
//                  the overlapping time
//              
// 
//          - Tools
//              - MultiDimensional Arrays, Nested Loops
//----------------------------------

use std::cmp;

#[derive(Debug)]
#[derive(PartialEq)]
struct Meet{
    start_time:i32,
    end_time: i32,
}

impl  Meet {
    fn new(start:i32,end:i32)->Self{
        Meet{
            start_time:start,
            end_time: end
        }
    }

    fn overlap(&self,meet:&Meet) -> Option<Meet>{
        let mut intersection_time:Meet = Meet::new(0,0);

        let late_starting_meet_time = cmp::max(self.start_time,meet.start_time);
        let early_ending_meet_time = cmp::min(self.end_time,meet.end_time);

        if late_starting_meet_time < early_ending_meet_time{
            intersection_time.start_time = late_starting_meet_time;
            intersection_time.end_time = early_ending_meet_time;
            Some(intersection_time)
        }else{
            None
        }
    }
}

// fn overlap(meet_1:Meet,meet_2:Meet) -> Option<Meet>{
//     let mut intersection_time:Meet = Meet::new(0,0);
//     if cmp::max(meet_1.start_time,meet_2.start_time) < cmp::min(meet_1.end_time,meet_2.end_time){
//         intersection_time.start_time = cmp::max(meet_1.start_time,meet_2.start_time);
//         intersection_time.end_time = cmp::min(meet_1.end_time,meet_2.end_time);
//         Some(intersection_time)
//     }else{
//         None
//     }
// }

fn overlapping_meetings (meetings_a: Vec<Meet>,meeting_b: Vec<Meet>)-> Vec<Meet>{

    let mut intersection = Vec::new();

    for i in 0..meetings_a.len(){
        for j in 0..meeting_b.len(){
            let overlap_time = meetings_a[i].overlap(&meeting_b[j]);
            if overlap_time != None{
                intersection.push(overlap_time.unwrap());
            }
        }
    }

    intersection
}


fn main() {
    let meeting_emp_1 = vec![Meet::new(13,15),Meet::new(15,16),Meet::new(7,9)];
    let meeting_emp_2 = vec![Meet::new(14,15),Meet::new(5,10)];

    let intersections = overlapping_meetings(meeting_emp_1, meeting_emp_2);
    println!("The employees overlapping meeting times are: {:?}",intersections);

}
