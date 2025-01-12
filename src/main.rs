/*
 * TSTAMPER UTILITY
 * ----------------
 * Changelog:
 * Jacob S. - Added support for ISO8601 outputs, switched to chrono package - (2025-01-12T16:57:42.389463500-06:00)
 */

use std::env;
use chrono::prelude::*;

const HELP_TEXT: &str = 
"TSTAMPER SIMPLE TIMESTAMP UTILITY
By Jacob S. (jsimeone0105@gmail.com)
Licensed for use under the MIT license (Should have been distributed with this source)

DESCRIPTION:
\tDead simple terminal application for quickly obtaining current timestamps in
\tdifferent formats. Only use one option as listed below, per run. For example:
\t\"tstamper --ms\" --> VALID
\t\"tstamper --ms --iso8601\" --> INVALID

By default, the application will return the UNIX timestamp, in seconds since epoch.

OPTIONS:
\t-h
\tDisplay help for this application (this output)

\t--ms
\tDisplay the unix timestamp in milliseconds rather than seconds.

\t--iso8601
\tDisplay the current time with local timezone offset to UTC in the ISO8601/RFC3339 format

\t--iso8601-utc
\tDisplay the current UTC time in the ISO8601/RFC3339 format (also includes offset, which is always zero)";

const HELP_SWITCH: &str = "-h";
const MS_SWITCH: &str = "--ms";
const ISO_SWITCH: &str = "--iso8601";
const ISO_SWITCH_UTC: &str = "--iso8601-utc";

fn get_now_utc() -> DateTime<Utc> {
    return DateTime::from(Utc::now());
}

fn get_now_local() -> DateTime<Local> {
    return DateTime::from(Local::now());
}

fn get_unix_timestamp_s() -> i64 {
    return get_now_utc().timestamp();
}

fn get_unix_timestamp_ms() -> i64 {
    return get_now_utc().timestamp_millis();
}

fn get_timestamp_iso8601() -> String {
    return get_now_local().to_rfc3339(); // Same as ISO8601 for our purposes here
}

fn get_timestamp_iso8601_utc() -> String {
    return get_now_utc().to_rfc3339(); // Same as ISO8601 for our purposes here
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
        match args[1].as_str() {
            HELP_SWITCH => { print_help(); },
            MS_SWITCH => { println!("{:?}", get_unix_timestamp_ms()); },
            ISO_SWITCH => { println!("{}", get_timestamp_iso8601()) },
            ISO_SWITCH_UTC => { println!("{}", get_timestamp_iso8601_utc()) },
            _ => { println!("Invalid input."); print_help(); }
        }
    } else {
        print_help();
    }
}
