use std::{fs::File, io::ErrorKind};

pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    let mut idx = 0;
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            idx = i + 1;
            return &s[..i];
        }
    }

    &s[idx..]
}

pub fn open_file(path: &str) -> File {
    let f = File::open(path);

    let file = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create(path) {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file, but there was a problem: {:?}", e);
                }
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };

    return file;
}