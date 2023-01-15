use std::io::stdout;

use std::io::Write;

fn main() {
    let mut output = stdout();

    writeln!(output, "Hello world!");
}
