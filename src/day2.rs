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
    is_valid_part_one: bool,
    is_valid_part_two: bool,
}

/*
    is_part_one_password_valid calculates if password satisifies the min and max count of character 
*/
fn is_part_one_password_valid(password: &str, min: &i64, max: &i64, character: &char) -> bool {

    let mut count: i64 = 0;
    for password_char in password.chars() {
        if password_char == *character {
            count += 1;
        }
    }

    *min <= count && count <= *max
}


/*
    is_part_two_password_valid calculates if password satisifies the position 1 or position 2 character requirements
*/
fn is_part_two_password_valid(password: &str, first_position: &usize, second_position: &usize, character: &char) -> bool {

    let first_position_valid = password.chars().nth(*first_position - 1).unwrap() == *character;
    let second_position_valid = password.chars().nth(*second_position - 1).unwrap() == *character;  

    first_position_valid ^ second_position_valid
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
    let first_position = minmax_string_split[0].parse::<usize>().unwrap();
    let second_position = minmax_string_split[1].parse::<usize>().unwrap();
    let character = character.chars().nth(0).unwrap();

    let is_valid_part_one = is_part_one_password_valid(&password, &min, &max, &character);
    let is_valid_part_two = is_part_two_password_valid(&password, &first_position, &second_position, &character);

    //create an object and return it
    PasswordEntry {
        rule: PasswordRule { 
            min: min, 
            max: max,
            character: character,
        }, 
        password: password, 
        is_valid_part_one: is_valid_part_one,
        is_valid_part_two: is_valid_part_two,
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
        is_valid_part_one: data.is_valid_part_one,
        is_valid_part_two: data.is_valid_part_two,
    });
}

fn main() {
    println!("Day 2!");

    let passwords = utility::load::<Vec<PasswordEntry>, PasswordEntry>("./resources/day2.txt", &parse_row, &transform_data);

    let mut part_one_valid_count = 0;
    let mut part_two_valid_count = 0;
    
    for password in passwords {

        if password.is_valid_part_one == true {
            part_one_valid_count += 1;
        }

        if password.is_valid_part_two == true {
            part_two_valid_count += 1;
        }
    }

    println!("(Part One) Number of invalid passwords: {}", part_one_valid_count);
    println!("(Part Two) Number of invalid passwords: {}", part_two_valid_count);
 }
