// use std::collections::HashMap;

pub fn middle_function() -> f32 {
    let v: Vec<i32> = vec![13, 5, 8, 10, 12, 10, 9];

    let mut sum: i32 = 0;
    let mut cnt: i32 = 0;
    for i in v {
        sum += i;
        cnt += 1;
    }
    sum as f32/cnt as f32
}

pub fn min_function() -> i32 {
    let v: Vec<i32> = vec![13, 5, 8, 10, 12, 10, 9];

    let mut min: i32 = v[0];
    for i in v {
        if i < min {
            min = i;
        }
    }
    min
}

pub fn freq_function() -> i32 {
    let v: Vec<i32> = vec![13, 5, 8, 10, 10, 12, 10, 8, 10, 8, 9];

    let mut max: usize = 0;
    let mut max_key = &0;
    for i in &v {
        let count = v.iter().filter(|&n| n == i).count();
        
        if max < count {
            max = count;
            max_key = i;
        }
    }
    // let mut map = HashMap::new();
    // for i in v {
    //     let count = map.entry(i).or_insert(0);
    //     *count += 1;
    // }

    // let mut max = 0;
    // let mut max_key = 0;
    // for (key, value) in map {
    //     if max < value {
    //         max = value;
    //         max_key = key;
    //     }
    // }
    *max_key
}

pub fn make_pig_latin(_str: &str) -> String {

    let i = &_str[..1];
    //let mut t: String = String::from("");

    if i == "a" || i == "e" || i == "i" || i == "o" || i == "u" {
        let t = format!("{}-hay", &_str);
        return t;
    }
    else {
        let t = format!("{}-{}ay", &_str[1..], i);
        return t;
    }
}