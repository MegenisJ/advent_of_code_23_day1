use std::fs;
fn main() {
    let mut total = 0;
    let letters: Vec<char> = ('a'..='z').collect();
    
    for line in fs::read_to_string("input.txt").unwrap().lines(){
        let mut result = String::new();
        
        let ints = &line.replace(&*letters, "");
        
        result += &ints.chars().next().unwrap().to_string();
        result += &ints.chars().last().unwrap().to_string();
       
        total += result.parse::<i32>().unwrap();
    }
    
    println!("{}",total);
}

