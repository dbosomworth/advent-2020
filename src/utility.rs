use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

// Loads a file into a hasmap where the data is the key, and the count of the data is the value
// Normally we would leave the transformation to what's recieving the data, however, i wanted to learn 
// about rust callbacks
pub fn load_and_transform_file_data(filename: &str, transform_callback: &dyn Fn(&mut HashMap<i64, i64>, &i64)) -> HashMap<i64, i64> {
    
    //map to load data into
    let mut values:HashMap<i64, i64> = HashMap::new();

    //open the file, panic if it's not there
    let file = File::open(filename)
        .expect(&format!("file not found {}", filename));
    
    //begin reading
    let reader = BufReader::new(file);
    
    //Load the data into a Vec
    let line_data = reader.lines()                        //1. get the lines
            .map(|x| x.unwrap().parse::<i64>().unwrap() ) //2. iterate through converting to i64
            .collect::<Vec<i64>>();                       //3. Collect into a vec

    //transform our data into our map
    for data in line_data {
        transform_callback(&mut values, &data);
    }

    return values;
}