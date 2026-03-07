fn main(){
    
pub fn fizz_buzz(n: i32) -> Vec<String> {
    let n = n as usize;
    let mut ret: Vec<String> = Vec::with_capacity(n);

    for i in 1..=n {
        let res = if i % 15 == 0 {
            "FizzBuzz".to_string()
        } else if i % 3 == 0 {
            "Fizz".to_string()
        } else if i % 5 == 0 {
            "Buzz".to_string()
        } else {
            i.to_string()
        };

        ret.push(res);
    }
    ret
}
}