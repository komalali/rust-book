use std::io;
use std::collections::HashMap;

fn main() {
    println!("Hi, welcome to Dunder Mifflin Paper Company. We hope you're having a terrific day!");

    let mut departments = HashMap::new();
    loop {
        println!("Please enter a command.");
        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input)
            .expect("Failed to read line");
        let mut iter = user_input.split_whitespace();

        match iter.next() {
            Some("Add") | Some("add") => {
                let employee = match iter.next() {
                    Some(name) => String::from(name),
                    None => panic!("Missing employee name.")
                };
                match iter.next() {
                    Some(_) => (),
                    None => panic!("Missing \"to [department]\".")
                };
                let department = match iter.next() {
                    Some(name) => String::from(name),
                    None => panic!("Missing department name.")
                };
                let employees: &mut Vec<String> = departments
                    .entry(department)
                    .or_insert(Vec::new());
                employees.push(employee);
            },
            None => panic!("You gotta say something bud."),
            Some("quit") => {
                println!("Okay see ya!");
                break;
            },
            Some(anything) => {
                match departments.get(anything) {
                    Some(val) => println!("The following people work in the {} department: {:?}", anything, val),
                    None => println!("No matching department.")
                }
            },
        }
    }
}
