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
        "Billy Bigsby", 
        "Marty Bigsby", 
        "Rodger Bigsby", 
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
    println!("   Review employees in 'Department'");
    println!("   Review all employees");

    let mut user_entry = Vec::new();
    manual_entry(&mut user_entry);

    let parsed_user_entry = split_string_on_space_return_multi_part_vec(user_entry[0].clone());
    println!("parsed_user_entry: {:?}", parsed_user_entry);    
    
    // User interface
    match parsed_user_entry[0].trim() {
        "add" => {
            println!("Add");
        },
        "review" => {
            if parsed_user_entry[1].trim() == "all" {
                println!("Review all");
            } else {
                println!("Review");
            }
        },
        &_ => {
            println!("Failed");
        }
    }

    // // Print data for Review    
    // all_department_print(company_department_map.clone(), dept_index);
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

fn add_emp_to_dept(mut company_department_map: HashMap<String, Vec<String>>, dept: &str, employee: String) -> HashMap<String, Vec<String>> {
    company_department_map.entry(dept.to_string()
        .to_string())
        .or_insert_with(Vec::new)
        .push(format!("{employee}"));

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