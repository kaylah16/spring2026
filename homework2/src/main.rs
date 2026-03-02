fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect(); //text separated to words and inserted to a vector
    let mut max_count: usize = 0;
    let mut max_word: &str = "";
    
    //iterate each word
    for i in 0..words.len() {
        //get current word
        let current_word = words[i];
        let mut repeat: usize = 0; // keep track of repeated words

        //check if word is frequent
        for j in 0..words.len(){
            //reference counter
            let ref_count: &mut usize = &mut repeat;

            //if word the same as current, increment count
            if words[j] == current_word {
                *ref_count += 1;
            }
        }
        //if new repeated word is bigger than max_count, reassign
        if repeat > max_count{
            //assign max count
            max_count = repeat;
            //get max_word
            max_word = current_word;
        }
    }
    
    (max_word.to_string(), max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
