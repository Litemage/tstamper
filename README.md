# TSTAMPER

`TSTAMPER` (Pronounced "tee-stamper") is a super simple command line utility written in the [rust](https://www.rust-lang.org) programming language for quickly retrieving the current unix timestamp. I wrote this program to avoid continually opening a browser and typing in google just to snag the current timestamp. 

**Future Features**

In time, I intend on adding the following features:

- Support for ISO 8601 formatting
- Support for local timezones

# Install

The recommended install proceadure is to clone this repository at the master branch, and build the utility yourself. To do this, ensure you have [rust installed](https://doc.rust-lang.org/stable/book/ch01-01-installation.html). Build the application using the following command in the root of this repository:

```
cargo build --release
```

Then, copy the file `./target/release/tstamper.exe` to a directory of your choice to add to the `path` environment variable on your respective operating system. Personally, on my systems I add a `tools` directory to my `root` or `C:\` directory that all of these files go into, up to you.

After this, you're good to go.

# Use

To use the program, there are currently two switches:

- `-h` -> Show the program help
- `--ms` -> Output the UNIX timestamp in ms since epoch, instead of seconds.

Running `tstamper` without arguments outputs the time since UNIX epoch in seconds. Adding the `--ms` switch outputs the timestamp in ms.

**Output in seconds:**
```
tstamper
```

**Output in milliseconds:**

```
tstamper --ms
```