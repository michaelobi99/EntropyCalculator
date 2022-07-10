use std::io::{Read};
use std::fs::File;
use std::collections::{HashMap};

fn read_file(input_file: &mut File) -> (HashMap<char, u32>, u32){
    let mut buffer =  [0u8; 4096];
    let mut char_buffer: Vec<char> = Vec::new();
    char_buffer.resize(256, '0');
    let mut frequency_buffer: Vec<u32> = Vec::new();
    frequency_buffer.resize(256, 0);
    for i in 0..=255u8{
        char_buffer[i as usize] = i as char;
    }
    loop{
        let num_bytes = input_file.read(&mut buffer).unwrap();
        for elem in 0..num_bytes{
            frequency_buffer[buffer[elem] as char as usize] += 1;
        }
        if num_bytes < buffer.len(){
            break;
        }
    }
    let cum_freq: u32 = frequency_buffer.iter().sum();
    println!("\n{:?}", cum_freq);
    (char_buffer.into_iter().zip(frequency_buffer).collect::<HashMap<char, u32>>(), cum_freq)
}

fn calculate_entropy(symbol_map: &HashMap<char, u32>, cum_freq: u32) -> f64{
    let mut probability_list : [f64; 256] = [0.0f64; 256];
    for i in 0..=255u8{
        probability_list[i as usize] = symbol_map[&(i as char)] as f64 / cum_freq as f64;
    }
    let mut entropy: f64 = 0.0;
    for elem in probability_list.iter(){
        if *elem > 0.0{
            entropy += elem * -elem.log2();
        }
    }
    entropy
}

fn main(){
    //println!("Enter the name of the file, with its full path: ");
    //let mut filename : String = String::new();
    //stdin().read_line(&mut filename).expect("Error reading string");
    let mut input_file : File = File::open("test.txt").expect("Failed to open file");
    let (symbol_map, cum_freq) = read_file(&mut input_file);
    let entropy_of_source : f64 = calculate_entropy(&symbol_map, cum_freq);
    println!("Entropy of file = {} bits/symbol", entropy_of_source);
    println!("Hence the minimum number of bits needed to encode is {}", entropy_of_source * cum_freq as f64);
}