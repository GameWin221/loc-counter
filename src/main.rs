use std::collections::HashMap;
use std::io::{self, BufRead};
use std::env;
use std::fs;

fn get_file_loc(path: &std::path::Path, extensions: &HashMap<String, bool>, file_count: &mut u32) -> u32 {
    // Open a file.
    let file = fs::File::open(path).unwrap();

    // If the file has no extension, it is not a valid file.
    if path.extension().is_none() {
        return 0;
    }

    // Get the extension of the file
    let file_extension = path.extension().unwrap().to_str().unwrap().to_string();

    //Should filter by extension?
    if extensions.len()>0 {
        // Is the extension not the list of extensions?
        if !extensions.contains_key(&file_extension){
            return 0;
        }
    }

    // Get the actual number of lines of the file.
    let file_loc = io::BufReader::new(file).lines().count() as u32;

    // Print the file name and the number of its lines.
    println!("{:?} has {} loc", path, file_loc);

    *file_count += 1;

    return file_loc;
}
fn get_dir_loc(path: &std::path::Path, extensions: &HashMap<String, bool>, file_count: &mut u32) -> u32 {
    let mut loc = 0;
    
    // For each file/dir in the directory.
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();

        // If the entry is a directory, recursively add loc. If it's a file just add its loc.
        if file_type.is_dir() {
            loc += get_dir_loc(&entry.path(), extensions, file_count);
        } else if file_type.is_file() {
            loc += get_file_loc(&entry.path(), extensions, file_count);
        }
    }
    
    loc
}

fn main() {
    // Get arguments.
    let args: Vec<String> = env::args().collect();

    let mut loc = 0;
    let mut file_count = 0;

    let mut extensions = HashMap::new();

    // Check if the user entered any arguments. If not, print the correct usage.
    if args.len() > 1 {
        let src_dir = &args[1];

        // Check if the user entered any extensions. If so, add them to the hashmap.
        if args.len() > 2{
            for i in 2..args.len() {
                let trimmed_ext = args[i].replace(".", "");

                extensions.insert(trimmed_ext, true);
            }
        }

        let path = std::path::Path::new(src_dir);

        assert!(path.exists(), "That path does not exist.");

        // Get the total number of lines of code in the directory.
        if path.is_dir() {
            loc = get_dir_loc(path, &extensions, &mut file_count);
        } else if path.is_file() {
            loc = get_file_loc(path, &extensions, &mut file_count);
        }
        
    } else {
        println!("Please provide a directory! - Usage: ./loc_counter <directory> [extensions]");
    }

    println!("Your source code has a total of {} lines of code in {} files!", loc, file_count);
}
