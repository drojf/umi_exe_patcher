extern crate byteorder;

use std;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

//use AsRef so can accept different types of references?
//https://doc.rust-lang.org/std/convert/trait.AsRef.html
pub fn read_file_as_bytes<P: AsRef<Path>>(path : P) ->  std::io::Result<Vec<u8>>
{
    let file = File::open(path)?;

    let mut buf_reader = BufReader::new(file);

    let mut buffer = Vec::new();

    buf_reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn find_in_bytes(bytes_to_search : &Vec<u8>, target : &[u8]) -> Vec<usize>
{
    let mut matches = Vec::new();

    if bytes_to_search.len() < target.len()
    {
        println!("Input too small - can't find target");
        return matches;
    }
    //after this point, bytes_to_search is >= target length

    let last_search_index = bytes_to_search.len()-target.len();

    //INCLUSIVE for loop - want to include the last search index!
    'outer: for search_loc in 0..=last_search_index
    {
        for i in 0..target.len()
        {
            if bytes_to_search[search_loc + i] != target[i]
            {
                continue 'outer;
            }
        }

        println!("Got match at {}", search_loc);
        matches.push(search_loc);
    }

    matches
}

fn replace_at_position(bytes : &mut Vec<u8>, replacement : &[u8], index : usize, )
{
    let mut temp = Vec::new();
    for i in 0..replacement.len()
    {
        temp.push(bytes[index + i]);
        bytes[index + i] = replacement[i];
    }

    println!("Replaced {}: {:?} -> {:?}", index, temp, replacement);
}

pub fn find_and_replace_once(bytes : &mut Vec<u8>, find_str : &[u8], replacement : &[u8]) -> Result<(), &'static str>
{
    if find_str.len() != replacement.len()
    {
        return Err("search string length does not equal replace string length!")
    }

    let matches = find_in_bytes(bytes, find_str);

    return match matches.len() {
        0 =>  Err("couldn't find search string!"),
        1 => {
            replace_at_position(bytes, replacement, matches[0]);
            Ok(())
        },
        _ => Err("Multiple matches found!"),
    }
}