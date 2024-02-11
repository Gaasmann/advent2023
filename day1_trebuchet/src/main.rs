mod calibration_reader {
    use phf::phf_map;
    use std::fs;
    use std::io::BufRead;
    use std::io::BufReader;

    const NUMBERS: phf::Map<&str, char> = phf_map! {
        "zero" => '0',
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
    };
    pub fn get_calibration(filename: &str) -> u32 {
        let document_file = open_calibration_document(filename).unwrap();
        let mut result = 0u32;
        for line in get_line_iter(document_file) {
            let number = extract_number_from_string(line);
            result += number as u32;
        }
        result
    }

    fn open_calibration_document(filename: &str) -> std::io::Result<BufReader<fs::File>> {
        let file = fs::File::open(filename)?;
        Ok(BufReader::new(file))
    }

    fn get_line_iter(buffer: impl BufRead) -> impl Iterator<Item = String> {
        buffer.lines().map(|l| l.unwrap())
    }

    fn extract_number_from_string(string: String) -> u8 {
        let mut res_buf = String::new();
        let mut number_buf = String::new();
        // forward
        'outer: for chr in string.chars() {
            if chr.is_ascii_digit() {
                res_buf.push(chr);
                break;
            } else {
                number_buf.push(chr);
                for key in NUMBERS.keys() {
                    if number_buf.contains(key) {
                        res_buf.push(NUMBERS[key]);
                        break 'outer;
                    }
                }
            }
        }
        // backward
        number_buf.clear();
        'outer: for chr in string.chars().rev() {
            if chr.is_ascii_digit() {
                res_buf.push(chr);
                break;
            } else {
                number_buf.insert(0, chr);
                for key in NUMBERS.keys() {
                    if number_buf.contains(key) {
                        res_buf.push(NUMBERS[key]);
                        break 'outer;
                    }
                }
            }
        }
        // Conversion and return
        res_buf.parse().unwrap()
    }
}

use crate::calibration_reader::get_calibration;
fn main() {
    let result = get_calibration("adventofcode.com_2023_day_1_input.txt");
    println!("The result is {}.", result);
}
