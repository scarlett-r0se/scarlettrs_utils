

/// #math functions
/// idk, i wanna start doing rust stuff
pub mod math 
{
    ///
    /// # is_even 
    /// Return Values:
    /// 0 if even 
    /// 1 if odd
    pub fn is_even(x:i32) -> i32
    {
        return x % 2;
    }

    /// # fibonacci 
    /// Will take in a number and return the nth fibonacci number 
    /// ```
    /// scarlettrs_utils::math::fibonacci(10);
    /// ```
    /// 
    /// Using the Mathmatitcal Expression:
    /// <br>
    /// F<sub>n</sub> = F<sub>n-1</sub> + F<sub>n-2</sub>
    /// <br>
    /// Where n >2
    /// <br>
    /// 
    /// See <https://en.wikipedia.org/wiki/Fibonacci_sequence>
    /// 
    /// 
    pub fn fibonacci (x:u32) -> u32
    {

        match x 
        {
            0 => return 0,
            1 => return 1,
            2 => return 1,
            _=> {}
        }


        let mut fn_minus_two: u32 = 0;
        let mut fn_minus_one : u32 = 1;
        let mut fn_n: u32 = 1; 
        let mut temp: u32;

        for _i in 1..=x
        {
            fn_n = fn_minus_two + fn_minus_one;
            temp = fn_minus_two;
            fn_minus_two = fn_n;
            fn_minus_one = temp;

        }
        
        
        return fn_n;
    }

    


}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = is_even(5);
//         assert_eq!(result, 1);

//         let result = is_even(6);
//         assert_eq!(result, 0);
//     }
// }