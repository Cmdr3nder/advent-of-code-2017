use util::files::read_file_to_string;

use std::collections::HashMap;

pub fn main() {
    println!("-- Running Day 13 --");
    let input = &read_file_to_string("src/days/input/day_13.txt");
    let scanners = parse_scanners(input);

    let severity = determine_severity(&scanners, 0);
    println!("Severity of trip: {}", severity);

    let success_delay = determine_delay(&scanners);
    println!("Delay for success: {}", success_delay);
}

fn parse_scanners(input: &str) -> HashMap<u8, u8> {
    let mut scanners = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let scanner_depth = parts[0].parse::<u8>().unwrap();
        let scanner_range = parts[1].parse::<u8>().unwrap();

        scanners.insert(scanner_depth, scanner_range);
    }

    scanners
}

fn determine_delay(scanners: &HashMap<u8, u8>) -> usize {
    let mut delay = 0;

    while determine_severity(scanners, delay) > 0 {
        delay = delay + 1;
    }

    delay
}

fn determine_severity(scanners: &HashMap<u8, u8>, delay: usize) -> usize {
    let max_depth = max_scanned_depth(scanners);
    let mut scanners = init_scanners(scanners);
    let mut severity = 0usize;

    for _ in 0..delay {
        scan_forward(&mut scanners);
    }

    for depth in 0..(max_depth + 1) {
        if let Some(scanner) = scanners.get(&depth) {
            if scanner.detect() {
                let depth = depth as usize;
                let range = scanner.range as usize;
                severity = severity + (depth * range);
            }
        }

        scan_forward(&mut scanners);
    }

    severity
}

fn max_scanned_depth(scanners: &HashMap<u8, u8>) -> u8 {
    let mut max = u8::min_value();

    for key in scanners.keys() {
        if *key > max {
            max = *key;
        }
    }

    max
}

#[derive(Copy, Clone)]
struct Scanner {
    range: u8,
    pos: u8,
    facing: Facing
}

#[derive(Copy, Clone)]
enum Facing {
    End,
    Beginning
}

impl Scanner {
    fn detect(self) -> bool {
        self.pos == 0
    }

    fn scan(self) -> Scanner {
        match self.facing {
            Facing::Beginning => {
                if self.pos == 0 {
                    Scanner {
                        range: self.range,
                        pos: 1,
                        facing: Facing::End
                    }
                } else {
                    Scanner {
                        range: self.range,
                        pos: self.pos - 1,
                        facing: self.facing
                    }
                }
            },
            Facing::End => {
                if self.pos == (self.range - 1) {
                    Scanner {
                        range: self.range,
                        pos: self.range - 2,
                        facing: Facing::Beginning
                    }
                } else {
                    Scanner {
                        range: self.range,
                        pos: self.pos + 1,
                        facing: self.facing
                    }
                }
            }
        }
    }

    fn new(range: u8) -> Scanner {
        Scanner {
            range,
            pos: 0,
            facing: Facing::End
        }
    }
}

fn init_scanners(scanners: &HashMap<u8, u8>) -> HashMap<u8, Scanner> {
    scanners.into_iter().map(|(key, range)| {
        (*key, Scanner::new(*range))
    }).collect()
}

fn scan_forward(scanners: &mut HashMap<u8, Scanner>) {
    let keys: Vec<u8> = scanners.keys().map(|key| *key).collect();

    for key in keys {
        if let Some(scanner) = scanners.remove(&key) {
            scanners.insert(key, scanner.scan());
        }
    }
}