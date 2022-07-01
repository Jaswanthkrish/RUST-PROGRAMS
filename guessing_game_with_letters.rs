use std::cmp::Ordering;
use std::io;
use fastrand;
fn main(){

    let length = 1;
    

    let rand_str: String =std::iter::repeat_with(fastrand::lowercase)
            .take(length)
            .collect();
        

    println!("The secret alphabet is : {}",rand_str);


        println!("Please enter any alphabet");
        let mut input_str= String::new();

        
        input_str.clear();
        io::stdin().read_line(&mut input_str).unwrap();
        println!("You guessed: {}",input_str);

        match rand_str.ne(&input_str){
            true => println!("You win!"),
            false=> match input_str.cmp(&rand_str){
            
                Ordering::Less => println!("Letter is too small! "),
                Ordering::Greater => println!("Letter is too big!"),
                Ordering::Equal => {
                    println!("You win");
                }
            }
            
        }


    
}