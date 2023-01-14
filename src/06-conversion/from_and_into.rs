/**

From and Into
The From and Into traits are inherently linked, and this is actually part of its implementation. 
If you are able to convert type A from type B, then it should be easy to believe that we should
be able to convert type B to type A.

From
The From trait allows for a type to define how to create itself from another type, 
hence providing a very simple mechanism for converting between several types.
There are numerous implementations of this trait within the standard library 
for conversion of primitive and common types.

For example we can easily convert a str into a String


let my_str = "hello";
let my_string = String::from(my_str);


Into
The Into trait is simply the reciprocal of the From trait. 
That is, if you have implemented the From trait for your type, Into will call it when necessary.

Using the Into trait will typically require specification of the type to convert into 
as the compiler is unable to determine this most of the time. 
However this is a small trade-off considering we get the functionality for free.

*/

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

//implement the trait From for types of i32 for the type Number
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let myNum = 32i32;
    let num = Number::from(30);
    let num2 = Number::from(myNum);
    //it's mandatory to specify the type when using into
    let num3 : Number = myNum.into();
    println!("My number is {:?}, num2 is: {:?}, num3 is {:?}", num, num2, num3.value);
}

