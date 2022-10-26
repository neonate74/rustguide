extern crate communicator;

use communicator::hr::MemberList;
use communicator::hr::Department;

#[test]
fn it_adds_two() {
    assert_eq!("aaa", communicator::common::first_word("aaa bbb"));
}

#[test]
fn add_member() {
    let mut mem: MemberList = MemberList::new();

    mem::add_member(Department::Engineering, "Monk");

    assert_eq!(Department::Engineering, mem::list[0].get("Monk"));
}