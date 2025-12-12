/* 
Given a list of integers, use a vector and return the median 
(when sorted, the value in the middle position) 
and mode (the value that occurs most often; a hash map will be helpful here) of the list.
*/

use std::collections::HashMap;

pub fn median(number_list: &mut Vec<i16>) -> f64 {

    number_list.sort();

    let vector_length: usize = number_list.len();

    if (vector_length % 2) == 0 {
        let n1: i16 = number_list[(vector_length/2) - 1];
        let n2: i16 = number_list[vector_length/2];

        return (f64::from(n1) + f64::from(n2)) / 2.;
    } else {
        let n: i16 = number_list[vector_length/2];
        return f64::from(n);
    }
}

pub fn mode(number_list: &mut Vec<i16>) -> (i16, Vec<i16>) {
    let mut int_to_count_map: HashMap<i16, i16> = HashMap::new();

    for n in number_list {
        let count = int_to_count_map.entry(*n).or_insert(0);
        *count +=1;
    }

    let mut highest_repetition: i16 = 0;
    let mut value_list: HashMap<i16, Vec<i16>> = HashMap::new(); //TODO: WOULD PROBABLY BE BETTER AS A STRUCT

    for (key, value) in int_to_count_map.into_iter() {
        if value < highest_repetition {continue;}
        if value == highest_repetition {
            let list = value_list.entry(value).or_insert(vec![key]);
            list.push(key);
        }
        if value > highest_repetition {
            highest_repetition = value;
            value_list.clear();
            value_list.insert(value, vec![key]);
        }
    }

    return (highest_repetition, value_list.remove(&highest_repetition).expect("its not production code and the input is baked in anyways so whatever"))
}