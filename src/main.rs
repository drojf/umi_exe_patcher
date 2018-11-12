extern crate byteorder;
mod util;
mod umineko_change_resolution;
extern crate clap;

use umineko_change_resolution::{ScreenResolution, DimensionsWindowsLinux, DimensionsMac, GetDimensionsSearchString};
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::path::Path;
use std::ffi::OsStr;
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
    let replacement_width = T::get_width_bytes(replacement_dimensions.width);

    find_and_replace_once(file_as_bytes, &search_height, &replacement_height).expect("failed to replace HEIGHT");
    find_and_replace_once(file_as_bytes, &search_width, &replacement_width)  .expect("failed to replace WIDTH");
}

fn umineko_video(file_as_bytes : &mut Vec<u8>) {


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
                           //Patch Resolution subcommand:
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
                          .subcommand(SubCommand::with_name("video")
                              .about("Disables 2x video scaling (NOT REVERSIBLE)")
                          )
                          .get_matches();



    let input_file = matches.value_of("input").expect("Missing input file argument");
    let output_file = matches.value_of("output").expect("Missing output file argument");

    //determine if the input file is a (Windows/linux) or Mac filename
    let is_mac =  match Path::new(input_file).file_name().and_then(OsStr::to_str)
    {
        Some("umineko4") => true, //linux exe
        Some("umineko8") => true,
        _ => false,
    };

    println!("Is Mac Exe:{} (Note: win and linux patched in same way. Mac patched differently)", is_mac);

    let mut file_as_bytes = read_file_as_bytes(input_file).expect("could open input file");

    println!("File size is {} bytes", file_as_bytes.len());

    //Execute subcommands:
    if let Some(submatches) = matches.subcommand_matches("resolution")
    {
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
        if is_mac {
            umineko_change_width::<DimensionsMac>(&mut file_as_bytes, &search, &replace);
        } else {
            umineko_change_width::<DimensionsWindowsLinux>(&mut file_as_bytes, &search, &replace);
        }
    }

    if let Some(submatches) = matches.subcommand_matches("video")
    {

    }

    let mut output_file = File::create(output_file).expect("couldn't open output file");
    output_file.write_all(&file_as_bytes).expect("couldn't write to output file");
}
