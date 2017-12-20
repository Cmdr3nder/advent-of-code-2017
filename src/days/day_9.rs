use util::files::read_file_to_string;

use std::str::Chars;

pub fn main() {
    println!("-- Running Day 9 --");
    let input = &read_file_to_string("src/days/input/day_9.txt");
    let root = parse_groups(input);
    let total_score = root.score(1);

    println!("Total score is {}", total_score);
}

struct Group {
    children: Vec<Group>
}

impl Group {
    fn score(self, self_score: u32) -> u32 {
        let mut total_score = self_score;

        for child_group in self.children {
            total_score = total_score + child_group.score(self_score + 1);
        }

        total_score
    }
}

fn parse_groups(input: &str) -> Group {
    let mut char_iter = input.chars();

    match char_iter.next() {
        Some(ch) => {
            if ch != '{' {
                panic!("Unexpected input {}", ch);
            }
        },
        None => panic!("No input given.")
    };

    parse_group(&mut char_iter)
}

fn parse_group(char_iter: &mut Chars) -> Group {
    let mut next = match char_iter.next() {
        Some(ch) => ch,
        None => panic!("Expected end of group, not end of input.")
    };

    let mut children = Vec::new();

    while next != '}' {
        if next == '{' {
            children.push(parse_group(char_iter));
        } else if next == '<' {
            parse_garbage(char_iter);
        } else if next == '!' {
            char_iter.next();
        } else if next == ',' || next.is_whitespace() {
            //ignore for now, extraneous structure
        } else {
            panic!("Unexpected input {}", next);
        }

        next = match char_iter.next() {
            Some(ch) => ch,
            None => panic!("Expected end of group, not end of input. {}", next)
        }
    }

    Group {
        children
    }
}

fn parse_garbage(char_iter: &mut Chars) {
    let mut next = match char_iter.next() {
        Some(ch) => ch,
        None => return
    };

    while next != '>' {
        if next == '!' {
            char_iter.next();
        }

        next = match char_iter.next() {
            Some(ch) => ch,
            None => return
        };
    }
}