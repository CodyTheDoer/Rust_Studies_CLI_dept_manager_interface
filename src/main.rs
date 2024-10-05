// Using a hash map and vectors
// Create a text interface to allow a user to add employee names to a department in a company
// for example, “Add Sally to Engineering” or “Add Amir to Sales.” 
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::io;
use std::collections::HashMap;

fn main() {
    println!("Welcome to the Bigsby Department Manager Interface.");
    
    // Gen HashMap and departments
    let mut company_department_map: HashMap<String, Vec<String>> = HashMap::new(); 
    let dept_index: Vec <&str> = [
        "Sales", 
        "Reporting", 
        "IT",
        "Maintenance",
        "Shipping"
    ].to_vec();
    
    // Gen current employee data
    let dept_employees_sales: Vec <&str> = [
        "Marty Bigsby", 
        "Rodger Bigsby", 
        "Billy Bigsby", 
        "Bille Bigsby",
    ].to_vec();

    let dept_employees_reporting:  Vec <&str> = [
        "Sally Bigsby", 
        "Carl Bigsby",
    ].to_vec();

    let dept_employees_it:  Vec <&str> = [
        "Briana Bigsby",
    ].to_vec();
    
    let dept_employees_maintenance:  Vec <&str> = [
        "Todathan Bigsby",
    ].to_vec();
    
    let dept_employees_shipping:  Vec <&str> = [
        "Karly Bigsby", 
        "Carly Bigsby",
    ].to_vec();
    
    // store data
    company_department_map = add_emp_batch_to_dept(company_department_map.clone(), dept_index.clone(), 0, dept_employees_sales);
    company_department_map = add_emp_batch_to_dept(company_department_map.clone(), dept_index.clone(), 1, dept_employees_reporting);
    company_department_map = add_emp_batch_to_dept(company_department_map.clone(), dept_index.clone(), 2, dept_employees_it);
    company_department_map = add_emp_batch_to_dept(company_department_map.clone(), dept_index.clone(), 3, dept_employees_maintenance);
    company_department_map = add_emp_batch_to_dept(company_department_map.clone(), dept_index.clone(), 4, dept_employees_shipping);

    println!("What would you like to do?");
    println!("Example Actions: ");
    println!("   Add 'Employee' to 'Department'");
    println!("   Review 'Department'");
    println!("   Review all employees");

    let mut user_entry = Vec::new();
    manual_entry(&mut user_entry);

    company_department_map = user_interchange(company_department_map.clone(), dept_index.clone(), &user_entry.clone());

    while main_again() == true {
        println!("What would you like to do?");
        println!("Example Actions: ");
        println!("   Add 'Employee' to 'Department'");
        println!("   Review 'Department'");
        println!("   Review all employees");
    
        let mut user_entry = Vec::new();
        manual_entry(&mut user_entry);
    
        company_department_map = user_interchange(company_department_map.clone(), dept_index.clone(), &user_entry.clone());
    }
}

fn main_again() -> bool {
    println!("Would you like to do anything else? (Y/N)");

    let mut continue_yn = String::new();
    io::stdin()
        .read_line(&mut continue_yn)
        .expect("Failed to read line");

    match continue_yn.trim() {
        "y" | "Y" => {
            let bool = true;
            bool
        },
        "n" | "N" => {
            println!("Exiting...");
            let bool = false;
            bool
        },
        _ => {
            println!("Failure: Bad Input, exiting...");
            let bool = false;
            bool
        },
    }
}

fn user_interchange(mut company_department_map: HashMap<String, Vec<String>>, dept_index: Vec<&str>, user_entry: &Vec<String>) -> HashMap<String, Vec<String>> {
    let parsed_user_entry = split_string_on_space_return_multi_part_vec(user_entry[0].clone());
    // User interface
    match parsed_user_entry[0].trim() {
        "add" => {
            match parsed_user_entry[3].trim() {
                "sales" => {
                    let employee = parsed_user_entry[1].trim().to_owned() + " Bigsby";
                    company_department_map = add_emp_to_dept(company_department_map.clone(), &dept_index[0], employee.to_string());
                    println!("Added {:?} to Sales Department", capitalize(parsed_user_entry[1].trim()) + " Bigsby");
                }, 
                "reporting" => {
                    let employee = parsed_user_entry[1].trim().to_owned() + " Bigsby";
                    company_department_map = add_emp_to_dept(company_department_map.clone(), &dept_index[1], employee.to_string());
                    println!("Added {:?} to Reporting Department", capitalize(parsed_user_entry[1].trim()) + " Bigsby");
                }, 
                "it" => {
                    let employee = parsed_user_entry[1].trim().to_owned() + " Bigsby";
                    company_department_map = add_emp_to_dept(company_department_map.clone(), &dept_index[2], employee.to_string());
                    println!("Added {:?} to IT Department", capitalize(parsed_user_entry[1].trim()) + " Bigsby");
                }, 
                "maintenance" => {
                    let employee = parsed_user_entry[1].trim().to_owned() + " Bigsby";
                    company_department_map = add_emp_to_dept(company_department_map.clone(), &dept_index[3], employee.to_string());
                    println!("Added {:?} to Maintenance Department", capitalize(parsed_user_entry[1].trim()) + " Bigsby");
                }, 
                "shipping" => {
                    let employee = parsed_user_entry[1].trim().to_owned() + " Bigsby";
                    company_department_map = add_emp_to_dept(company_department_map.clone(), &dept_index[4], employee.to_string());
                    println!("Added {:?} to Shipping Department", capitalize(parsed_user_entry[1].trim()) + " Bigsby");
                },
                _ => {
                    println!("Addition Failed");
                }     
            }
        }, 
        "review" => { // Print data for Review   
            match parsed_user_entry[1].trim() {
                "all" => {
                    println!("Review all:");
                    all_department_print(company_department_map.clone(), dept_index.clone());    
                },
                "sales" => {
                    println!("Review Sales Dept:");
                    department_print(company_department_map.clone(), dept_index.clone(), 0);
                },
                "reporting" => {
                    println!("Review Reporting Dept:");
                    department_print(company_department_map.clone(), dept_index.clone(), 1);
                },
                "it" => {
                    println!("Review IT Dept:");
                    department_print(company_department_map.clone(), dept_index.clone(), 2);
                },
                "maintenance" => {
                    println!("Review maintenance Dept:");
                    department_print(company_department_map.clone(), dept_index.clone(), 3);
                },
                "shipping" => {
                    println!("Review shipping Dept:");
                    department_print(company_department_map.clone(), dept_index.clone(), 4);
                },
                _ => {
                    println!("Review Failed:");
                }
            }
        },
        _ => {
            println!("User interchange failed");
        }  
    }

    company_department_map
}

fn split_string_on_space_return_multi_part_vec(s: String) -> Vec<String> {
    let parts: Vec<String> = s
        .split(" ")
        .map(|part| part.to_string().to_lowercase())
        .collect();
    parts
}


fn add_emp_batch_to_dept(mut company_department_map: HashMap<String, Vec<String>>, dept_index: Vec<&str>, dept: usize, employees: Vec<&str>) -> HashMap<String, Vec<String>> {
    for employee in employees {
        company_department_map = add_emp_to_dept(company_department_map.clone(), &dept_index[dept], employee.to_string());
    }

    company_department_map
}

fn capitalize(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        None => String::new(),
        Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn add_emp_to_dept(mut company_department_map: HashMap<String, Vec<String>>, dept: &str, employee: String) -> HashMap<String, Vec<String>> {
    let employee_capitalized = capitalize(&employee);

    company_department_map.entry(dept.to_string()
        .to_string())
        .or_insert_with(Vec::new)
        .push(format!("{employee_capitalized}"));

    company_department_map
}

fn sort_employees(company_department_map: HashMap<String, Vec<String>>, target_department: Vec<&str>, index: u64) -> Vec<String> {
    let mut employees_sorted: Vec<String> = Vec::new();
    
    for (department, employees) in company_department_map {
        if department == target_department[index as usize] {
            for employee in employees {
                employees_sorted.push(employee.to_string());
            }
        }
    }
    
    employees_sorted.sort_by(|a, b| a.cmp(&b));
    employees_sorted
}

fn department_print(company_department_map: HashMap<String, Vec<String>>, target_department: Vec<&str>, index: u64) {
    println!("dept: {:?}", target_department[index as usize]);    
    let sorted_employees = sort_employees(company_department_map, target_department, index);
    println!("sorted_employees: {:?}", sorted_employees);
}

fn all_department_print(company_department_map: HashMap<String, Vec<String>>, target_department: Vec<&str>) {
    for i in 0..target_department.len() {  
        let index = i;
        department_print(company_department_map.clone(), target_department.clone(), index.try_into().unwrap());
    }
}

fn manual_entry(v: &mut Vec<String>) {
    println!("Please enter your selection: 'X' to Exit");
    
    let mut user_entry = String::new();
    let mut clean_float = Vec::new();
    
    io::stdin()
        .read_line(&mut user_entry)
        .expect("Failed to read line");
    
    user_entry = user_entry.trim().to_string();

    for char in user_entry.chars() {
        if char.is_ascii(){
            clean_float.push(char as u8)
        }
    }
    
    if !clean_float.is_empty() {
        v.push(String::from_utf8(clean_float).expect("Our bytes should be valid utf8"));
    }

    for char in user_entry.chars() {
        if char == 'x' || char == 'X'{
            println!("Exiting...");
        }
    }
}