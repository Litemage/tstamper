use std::time::{SystemTime, Duration, UNIX_EPOCH};
use std::env;

const HELP_TEXT: &str = "TSTAMPER SIMPLE TIMESTAMP UTILITY\nBy Jacob S. (jsimeone0105@gmail.com)\nLicensed for use under the MIT license (Should have been distributed with this source)\n\nDESCRIPTION:\n\tDead simple terminal application for quickly obtaining the current unix timestamp\n\tin either seconds or ms. Provide the '--ms' flag for ms.\n\nOPTIONS:\n\t-h\n\tDisplay help for this application (this output)\n\t--ms\n\tDisplay the unix timestamp in milliseconds rather than seconds.";
const HELP_SWITCH: &str = "-h";
const MS_SWITCH: &str = "--ms";

fn get_unix_timestamp() -> Duration {
    return SystemTime::now().duration_since(UNIX_EPOCH).expect("Invalid time points (Time went backwards)");
}

fn get_unix_timestamp_s() -> u64 {
    return get_unix_timestamp().as_secs();
}

fn get_unix_timestamp_ms() -> u128 {
    return get_unix_timestamp().as_millis();
}

fn print_help() {
    println!("{}", HELP_TEXT);
}

fn main() {
    // Super simple argument parsing for now. Help with -h, milliseconds with --ms
    let args: Vec<String> = env::args().collect();
    let len: usize = args.len();

    if len == 1 {
        // Just output the timestamp, in seconds
        println!("{:?}", get_unix_timestamp_s());
    } else if len == 2 {
        // Check if the argument passed matches the two switches
        if args[1].as_str() == HELP_SWITCH {
            print_help();
        } else if args[1].as_str() == MS_SWITCH {
            println!("{:?}", get_unix_timestamp_ms());
        } else {
            println!("Invalid Input.");
            print_help();
        }
    } else {
        print_help();
    }
}
