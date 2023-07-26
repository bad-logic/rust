//--------------------------------------------------
// function within a trait can use each other
//--------------------------------------------------

struct Data{
    some_data:Vec<i32>
}

trait BasicStats{
    fn mean(&self) -> f32;
    fn variance(&self) -> f32;
}

impl BasicStats for Data{
    fn mean(&self) -> f32{
        let mut sum:f32 = 0.0;
        for i in self.some_data.iter(){
            sum += *i as f32;
        }

        sum / self.some_data.len() as f32
    }

    fn variance(&self) -> f32{
        let mean:f32 = self.mean(); // using same trait function mean
        let mut sum_squared_diff: f32 = 0.0;
        for i in self.some_data.iter(){
            sum_squared_diff += (*i as f32 - mean) * (*i as f32 - mean)
        }

        sum_squared_diff / self.some_data.len() as f32
    }
}

fn main() {

    let my_data:Data = Data{
        some_data: vec![1,2,3,4,5,6,7,8]
    };

    println!("The mean and variance of the given data is {} and {} resp",my_data.mean(),my_data.variance());
}