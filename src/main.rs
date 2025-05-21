use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

mod pi; // Declare the pi module

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check for the correct number of arguments
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        eprintln!("Outputs the Pi index for each byte in the input file.");
        process::exit(1);
    }

    // Get the input file path from the arguments
    let file_path = &args[1];

    // Open the specified file
    let mut file = File::open(file_path)?;

    // Create the PiEncoder instance.
    // This calculation happens at compile time due to `const fn`,
    // but the first time you build, it might take a moment.
    eprintln!("Initializing PiEncoder (this happens at compile time)...");
    let encoder = pi::PiEncoder::new();
    eprintln!("PiEncoder initialized.");

    let mut buffer = [0; 4096]; // Use a buffer for efficient reading

    // Read the file in chunks and process each byte
    loop {
        // Read bytes into the buffer
        let bytes_read = file.read(&mut buffer)?;

        // If 0 bytes were read, we've reached the end of the file
        if bytes_read == 0 {
            break;
        }

        // Process each byte in the buffer slice that was just read
        for byte in &buffer[0..bytes_read] {
            // Get the corresponding Pi index for the byte
            let index = encoder.get(*byte);
            // Print the index, followed by a newline
            println!("{}", index);
        }
    }

    // Indicate success
    Ok(())
}
