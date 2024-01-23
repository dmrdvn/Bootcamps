use std::io;

#[allow(non_snake_case)]
struct WordCounter {
    word: String,
}

impl WordCounter {
    fn new(text: &str) -> WordCounter {
        
        //Instance Struct
        WordCounter {
            word: text.to_string(),
        }

    }

    fn count_words(&self) -> Result<usize, &'static str> {

        //Checking if word is empty
        if self.word.trim().is_empty() {
            return Err("Word is empty")
        }
        
        //Splitting and collecting the word and returning the length
        let words: Vec<&str> = self.word.split_whitespace().collect();
        Ok(words.len())
        
    }
}


fn main() {

   //Waiting for text from the user
   println!("Enter some text:");
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("Failed to read line");

   // Trim the input and create WordCounter struct
   let word_counter = WordCounter::new(&input.trim());

   // Count the words and print the result (or error)
   match word_counter.count_words() {
       Ok(word_count) => println!("Word count: {}", word_count),
       Err(err) => println!("{}", err),
   }
    
}
