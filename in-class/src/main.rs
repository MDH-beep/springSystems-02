fn fahrenheit_to_celsius(f:f64)-> f64 {
    
(f-32.0)/1.8
    
}

fn celsius_to_fahrenheit(c:f64)-> f64 {
    
    c * 1.8 + 32.0
    
}

fn main() {
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

   fn is_even(n: i32) -> bool
   {
    
   }

}
