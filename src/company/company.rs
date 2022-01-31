use crate::aquisition::aquisition as aq;
use std::collections::HashMap;

#[derive(Debug)]
struct Company {
    employees: HashMap<String, Vec<String>>,
}

impl Company {
    fn add(&mut self, inter: &str) {
        let mut name = String::new();
        let mut department = String::new();
        let mut flag = false;
        for word in inter.split_whitespace() {
            match word {
                "to" | "TO" | "To" | "tO" | "2" => {
                    flag = true;
                }
                any => {
                    if flag {
                        department.push_str(any);
                    } else {
                        name.push(' ');
                        name.push_str(any);
                        name = name.trim().to_string();
                    }
                }
            }
        }
        if !name.is_empty() && !department.is_empty() {
            let strvec = self.employees.entry(department).or_insert(vec![]);
            strvec.push(name);
            strvec.sort();
        } else {
            println!("Malformed expression");
        }
    }
    fn retrive(&self, dep: &str) {
        let dep = dep.trim();
        match dep {
            "all" => {
                for (department, employees) in &self.employees {
                    Company::printer(&department, employees);
                }
            }
            department => match self.employees.get(department) {
                Some(employees) => Company::printer(department, employees),
                None => {
                    println!("Not a department, departments are:");
                    for each in self.employees.keys() {
                        println!("{}", each);
                    }
                }
            },
        }
    }
    fn printer(department: &str, employees: &Vec<String>) {
        println!("{}", department);
        for each in employees {
            println!("\t {}", each);
        }
    }
    fn new() -> Company {
        Company {
            employees: HashMap::new(),
        }
    }
}

pub fn interact() {
    let mut c = Company::new();
    loop {
        println!("Welcome to the Company");
        let inter = aq::aquire_str();
        let (first, rest) = split_first(&inter);
        let first_word = first.to_lowercase();
        match first_word.as_str() {
            "add" => {
                c.add(rest);
            }
            "see" => c.retrive(rest),
            "q" => break,
            _ => {
                println!("Not an acceptable instruction");
                println!("Instructions are add, see, q");
                continue;
            }
        }
    }
}

fn split_first(s: &str) -> (&str, &str) {
    if let Some(i) = s.find(' ') {
        return (&s[..i], &s[i..]);
    }
    (&s, "")
}
