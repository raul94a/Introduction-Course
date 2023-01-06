fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    //i is Signed integer, U is Unsigned integer
    //Here, if we change the type from i32 to u32 the file cannot be
    //compiled, as the operation will be negative (and an unsigned value cannot be negative )
    //It only would work if the first term was >= 2 (in other words, if the operation is NOT negative)


    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    //<< is the left-shift operator and moves the bites to the left
    //>> is the right-shift operator and moves the bites to the right

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 128 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u64);
}
