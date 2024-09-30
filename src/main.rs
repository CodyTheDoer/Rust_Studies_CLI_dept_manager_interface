// Using a hash map and vectors
// Create a text interface to allow a user to add employee names to a department in a company
// for example, “Add Sally to Engineering” or “Add Amir to Sales.” 
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;

fn main() {
    println!("Welcome to the Bigsby Department Manager Interface.");
    
    let department_key: Vec <&str> = [
        "Sales", 
        "Reporting", 
        "IT",
        "Maintenance",
    ].to_vec();

    println!("department_key: {:?}", department_key);

    // Gen HashMap and gen data
    let mut company_department_map: HashMap<String, Vec<String>> = HashMap::new(); 
    company_department_map.entry(department_key[0]
        .to_string())
        .or_insert_with(Vec::new)
        .push("Carl Bigsby".to_string());
    company_department_map.entry(department_key[0]
        .to_string())
        .or_insert_with(Vec::new)
        .push("Brian Bigsby".to_string());
    company_department_map.entry(department_key[1]
        .to_string())
        .or_insert_with(Vec::new)
        .push("Todathan Bigsby".to_string());

    // if let Some(sales_department) = company_department_map.get("Sales") {
    //     println!("Sales Department: {:?}", sales_department);
    // }

    department_print(company_department_map, 0);

    for department in company_department_map.iter() {
        println!("Company Department Map: {:?}", department);
    }
}

fn department_print(hashmap: HashMap<String>, index: u64) {
    if let Some(department) = hashmap.get(index) {
        println!("Department: {:?}", department);
    }
}

// fn manual_entry(v: &mut Vec<String>) {
//     println!("Please enter your selection: 'X' to Exit");
    
//     let mut user_entry = String::new();
//     let mut clean_float = Vec::new();
    
//     io::stdin()
//         .read_line(&mut user_entry)
//         .expect("Failed to read line");
    
//     user_entry = user_entry.trim().to_string();

//     for char in user_entry.chars() {
//         if char.is_ascii(){
//             clean_float.push(char as u8)
//         }
//     }
    
//     if !clean_float.is_empty() {
//         v.push(String::from_utf8(clean_float).expect("Our bytes should be valid utf8"));
//     }

//     for char in user_entry.chars() {
//         if char == 'x' || char == 'X'{
//             println!("Exiting...");
//         }
//     }
// }