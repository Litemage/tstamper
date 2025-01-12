# TSTAMPER

`TSTAMPER` (Pronounced "tee-stamper") is a super simple command line utility written in the [rust](https://www.rust-lang.org) programming language for quickly retrieving the current timestamp in different formats. I wrote this program to avoid continually opening a browser and typing in google just to snag the current timestamp.

# Install

The recommended install proceadure is to clone this repository at the master branch, and build the utility yourself. To do this, ensure you have [rust installed](https://doc.rust-lang.org/stable/book/ch01-01-installation.html). Build the application using the following command in the root of this repository:

```
cargo build --release
```

Then, copy the file `./target/release/tstamper.exe` to a directory of your choice to add to the `path` environment variable on your respective operating system. Personally, on my systems I add a `tools` directory to my `root` or `C:\` directory that all of these files go into, up to you.

After this, you're good to go.

# Use

By default, calling `tstamper` will return the current time as a UNIX timestamp, in seconds since epoch.

The following options are supported for changing the output format of `tstamper`:

- `-h` -> Show the program help
- `--ms` -> Output the UNIX timestamp in ms since epoch, instead of seconds.
- `--iso8601` -> Output the current local time in ISO8601/RFC3339 format with offset from UTC
- `--iso8601-ms` -> Output current UTC time in the ISO8601/RFC3339 format. (Still includes 00:00 offset)

Only one option per call to `tstamper` is currently supported. For example:

`tstamper --ms` -> Valid ✔️

`tstamper --ms --iso8601` -> NOT valid ❌

# Example Usage

**Output in seconds:**
```
tstamper
```

**Output in milliseconds:**

```
tstamper --ms
```