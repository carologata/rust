fn main() {
    
    //1
    let y = {
        let z = 3;
        z + 1
    };
    println!("y: {y}");
    
    //2
    another_function(5, 'h');

    //3
    let x = five();
    println!("x: {x}");

    //4
    let result = plus_one(5);
    println!("result: {result}");
}

fn another_function(x: i32, unit_label: char) {
    
    println!("The measurement is: {x}{unit_label}");

}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 
{
    x + 1
}