extern crate communicator;

use communicator::hr::MemberList;
use communicator::hr::Department;

fn main() {
    communicator::try_me();

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