//import os
//import sys
use std::env;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    //os.chdir("/inkbox/book/split")
    env::set_current_dir("/inkbox/book/split/").unwrap();

    //book = open("../book.txt", "r")
    let mut book = File::open("../book.txt").unwrap();

    //text = book.read()
    let mut text: String = String::new();
    book.read_to_string(&mut text);

    // book.close()
    drop(book);

    //booklist = text.split(" ")
    let booklist = text.split(" ");

    //words_number = sys.argv[1]
    let args: Vec<String> = env::args().collect();
    let words_number = &args[1];

    //words_number_int = int(words_number)
    let words_number_int: i32 = words_number.parse().unwrap();

    // split_booklist = [booklist[x:x+words_number_int] for x in range(0, len(booklist),words_number_int)]

    let mut split_booklist: Vec<Vec<String>> = Vec::new();
    let mut count = 0;
    let mut one_file: Vec<String> = Vec::new();
    for word in booklist {
        count += 1;
        one_file.push(word.to_string());
        if count == words_number_int {
            count = 0;
            split_booklist.push(one_file);
            one_file = Vec::new();
        }
    }

    //println!("{:?}", split_booklist);

    let mut count_file_name = 0;
    for one_file_string in split_booklist {
        let mut purged_vec: String = String::new();
        for string in one_file_string {
            let mut new_string: String = string;

            //let abc = vec![
            //    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
            //    "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "ą", "ć", "ę", "ł", "ń", "ó",
            //    "ś", "ź", "ż", ",",
            //];
            /*
            for letter in abc {
                new_string = new_string.replace(
                    &(letter.to_string() + &"\n".to_string()),
                    &(letter.to_string() + &" "),
                );
                new_string = new_string.replace(
                    &(letter.to_string().to_uppercase() + &"\n".to_string()),
                    &(letter.to_string().to_uppercase() + &" "),
                );
            }
            */
            if new_string.is_empty() {
                continue;
            }

            if !new_string.contains(".\r\n") {
                new_string = new_string.replace("\r\n", "");
            }

            new_string = new_string.replace("  ", " ");

            purged_vec.push_str(&new_string);

            if !new_string.contains("\n") {
                purged_vec.push_str(" ");
            }
        }

        let mut new_file = File::create(count_file_name.to_string()).unwrap();

        new_file.write_all(purged_vec.as_bytes());
        count_file_name += 1;
    }
}
