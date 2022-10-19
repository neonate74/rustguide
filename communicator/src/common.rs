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