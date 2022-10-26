use std::{fs::File, io::{ErrorKind, Read}};

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

pub fn open_file(path: &str) -> Result<File, String> {
    let f = File::open(path);

    match f {
        Ok(file) => Ok(file),
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create(path) {
                Ok(fc) => Ok(fc),
                Err(e) => Err(format!("Tried to create file, but there was a problem: {:?}", e.to_string()))
            }
        },
        Err(error) => Err(format!("There was a problem opening the file: {:?}", error))
    }
}

pub fn read_string_from_file(path: &str) -> Result<String, String> {
    let mut f = match open_file(path) {
        Ok(file) => file,
        Err(e) => {
            return Err(format!("read_string_from_file(path) > open_file(path): {:?}", e));
        }
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(format!("read_string_from_file(path) > read_to_string(buf): {:?}", e)),
    }
}