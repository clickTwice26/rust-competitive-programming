use std::io::{self, BufWriter, Write, Read};

fn solve<R: Read, W: Write>(inner_stdin: &mut R, inner_stdout: &mut W) {
    let mut buffer = String::new();
    inner_stdin.read_to_string(&mut buffer).unwrap();
    let mut token_scanner = buffer.split_whitespace();

    macro_rules! next {
        ($t:ty) => {
            token_scanner.next().unwrap().parse::<$t>().unwrap()
        };
    }

    let w: i32 = next!(i32);
    if w > 2 && w % 2 == 0 {
        writeln!(inner_stdout, "YES").unwrap();
    } else {
        writeln!(inner_stdout, "NO").unwrap();
    }
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut inner_stdin = stdin.lock();
    let mut inner_stdout = BufWriter::new(stdout.lock());

    solve(&mut inner_stdin, &mut inner_stdout);
    inner_stdout.flush().unwrap();
}
