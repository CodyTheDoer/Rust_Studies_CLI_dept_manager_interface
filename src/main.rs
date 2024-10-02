// Using a hash map and vectors
// Create a text interface to allow a user to add employee names to a department in a company
// for example, “Add Sally to Engineering” or “Add Amir to Sales.” 
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;

fn main() {
    println!("Welcome to the Bigsby Department Manager Interface.");
    
    // Gen HashMap 
    let mut company_department_map: HashMap<String, Vec<String>> = HashMap::new(); 
    let dept_index: Vec <&str> = [
        "Sales", 
        "Reporting", 
        "IT",
        "Maintenance",
        "Delivery"
    ].to_vec();
    
    // Gen data
    company_department_map = add_emp_to_dept(company_department_map.clone(), dept_index[0], "Billy Bigsby");
    company_department_map = add_emp_to_dept(company_department_map.clone(), dept_index[0], "Marty Bigsby");
    company_department_map = add_emp_to_dept(company_department_map.clone(), dept_index[0], "Rodger Bigsby");

    company_department_map = add_emp_to_dept(company_department_map.clone(), dept_index[1], "Sally Bigsby");
    company_department_map = add_emp_to_dept(company_department_map.clone(), dept_index[1], "Carl Bigsby");
    
    company_department_map = add_emp_to_dept(company_department_map.clone(), dept_index[2], "Briana Bigsby");
    
    company_department_map = add_emp_to_dept(company_department_map.clone(), dept_index[3], "Todathan Bigsby");
    
    company_department_map = add_emp_to_dept(company_department_map.clone(), dept_index[4], "Karly Bigsby");
    company_department_map = add_emp_to_dept(company_department_map.clone(), dept_index[4], "Carl Bigsby");

    // Print Data for Review
    department_print(company_department_map.clone(), dept_index[0].to_string());
    department_print(company_department_map.clone(), dept_index[1].to_string());
    department_print(company_department_map.clone(), dept_index[2].to_string());
    department_print(company_department_map.clone(), dept_index[3].to_string());
    department_print(company_department_map.clone(), dept_index[4].to_string());
}

fn add_emp_to_dept(mut hashmap: HashMap<String, Vec<String>>, dept: &str, employee: &str) -> HashMap<String, Vec<String>> {
    hashmap.entry(dept.to_string()
        .to_string())
        .or_insert_with(Vec::new)
        .push(format!("{employee}").to_string());

    hashmap
}

fn department_print(hashmap: HashMap<String, Vec<String>>, department: String) {
    for (departments, employees) in hashmap {
        if departments == department {
            println!("Company Department Map: {:?}", department);
            println!("Department Employees: {:?}", employees);
        }
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