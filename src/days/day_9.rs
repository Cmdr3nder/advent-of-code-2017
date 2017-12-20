use util::files::read_file_to_string;

use std::str::Chars;

pub fn main() {
    println!("-- Running Day 9 --");
    let input = &read_file_to_string("src/days/input/day_9.txt");
    let (root, garbage_count) = parse_groups(input);
    let total_score = root.score(1);

    println!("Total score is {}", total_score);
    println!("Garbage count is {}", garbage_count);
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

fn parse_groups(input: &str) -> (Group, u32) {
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

fn parse_group(char_iter: &mut Chars) -> (Group, u32) {
    let mut next = match char_iter.next() {
        Some(ch) => ch,
        None => panic!("Expected end of group, not end of input.")
    };

    let mut children = Vec::new();
    let mut garbage_count = 0;

    while next != '}' {
        if next == '{' {
            let (group, count) = parse_group(char_iter);
            children.push(group);
            garbage_count = garbage_count + count;
        } else if next == '<' {
            garbage_count = garbage_count + parse_garbage(char_iter);
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

    (Group {
        children
    }, garbage_count)
}

fn parse_garbage(char_iter: &mut Chars) -> u32 {
    let mut count = 0;
    let mut next = match char_iter.next() {
        Some(ch) => ch,
        None => return count
    };

    while next != '>' {
        if next == '!' {
            char_iter.next();
        } else {
            count = count + 1;
        }

        next = match char_iter.next() {
            Some(ch) => ch,
            None => return count
        };
    }

    count
}