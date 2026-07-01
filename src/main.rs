use ascii_converter::{AllowableOptions, input_to_enum, set_pipeline, set_pipeline_char};
use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    // Get the user input to determine which code to translate FROM
    println!(
        "\nFrom?\n(t)ext, (d)ecimal, (o)ctal, (h)exadecimal,\nHexadecimal with leading 0(x), (b)inary"
    );
    // Convert stdin to an enum
    let from_enum = input_to_enum(String::new());
    // If input is not allowable, exit with error
    if from_enum == AllowableOptions::Invalid {
        println!("[ INVALID ] From selection was invalid");
        return ExitCode::from(1);
    }

    // Get the user input to determine which code to translate TO
    println!(
        "\nTo?\n(t)ext, (d)ecimal, (o)ctal, (h)exadecimal,\nHexadecimal with leading (0)x, (b)inary"
    );
    // Convert stdin to an enum
    let to_enum = input_to_enum(String::new());
    // If input is not allowable, exit with error
    if to_enum == AllowableOptions::Invalid {
        println!("[ INVALID ] To selection was invalid");
        return ExitCode::from(1);
    }

    // Create new strings  the message to translate
    let mut message = String::new();
    // Get the user input to determine the MESSAGE
    println!("\nMessage:");
    // Read the message input
    let _message_input = io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");
    // Trim and parse the input to remove the newline character left by the user pressing ENTER
    let message: String = message.trim().parse().expect("Could not convert line");

    // Print a blank line for spacing
    println!();

    if from_enum == AllowableOptions::Text {
        let message_vector: Vec<char> = message.chars().collect();
        // Set up the translation pipeline and translate the message accordingly
        set_pipeline_char(to_enum, message_vector);
    } else {
        // Split the MESSAGE on White Space and collect into a vector
        let message_vector: Vec<_> = message.split_whitespace().collect();
        // Set up the translation pipeline and translate the message accordingly
        set_pipeline(from_enum, to_enum, message_vector);
    }

    // Exit with success
    ExitCode::SUCCESS
}
