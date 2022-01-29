use std::env;
use std::fs;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

fn main() {

    let args: Vec<String> = env::args().collect();
    let arguments = std::env::args(); // foo --no-bar --baz 42 --qux 'To be?'
    let arguments = arguments::parse(arguments).unwrap();

    println!("{:?}", args);
    let filename = match arguments.get::<String>("file"){
        Some(a) => a,
        None => panic!("missing argument --file")
    };

    // --snip--
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let words: Vec<&str> = contents.lines().collect();
    
    let mut words_map: HashMap<&str,i32> = HashMap::new();

    println!("length of words array {}", words.len());
    for word in words {
        if UnicodeSegmentation::graphemes(word,true).count() == 5{
            words_map.insert(word, 0);
        }
    }

    for (k,_) in words_map {
            println!("{}",k)
    }
    
}
