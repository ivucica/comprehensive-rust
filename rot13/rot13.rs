use std::io::Read;

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

// Implement the `Read` trait for `RotDecoder`.
impl<R> Read for RotDecoder<R> where R: Read {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self.input.read(buf) {
            Ok(n) => {
                // convert here
                for i in 0..n {
                    buf[i] = match buf[i] {
                        c if c >= b'A' && c <= b'Z' => {
                            if c + self.rot > b'Z' { b'A' + c + self.rot - b'Z' - 1 } else { c + self.rot }
                        },
                        c if c >= b'a'  && c <= b'z' => {
                            if c + self.rot > b'z' { b'a' + c + self.rot - b'z' - 1 } else { c + self.rot }
                        },
                        c => c,
                    };
                };
                Ok(n)
            },
            e => e,
        }
    }
}

fn main() {
    let mut rot =
        RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
    let mut result = String::new();
    rot.read_to_string(&mut result).unwrap();
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let mut rot =
            RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> { input: input.as_ref(), rot: 13 };
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);
        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}

