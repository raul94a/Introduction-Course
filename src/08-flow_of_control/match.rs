fn main(){
    let number = 13;
    println!("Tell  me  about {}",number);

    match number {
        13    =>  println!("EL TRECE"),
        _=>println!("OTRO")
    }
}