extern crate byteorder;
mod util;
mod umineko_change_resolution;
extern crate clap;

use umineko_change_resolution::{ScreenResolution, DimensionsWindows, GetDimensionsSearchString};
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use util::*;

use clap::{Arg, App, SubCommand};

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
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("~~~~~ NOTE: run [umi_exe_patcher -h] to see all arguments ~~~~~");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");

    let matches = App::new("My Super Program")
                          .version("1.0")
                          .author("some guy called drojf")
                          .about("Does awesome things")
                                  //specify input and output files
                          .arg(Arg::with_name("input")
                               .long("input")
                               .value_name("FILE")
                               .help("Sets the input file")
                               .takes_value(true)
                               .required(true))
                          .arg(Arg::with_name("output")
                               .long("output")
                               .value_name("FILE")
                               .help("Sets the output file")
                               .takes_value(true)
                               .required(true))

                          .subcommand(SubCommand::with_name("resolution")
                                      .about("Modifies the resolution of the game")
                                      //specify resolution to search for
                                      .arg(Arg::with_name("search_resolution")
                                           //.multiple(true)
                                           .long("search")
                                           .help("resolution to be replaced")
                                           .value_names(&["WIDTH", "HEIGHT"])
                                           .takes_value(true)
                                           .required(true))

                                      .arg(Arg::with_name("replacement_resolution")
                                           //.multiple(true)
                                           .long("replace")
                                           .help("new resolution")
                                           .value_names(&["WIDTH", "HEIGHT"])
                                           .takes_value(true)
                                           .required(true))
                          )
                          .get_matches();



    let input_file = matches.value_of("input").expect("Missing input file argument");
    let output_file = matches.value_of("output").expect("Missing output file argument");

    match matches.subcommand_matches("resolution")
    {
        Some(submatches) => {
            let searches : Vec<_> = submatches.values_of("search_resolution").unwrap().collect();

            let search = ScreenResolution::new(
                u32::from_str(searches[0]).expect("Search width could not be parsed"),
                u32::from_str(searches[1]).expect("Search height could not be parsed")
            );

            let replacements : Vec<_> = submatches.values_of("replacement_resolution").unwrap().collect();

            let replace = ScreenResolution::new(
                u32::from_str(replacements[0]).expect("Replace width could not be parsed"),
                u32::from_str(replacements[1]).expect("Replace height could not be parsed")
            );

            println!("Performing Resolution Patch: [{}x{}] -> [{}x{}]...", search.width, search.height, replace.width, replace.height);

            //let search_width = submatches.value_of("search_width").expect("Missing search width argument");
            //let search_height = submatches.value_of("search_height").expect("Missing search height argument");

            //let replacement_width = submatches.value_of("replacement_width").expect("Missing replacement_width argument");
            //let replacement_height = submatches.value_of("replacement_height").expect("Missing input replacement_height argument");
        }
        None => {}
    }




    let mut file_as_bytes = read_file_as_bytes(input_file).expect("could open input file");

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
