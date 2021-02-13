mod utility;

fn main() {
    println!("Day 1!");

    let expense_report = utility::load_file_data("./resources/day1.txt");
    
    //go through the set, find 2020 - expense and see if it exists in the set
    //if it does, we found the matching pair;

    for expense in &expense_report {
        //expense is (key, value) where (expense value, count of expense value)
        let required_value = 2020 - expense.0;

        if expense_report.contains_key(&required_value) 
            && *(expense_report.get(&required_value).unwrap()) > 0 as i64 {

            println!("The expected value is: {}", expense.0 * required_value);

            break;
        }
    }
}
