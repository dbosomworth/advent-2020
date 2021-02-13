use std::collections::HashMap;
mod utility;



//Our callback to the load_file_data 
fn transform_data(values: &mut rHashMap<i64, i64>, data: &i64) {
    let count = values.entry(*data).or_insert(0);
    *count += 1;
}

fn main() {
    println!("Day 1!");

    let expense_report = utility::load_and_transform_file_data("./resources/day1.txt", &transform_data);
    
    //go through the map, find 2020 - expense and see if it exists in the map
    //if it does, we found the matching pair;

    for expense in &expense_report {
        //expense is (key, value) where (expense value, count of expense value)

        let required_value = 2020 - expense.0;

        //if our required value exists in the map, then calculate the solution
        if expense_report.contains_key(&required_value) {
            println!("The expected value is: {}", expense.0 * required_value);
            break;
        }
    }
}
