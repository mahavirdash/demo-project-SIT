fn main()
{
    immutable_variable();
    mutable_variable();
}
fn immutable_variable()
{
    println!("immutable variable");
    let x=10;
    println!("the value of x is {}",x);  
    println!();
}
fn mutable_variable()
{
    println!("muttable variable");
    let mut x: i32=27;
    println!("the value of x is {}",x);
    x=1565;
    println!("the value of x is {}",x);
    println!();
}