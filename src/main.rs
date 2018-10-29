extern crate byteorder;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use byteorder::{ByteOrder, LittleEndian};

struct Dimensions {
    width : u32,
    height : u32,
}

impl Dimensions {
    fn new(width : u32, height : u32) -> Dimensions {
        Dimensions {
            width,
            height,
        }
    }
}

//use AsRef so can accept different types of references?
//https://doc.rust-lang.org/std/convert/trait.AsRef.html
fn read_file_as_bytes<P: AsRef<Path>>(path : P) ->  std::io::Result<Vec<u8>>
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

fn find_and_replace_once(bytes : &mut Vec<u8>, find_str : &[u8], replacement : &[u8]) -> Result<(), &'static str>
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

//patch umineko .exe to load 0.u instead of 0.utf
fn umineko_patch_load_0u(file_as_bytes : &mut Vec<u8>)
{
    find_and_replace_once(file_as_bytes, b"0.utf", b"0.u\x00\x00")  .expect("failed to replace 0.utf str");
    find_and_replace_once(file_as_bytes, b"%d.%s", b"%dx%s")        .expect("failed to replace %d.%s str");
    find_and_replace_once(file_as_bytes, b"%02d.%s", b"%d.%.1s")    .expect("failed to replace %02d.%s str");
    find_and_replace_once(file_as_bytes, b"saves/", b"mysav/")      .expect("failed to replace saves/ str");
}

fn get_height_bytes(height : u32) -> [u8; 5]
{
    get_dimensions_search_string(186, height)
}

fn get_width_bytes(height : u32) -> [u8; 5]
{
    get_dimensions_search_string(185, height)
}

fn get_dimensions_search_string(magic : u8, value : u32) -> [u8; 5]
{
    let mut bytes : [u8; 5] = [magic, 0, 0, 0, 0];
    LittleEndian::write_u32(&mut bytes[1..5], value);
    bytes
}

fn umineko_change_width(file_as_bytes : &mut Vec<u8>, search_dimension : &Dimensions, replacement_dimensions : &Dimensions)
{
    let search_height = get_height_bytes(search_dimension.height);
    let replacement_height = get_height_bytes(replacement_dimensions.height);

    let search_width = get_width_bytes(search_dimension.width);
    let replacement_width = get_width_bytes(search_dimension.width);

    find_and_replace_once(file_as_bytes, &search_height, &replacement_height).expect("failed to replace HEIGHT");
    find_and_replace_once(file_as_bytes, &search_width, &replacement_width)  .expect("failed to replace WIDTH");
}

fn main()
{
    println!("Hello, world!");

    let mut file_as_bytes = read_file_as_bytes("test.txt").expect("could open input file");

    //let search_string = b"0.utf";
    //let replace_string = b"asdff";

    /*let result = find_and_replace_once(&mut file_as_bytes, search_string, replace_string);

    match(result)
    {
        Err(str) => {
            println!("Error - couldn't perform replacement!: {}", str);

        },
        Ok(value) => {
            let mut output_file = File::create("output.txt").expect("couldn't open output file");
            output_file.write_all(&file_as_bytes).expect("couldn't write to output file");

        }
    }*/

    //umineko_patch_load_0u(&mut file_as_bytes);

    let source_dim = Dimensions::new(1280, 960);
    let target_dim = Dimensions::new(1920, 1080);

    umineko_change_width(&mut file_as_bytes, &source_dim, &target_dim);

    let mut output_file = File::create("output.txt").expect("couldn't open output file");
    output_file.write_all(&file_as_bytes).expect("couldn't write to output file");
}
