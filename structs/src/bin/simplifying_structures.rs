//---------------------------------------
// 
//      - Simplifying Structures
// 
// 
// 
//---------------------------------------

struct B {
    f2: u32
}
struct C {
    f1: u32,
    f3: u32
}
struct A {
    b: B,
    c: C
}

fn fn1(b:&mut B) -> &u32 {&b.f2}

fn fn2(c:&mut C) -> u32 { c.f1+ c.f3}

fn fn3(a: &mut A){
    let x = fn1(&mut a.b);
    let y = fn2(&mut a.c);

    println!("{}, {}",x,y);
}

fn main(){
    let mut var = A{
        b: B { f2: 23 },
        c:C {
            f1: 12,
            f3: 45
        }
    };

    fn3(&mut var);
}