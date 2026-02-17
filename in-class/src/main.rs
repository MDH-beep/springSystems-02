fn fahrenheit_to_celsius(f:f64)-> f64 {
    
(f-32.0)/1.8
    
}

fn celsius_to_fahrenheit(c:f64)-> f64 {
    
    c * 1.8 + 32.0
    
}

fn is_even (n: i32) -> bool
{ if n % 2 == 0  
{return true;}
else {
return false;}
}

fn check_guess(guess: i32, secret: i32) -> i32
{
    if guess == secret {
        return 0;}
        else if guess > secret {
            return 1;}
        else {
            return -1;
        }
        }  


fn main() {
    // Assignment 1
   const W: u32= 32;
   let mut x: f64 = 50.0;
   let mut counter = 6;
   while counter != 0 {
    println!("F to C = {}", fahrenheit_to_celsius(x));
    x += 1.0;
    counter -= 1;
   }

   let mut y :f64 = 10.0;
   let mut counter =6;
   while counter != 0 {
    println!("C to F = {}", celsius_to_fahrenheit(y));
    y += 1.0;
    counter -= 1;
   }
// Assignment 2
  let numbers = [ 2, 22, 54, 66, 20, 15, 40, 12, 6, 1];
  for num in numbers.iter()
  // for loop
  {
  if num % 3 == 0 && num % 5 == 0 {
        println!("FizzBuzz");
    } 
    else if num % 3 == 0 {
        println!("Fizz");
    } 
    else if num % 5 == 0 {
        println!("Buzz");
    } 
    else {
        if is_even(*num)
        {
            println!("Even :{}",num);
        } 
        else 
        {
            println!("Odd :{}",num);
        }
    }
}
 // while loop
    let mut counter = 0;
    let mut sum = 0;
    while counter < numbers.len() 
    {
        sum += numbers[counter];
        counter += 1;
    }
    println!("Total sum : {}", sum);

    // largest number loop
     let mut index = 0;
    let mut largest = numbers[0];

    loop {
        if index >= numbers.len() {
            break;
        }

        if numbers[index] > largest {
            largest = numbers[index];
        }

        index += 1;
    }

    println!("Largest number: {}", largest);

    // Assignment 3
     let secret = 22;
      let mut counter =0;
      let mut guess;

      loop {
        counter += 1;

        if counter == 1 {
            guess = 11;
        }
        else if counter == 2 {
            guess = 25;
        }
        else {
            guess = 22;
        }
         let result = check_guess(guess, secret);
        println!("Guess: {}, Result: {}", guess, result);
        if result == 0 {
        println!("Amount of guesses: {}", counter);
            break;
        }
        
      }
}
 


   


