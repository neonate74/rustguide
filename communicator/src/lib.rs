pub mod network;
pub mod hr;
pub mod common;
mod outermost;

use hr::MemberList;
use hr::Department;

pub fn try_me() {
    let re = outermost::middle_function();
    println!("The result of middle-secret function is {}", re);

    let min: i32 = outermost::min_function();
    println!("The minimum value is {}", min);

    // outermost::middle_secret_function();
    let freq = outermost::freq_function();
    println!("The most frequently value is {}", freq);

    let a = [0, 1, 2, 3, 4];
    let b = &a[1..3];

    for i in a {
        println!("slice num is {}", i);
    }

    let c = "apron";
    println!("The pig-Latin for {} is {}", c, outermost::make_pig_latin(c));
}

fn test_me() {
    let mut mem = MemberList::new();

    mem.add_member(Department::Engineering, "Sally");
    mem.add_member_des("Add Amir to Sales");
    mem.add_member(Department::Engineering, "Bob");
    mem.add_member(Department::Engineering, "Andrea");

    println!("{:?}", mem.list);

    let mut vec : Vec<(&String, &String)> = mem.list.iter().collect();
    vec.sort_by(|a, b| a.0.cmp(&b.0));
    println!("{:?}", vec);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        try_me();
    }

    #[test]
    fn it_test() {
        test_me();
    }

    #[test]
    fn temp_test() {
        let v = vec![1, 2, 3];

        let a = v[0];
        let b = v[1];
        let c = v[3];
        println!("{}", a);
        println!("{}", b);
        println!("{}", c);
    }
}
