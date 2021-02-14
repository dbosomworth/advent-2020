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

    println!("Day 1: Part 1!");

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

    println!("\nDay 1: Part 2!");

    

    //Not a fan of the for^2.
    //we already have the first_value, and what the remaining two values need to equal for that first_value
    // x = first_expense
    // 2020 - x = y + z
    for expense in &expense_report {
        
        for remaining in &expense_report {

            //edge case: skip ourself if we are the same
            if expense == remaining  {
                continue;
            }

            let missing_value = expense.1 - remaining.0;

            //if we aren't really the missing value, then skip
            if expense.0 + remaining.0 + &missing_value != 2020 {
                continue;
            }
          
            //if our required value exists in the map, then calculate the solution
            if expense_report.contains_key(&missing_value) {
                println!("The expected value is: {}", expense.0 * remaining.0 * &missing_value);
                return;
            }
        }
    }
}