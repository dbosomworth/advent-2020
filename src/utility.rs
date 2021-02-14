use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// Loads a text file where every line is a single value.
// T is the type you wish the data to be returned in.
// K is the type of data on each line.
// transform_callback is called once per line of data allowing you to add or transform the data
// before it's returned.
pub fn load<T: Default, K>(filename: &str, parse_callback: &dyn Fn(&str) -> K, transform_callback: &dyn Fn(&mut T, &K)) -> T {
    
    //map to load data into
    let mut values:T = T::default();

    //open the file, panic if it's not there
    let file = File::open(filename)
        .expect(&format!("file not found {}", filename));

    //begin reading
    let reader = BufReader::new(file);
    
    //Load the data into a Vec
    let line_data: Vec<K> = reader.lines()                        //1. get the lines
            .map(|x| parse_callback(&x.unwrap()))                 //2. iterate through calling the parse_callback
            .collect();                                           //3. Collect into a vec

    //transform our data into our map
    for data in line_data {
        transform_callback(&mut values, &data);
    }

    return values;
}