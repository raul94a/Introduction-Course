// `NanoSecond`, `Inch`, and `U64` are new names for `u64`.
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

struct BaseCircle {
    radius: f32
}
type Circle = BaseCircle;

fn main() {
    // `NanoSecond` = `Inch` = `U64` = `u64`.
    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;
    let _circle = Circle {
        radius: 8.66_f32 
    };
    let area = area(_circle);
    //If the same _circle it is used for the perimeter,
    //the compiler is gonna raise a NOT IMPLEMENET COPY TRAIT ERROR
    //so insted of that...
   // let per = perimeter(_circle);
   //...we can do

   let per = perimeter(Circle { radius:8.6_f32});

    println!("Area is {} and perimeter is {}", area,per);
    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}


fn area(circle: Circle) ->f32{
    return circle.radius * circle.radius *3.1416_f32
}

fn perimeter(circle: Circle) ->f32 {
    return circle.radius * 2.0 * 3.1416f32;
}