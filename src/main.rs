use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::vec;

fn main() {
    let mut word_collection: HashMap<String, i64> = HashMap::new();

    word_counter(get_source().unwrap(), &mut word_collection);
    register_result(&word_collection).unwrap();
}

fn word_counter(source: Vec<Vec<String>>, list: &mut HashMap<String, i64>) {
    for l in source {
        for w in l {
            list.entry(w).and_modify(|c| *c += 1).or_insert(1);
        }
    }
}

fn get_source() -> Result<Vec<Vec<String>>, String> {
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
                        // remove headers from some message apps, like 21/03/25 Doe:
                        let line = match l.split_once(": ") {
                            Some((_, text)) => text,
                            None => l,
                        };

                        let cleaned: String = line
                            .chars()
                            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                            .collect();

                        clear_buffer.push_str(&cleaned);
                        clear_buffer.push('\n');
                    }

                    for l in clear_buffer.to_lowercase().lines() {
                        words_vec.push(l.split(" ").map(|w| w.to_string()).collect());
                    }

                    return Ok(words_vec);
                }
                Err(e) => return Err(e.kind().to_string()),
            }
        }
        Err(e) => return Err(e.kind().to_string()),
    }
}

fn register_result(list: &HashMap<String, i64>) -> Result<(), String> {
    let mut sorted_list: Vec<(&String, &i64)> = list.iter().collect();
    
    sorted_list.sort_by(|a: &(&String, &i64), b: &(&String, &i64)| b.1.cmp(a.1));

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("./result.txt");
    let mut buffer = String::new();

    for (k, v) in sorted_list {
        buffer.push_str(format!("repeated: {k} {v} times\n").as_str());
    }

    match file {
        Ok(mut f) => match f.write(buffer.as_bytes()) {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e.kind().to_string()),
        },
        Err(e) => return Err(e.kind().to_string()),
    }
}
