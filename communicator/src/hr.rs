use std::collections::HashMap;

use crate::common::first_word;

pub enum Department {
    Sales,
    Engineering,
}

pub struct MemberList {
    pub list: HashMap<String, String>,
}

impl Default for MemberList {
    fn default() -> Self {
        Self::new()
    }
}

impl MemberList {
    pub fn new() -> MemberList {
        let list: HashMap<String, String> = HashMap::new();

        MemberList { list }
    }

    pub fn add_member(&mut self, dept: Department, name: &str) {
        let t = self.convert(&dept);
        self.list.entry(name.to_string()).or_insert(t);
    }

    pub fn add_member_des(&mut self, description: &str) {
        // let mut temp = description; // first_word(description);
        
        let mut v: Vec<&str> = Vec::new();

        // Add Amir to Sales
        // loop {
        //     let result = first_word(&temp);
        //     v.push(result);
        //     if result.len() == 0 || temp.len() == 0 || temp.len() == result.len() {
        //         break;
        //     }
        //     temp = &temp[result.len() + 1..];
        // }

        for txt in description.split_whitespace() {
            v.push(txt);
        }

        if v.get(1) != None && v.get(3) != None {
            self.add_member(self.convert_dept(v[3]), &v[1]);
        }
    }

    fn convert(&self, dept: &Department) -> String {
        let str_dept = match dept {
            Department::Engineering => "Engineering",
            Department::Sales => "Sales",
        };

        str_dept.to_string()
    }

    fn convert_dept(&self, str_dept: &str) -> Department {
        let dept = match str_dept {
            "Engineering" => Department::Engineering,
            "Sales" => Department::Sales,
            _ => todo!(),
        };

        dept
    }

}