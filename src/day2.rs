mod utility;

#[derive(Copy, Clone)]
struct PasswordRule {
    min: i64,
    max: i64,
    character: char
}

#[derive(Clone)]
struct PasswordEntry {
    rule: PasswordRule,
    password: String,
    is_valid: bool
}

/*
    is_password_valid calculates if password satisifies the min and max count of character 
*/
fn is_password_valid(password: &str, min: &i64, max: &i64, character: &char) -> bool {

    let mut count: i64 = 0;
    for password_char in password.chars() {
        if password_char == *character {
            count += 1;
        }
    }

    *min <= count && count <= *max
}

/*
Takes a line in the file and turns it into a PasswordEntry
Each line is in the format 
1-3 c: password
*/
fn parse_row(line: &str) -> PasswordEntry {
    
    //split into 1-3 c and password
    let line_split: Vec<&str> = line.split(": ").collect();

    //take 1-3 c and split out the min max and character
    let rule_string = line_split[0];
    let rule_string_split: Vec<&str> = rule_string.split(" ").collect();
    let minmax_string = rule_string_split[0];
    let character = rule_string_split[1];
    let minmax_string_split: Vec<&str> = minmax_string.split("-").collect();
    
    //take the password and min/max/char values and determine if the passwords are valid
    let password = String::from(line_split[1]);
    let min = minmax_string_split[0].parse::<i64>().unwrap();
    let max = minmax_string_split[1].parse::<i64>().unwrap();
    let character = character.chars().nth(0).unwrap();

    let is_valid = is_password_valid(&password, &min, &max, &character);

    //create an object and return it
    PasswordEntry {
        rule: PasswordRule { 
            min: min, 
            max: max,
            character: character,
        }, 
        password: password, 
        is_valid: is_valid,
    }
}

//transform pushes the PasswordEntry data into a Vec to be returned
fn transform_data(values: &mut Vec<PasswordEntry>, data: &PasswordEntry) {

    //Don't know rust well enough to figure out how to handle the lifetimes
    //to just push a reference around, and String apparently can't implement Copy, so, make our own copy.
    values.push(PasswordEntry {
        rule: PasswordRule {
            min: data.rule.min,
            max: data.rule.max,
            character: data.rule.character
        },
        password: String::from(&data.password),
        is_valid: data.is_valid
    });
}

fn main() {
    println!("Day 2: Part 1!");

    let passwords = utility::load::<Vec<PasswordEntry>, PasswordEntry>("./resources/day2.txt", &parse_row, &transform_data);

    let mut valid_count = 0;
    for password in passwords {
        if password.is_valid == true {
            valid_count += 1;
        }
    }

    println!("Number of invalid passwords: {}", valid_count);
    
 }
