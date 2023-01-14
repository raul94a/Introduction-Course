/*

for and iterators

The for in construct is able to interact with an Iterator in several ways.
As discussed in the section on the Iterator trait, by default the for loop
will apply the into_iter function to the collection. However, this is not
the only means of converting collections into iterators.

into_iter, iter and iter_mut all handle the conversion of a collection into
an iterator in different ways, by providing different views on the data within.

iter - This borrows each element of the collection through each iteration.
Thus leaving the collection untouched and available for reuse after the loop.

into_iter - This consumes the collection so that on each iteration the exact data is provided.
Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.

iter_mut - This mutably borrows each element of the collection, allowing for the collection to be modified in place.



*/

fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    //iter
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    //ITER INTO -> the collection is cleared after the loop = not available
    {
        let names = vec!["Bob", "Frank", "Ferris"];

        for name in names.into_iter() {
            match name {
                
                "Ferris" => println!("There is a rustacean among us!"),
                //If you don't want to do anything with the values, just pass a () to the assignment!
                _ => (),
            }
        }
        //collection not available. It's been consumed in the loop
        //println!("names: {:?}", names);
    }

    //ITER MUT
    {
        let mut names = vec!["Bob", "Frank", "Ferris"];

        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!",
                //if you don't wanna change the rest of the values!
                _ => name,
            }
        }
        println!("names: {:?}", names);
    }

}
