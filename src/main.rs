use scarlettrs_utils;

fn main ()
{
    //println!("{}", scarlettrs_utils::math::is_even(5));
    //println!("{}", scarlettrs_utils::math::is_even(6));


    let val:u32 = 20;
    //scarlettrs_utils::math::fibonacci(10);

    println!("The {} Fibonacci Number is {}", val, scarlettrs_utils::math::fibonacci(val));

}