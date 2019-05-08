use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

trait LossyLine {
    fn lossy_read_line(&mut self, buf: &mut String) -> std::io::Result<usize>;
}

impl<T: std::io::Read> LossyLine for BufReader<T> {
    fn lossy_read_line(&mut self, buf: &mut String) -> std::io::Result<usize> {
        let mut bytes = vec![];
        self.read_until(b'\n', &mut bytes).map(|n| {
            *buf = String::from_utf8_lossy(&bytes)
                .trim_end_matches('\r')
                .to_string();
            n
        })
    }
}

fn main() {
    let f: fs::File = fs::File::open("invalid.txt").expect("could not open input file");
    let mut line = String::new();
    let mut reader = BufReader::new(f);
    match reader.lossy_read_line(&mut line) {
        Err(e) => {
            println!("error reading one line: {} (line: {})", e, line);
            let mut buf = vec![];
            match reader.read_to_end(&mut buf) {
                // match reader.read_until(0xA, &mut buf) {
                Err(again) => println!("also error {}", again),
                Ok(len) => {
                    println!("did read {} bytes, buf: {:02X?}", len, buf);
                }
            }
        }
        Ok(len) => println!("did read {} bytes: {}", len, &line),
    }
}
