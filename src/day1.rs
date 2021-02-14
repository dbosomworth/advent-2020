use std::collections::HashMap;
mod utility;

//Callback to parse the i64 line
fn parse_row(line: &str) -> i64 {
    line.parse::<i64>().unwrap()
}

//Callback to transform the vector of i64 into a hashmap 
fn transform_data(values: &mut HashMap<i64, i64>, data: &i64) {
    values.entry(*data).or_insert(2020 - *data);
}

fn main() {
    println!("Day 1!");

    let expense_report = utility::load::<HashMap<i64, i64>, i64>("./resources/day1.txt", &parse_row, &transform_data);
    
    //go through the map, find 2020 - expense and see if it exists in the map
    //if it does, we found the matching pair;

    for expense in &expense_report {
        //expense is (key, value) where (expense value, count of expense value)

        //if our required value exists in the map, then calculate the solution
        if expense_report.contains_key(&expense.1) {
            println!("The expected value is: {}", expense.0 * expense.1);
            break;
        }
    }
}
