use std::collections::HashMap;
use std::fs;



#[derive(Debug)]
struct WordCounter {
    file_path: &'static str,
    words: Vec<String>,
    frequency: HashMap<String, u32>
}


impl WordCounter {

    fn new(path: &'static str) -> Result<Self, std::io::Error> {
        let words = Self::load_file(path)?;
        let fqc = Self::calculate_frequency(words.clone());

        Ok(Self {
            file_path: path,
            words: words,
            frequency: fqc
        })
    }

    fn load_file(path: &str) -> Result<Vec<String>, std::io::Error>{
        let content = fs::read_to_string(path)?;
        let vec_content = content.split_whitespace()
                                                             .map(|s| s.to_string())
                                                             .collect();
        Ok(vec_content)
    }

    fn calculate_frequency(words:  Vec<String>) -> HashMap<String, u32>{
        let mut fqcy_map =  HashMap::new();
        for word in words{
            match fqcy_map.get(&word) {
                Some(count) => { fqcy_map.insert(word.to_owned(), count +1); }
                None => { fqcy_map.insert(word.to_owned(), 1); }
            }
        }
        fqcy_map
    }

    pub fn get_top_5(&self) -> HashMap<String, u32> {
        let mut fqcy_top_5 =  HashMap::new();
        
        for (w, f) in self.frequency.clone().into_iter() {
            if fqcy_top_5.len() < 5 {
                fqcy_top_5.insert(w, f);
            }else {
                for (top_5_w, top_5_f) in fqcy_top_5.clone().into_iter() {
                    if f > top_5_f {
                        fqcy_top_5.remove(&top_5_w);
                        fqcy_top_5.insert(w, f);
                        break;
                    }
                }
            }
        }
        fqcy_top_5
    }
        
}




fn main() {
    let counter = WordCounter::new("src/text.txt").expect("Unable to load file");
    println!("{:?}", counter);

    let top_5 = counter.get_top_5();
    println!("{:?}", top_5);


}

