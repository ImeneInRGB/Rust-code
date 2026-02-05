/* my own reverse function instead of just using ".chars().rev().collect" */

pub fn reverse(input: &str) -> String {
  
    
    let mut reversed_word = String::from("");
    let word_to_chars: Vec<char> = input.chars().collect();
    let mut i = word_to_chars.len();
    
    
    while i > 0 {
        i-=1;
        reversed_word.push(word_to_chars[i]);    
    }
    
    reversed_word
}

fn main (){
    let mut word: &str = "word";
    
    print!("The reversed word is {}",reverse(word));
    }