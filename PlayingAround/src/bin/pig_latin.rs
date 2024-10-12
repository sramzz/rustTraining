/*
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added,
so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay).
Keep in mind the details about UTF-8 encoding!
*/
//Helper to find out if the first letter is consonant
// struct LetterInWord{
//     fisrt_letter: char,
//     is_consonant: bool,
// }

use std::fmt::format;

fn word_translation (word:&str) -> String {
    let mut word_without_first_char = String::new();
    for i in word.chars().enumerate(){
        //print!("{}",i.0==0);
        if i.0 == 0{continue;}
        //print!("-----\n{}",i.0);
        //print!("\n{}",word_without_first_char);
        word_without_first_char = format!("{}{}",word_without_first_char,i.1)
                
    }
    //print!("{}\n\n",word_without_first_char);
    word_without_first_char
}
fn is_first_letter_consonant(word: &str) -> (bool,char) {
    let consonants: [char; 21] = [
        'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w',
        'x', 'y', 'z',
    ];
    let first_letter = word.chars().next().unwrap();
    (consonants.contains(&first_letter),first_letter)
}
//Helper to find the position of the the first consonant in a string
// fn find_first_consonant(word: &str) {}
//Takes a string and converts it to pig latin
fn convert_to_pig(word: &str) -> String {
    let first_letter:(bool,char) = is_first_letter_consonant(word);
    if first_letter.0 == true{
        //let dummy1 = word.clone()
        format!("{}{}{}{}",word_translation(word),"-",first_letter.1,"ay")
    }
    else{
        format!("{}{}",word,"-hay")
    }
}
fn main() {
    let wordy: &str = "lisa";
    let wordy_translated = convert_to_pig(wordy);
    print!("{} is {} in pig-latin",wordy,wordy_translated)
}

