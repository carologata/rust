fn main() {

    let x = 3;

    if x < 5
    {
        println!("condition is true");
    }
    else
    {
        println!("contidition is false");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;

    let num = if condition { 5 } else { 7 };
    println!("The value is {num}");
}
