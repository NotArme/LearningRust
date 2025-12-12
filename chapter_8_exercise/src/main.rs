/*
Exercises proposed on chapter 8.3 of "The Rust Programming Language" 2024 edition 


    Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) 
and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added, 
so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). 
Keep in mind the details about UTF-8 encoding!
    Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; 
for example, “Add Sally to Engineering” or “Add Amir to Sales.” 
Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

The standard library API documentation describes methods that vectors, strings, and hash maps have that will be helpful for these exercises!


*/

mod median_mode;
mod pig_latin;
mod employee_map;


fn main() {
    println!("Hello, world!\n");
    
    let mut number_list: Vec<i16> = vec![4, -29, -493, 421, 9, 9, 39, -418, -32, 39, 39, 86, 422, -318, 9, -441, -441, 172, -156, -498, -120];

    println!("Vector is {:?}\n", number_list);

    let median = median_mode::median(&mut number_list);
    println!("Median is {}\n", median);

    let (repetitions, mode) = median_mode::mode(&mut number_list);
    println!("Mode is {:?}\nIt appears a total of {} times\n", mode, repetitions);

}
