fn main() {
    //First we will ask to the user to input two numbers
    
    println!("Please enter the first number: ");
    let mut num1 = String::new();
    std::io::stdin()
    .read_line(&mut num1)
    .expect("Failed to read line");

    println!("Please enter the second number: ");
    let mut num2 = String::new();
    std::io::stdin()
    .read_line(&mut num2)
    .expect("Failed to read line");

    //We will convert the string to a number
    let num1: f32 = num1.trim().parse().expect("Please type a number!");
    let num2: f32 = num2.trim().parse().expect("Please type a number!");    

    //We will ask the user to input the operation
    println!("Please enter the operation: ");
    let mut operation = String::new();
    std::io::stdin()
    .read_line(&mut operation)
    .expect("Failed to read line");

    //We will now ask if it is addition or subtraction

    if operation.trim() == "+" {
        let result = num1 + num2;
        println!("The result is: {}", result);
    } else if operation.trim() == "-" {
        let result = num1 - num2;
        println!("The result is: {}", result);
  
    } else {
        println!("Invalid operation");
    }


}
//We need to read two numbers from the user.
//we need to read the operation - additon, subtraction, multiplication, division
//we need to perform the operation on the two numbers
//we need to display the result to the user
