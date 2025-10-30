use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::vec;

fn main() {
    let mut word_collection: HashMap<String, i64> = HashMap::new();

    word_counter(get_source().unwrap(), &mut word_collection);

    let t = format!("{word_collection:?}");

    println!("{}", t)
}

fn word_counter(source: Vec<Vec<String>>, list: &mut HashMap<String, i64>) {
    for l in source {
        for w in l {
            list.entry(w)
            .and_modify(| c| *c += 1)
            .or_insert(1);
        }
    }
}

fn get_source() -> Result<Vec<Vec<String>>, String>{
    let text: Result<File, std::io::Error> = File::open("./source.txt"); 
    let mut buffer: String = String::new();

    match text {
        Ok(mut f) => {
            let res = f.read_to_string(&mut buffer);

            match res {
                Ok(_) => {
                    let mut words_vec: Vec<Vec<String>> = vec![];
                    let mut clear_buffer: String = String::new();

                    for l in buffer.lines() {
                        clear_buffer = l.chars()
                        .filter(|c| c.is_alphanumeric() || c.is_whitespace()) 
                        .collect()
                       
                    }

                    for l in clear_buffer.to_lowercase().lines() {
                        words_vec.push(
                            l.split(" ")
                            .map(|w| w.to_string())
                            .collect()
                        );
                    }

                    return Ok(words_vec)
                },
                Err(e) => return Err(e.kind().to_string())
            }
        },
        Err(e) => return Err(e.kind().to_string()),
    }
}
