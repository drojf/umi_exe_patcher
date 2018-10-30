extern crate byteorder;
mod util;
mod umineko_change_resolution;

use umineko_change_resolution::{ScreenResolution, DimensionsWindows, GetDimensionsSearchString};
use std::fs::File;
use std::io::prelude::*;


use util::*;

//patch umineko .exe to load 0.u instead of 0.utf
fn umineko_patch_load_0u(file_as_bytes : &mut Vec<u8>)
{
    find_and_replace_once(file_as_bytes, b"0.utf", b"0.u\x00\x00")  .expect("failed to replace 0.utf str");
    find_and_replace_once(file_as_bytes, b"%d.%s", b"%dx%s")        .expect("failed to replace %d.%s str");
    find_and_replace_once(file_as_bytes, b"%02d.%s", b"%d.%.1s")    .expect("failed to replace %02d.%s str");
    find_and_replace_once(file_as_bytes, b"saves/", b"mysav/")      .expect("failed to replace saves/ str");
}

fn umineko_change_width<T: GetDimensionsSearchString>(file_as_bytes : &mut Vec<u8>, search_dimension : &ScreenResolution, replacement_dimensions : &ScreenResolution)
{
    let search_height = T::get_height_bytes(search_dimension.height);
    let replacement_height = T::get_height_bytes(replacement_dimensions.height);

    let search_width = T::get_width_bytes(search_dimension.width);
    let replacement_width = T::get_width_bytes(search_dimension.width);

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

    let source_dim = ScreenResolution::new(1280, 960);
    let target_dim = ScreenResolution::new(1920, 1080);

    umineko_change_width::<DimensionsWindows>(&mut file_as_bytes, &source_dim, &target_dim);

    let mut output_file = File::create("output.txt").expect("couldn't open output file");
    output_file.write_all(&file_as_bytes).expect("couldn't write to output file");
}
