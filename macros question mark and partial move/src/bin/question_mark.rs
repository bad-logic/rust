

#[derive(Debug)]
enum MathError{
    DivisionError_DivisionByZero,
    LogError_NonPositiveLogarithm,
    SqrtError_NegativeSquareRoot,
}

type MathResult = Result<(),MathError>;


fn division(x:f64,y:f64)->MathResult{
    if y == 0.0 {
        Err(MathError::DivisionError_DivisionByZero)
    }else{
        println!("The division is successful and has a result of {}",x/y);
        Ok(())
    }
}

fn sqrt(x:f64) -> MathResult{
    if x < 0.0 {
        Err(MathError::SqrtError_NegativeSquareRoot)
    }else{
        println!("The square root operation was successful and has a result of {}",x.sqrt());
        Ok(())
    }
}

fn ln(x:f64) -> MathResult{
    if x <= 0.0 {
        Err(MathError::LogError_NonPositiveLogarithm)
    }else{
        println!("The log operation was successful and has a result of {}",x.ln());
        Ok(())
    }
}

fn operations(x:f64,y:f64)-> MathResult{
    division(x,y)?;
    sqrt(x)?;
    ln(x)?;
    Ok(())
}

fn main(){

    let result = operations(0.0,10.0);
    if result.is_ok(){
        println!("All the operations were executed successfully");
    }else{
        println!("{:?}",result);
    }

}