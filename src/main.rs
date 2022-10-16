// Import Crates

use sha2::Sha256;
use sha2::Digest;
use std::fs::File;
use std::io::prelude::*;



// hash_single_input function is in assignment README to be used

fn hash_single_input(a: &str) -> String {
    let mut hasher = Sha256::new();
    let input = a;
    hasher.update(input);
    let hash = hasher.finalize();
    let hex = hex::encode(&hash);

    return hex.to_string();
}

//to read a file 

fn read_file() {
    for i in 0..5 {
        let input_files = ["input0.txt","input1.txt","input2.txt","input3.txt","input4.txt"];

        let mut file = File::open(input_files[i]).expect("Can't open file!");

        let mut contents= Vec::new();

        file.read_to_end(&mut contents)
        .expect("Oops! Can not read the file!");

        //println!("File Contents:\n\n{:?}", contents);
    } 
}

fn merkle_root(filename: String) -> String {
    read_file();
    // Read Input Data from txt file
    todo!()

    // Create vector of strings for leaves
    

    // Hash inputs and append to vector
    

    // Then Write an algorithm that calculates the ROOT


    // Return the root hash as a String
}

// You can use templates below or just remove
// Helper function to create a next leaves level may help you :)
fn create_next_level(current_level: Vec::<String>) -> Vec::<String> {
    todo!();
}


// Helper function may help you to hash an input or You can write macro rules
fn hash_input(a: &str) -> String {
    todo!();
}

fn main() { 
    read_file()

   

}



// Pass all tests!
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn input_0() {
        let result = merkle_root("input0.txt".to_string());
        assert_eq!(result, "ff41418be11ed77612aeb83ee0bcf97a5853a4c291e23bd4d4cc6435fcfabdf9");
    }

    #[test]
    fn input_1() {
        let result = merkle_root("input1.txt".to_string());
        assert_eq!(result, "98a77b2c3ff5f6c2aca697f60b2aa2a1a2733be36dbd35bae23d517c2ad5985e");
    }

    #[test]
    fn input_2() {
        let result = merkle_root("input2.txt".to_string());
        assert_eq!(result, "3c0fb0638de91551eae4e9d984d72034aa0693be37b51737e6b81bc489866e5e");
    }

    #[test]
    fn input_3() {
        let result = merkle_root("input3.txt".to_string());
        assert_eq!(result, "f03b1c9163babeb728ac011fe0c2c9c69700a2f8ddde211ec07d621cdb322cfe");
    }

    #[test]
    fn input_4() {
        let result = merkle_root("input4.txt".to_string());
        assert_eq!(result, "f83e74742fda659dfc07615881af796abafc434f591aeb23b9f4366abe03c597");
    }
}
