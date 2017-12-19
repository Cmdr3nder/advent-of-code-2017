use util::files::read_file_to_string;

use std::collections::HashMap;

pub fn main() {
    println!("-- Running Day 8 --");
    let input = &read_file_to_string("src/days/input/day_8.txt");
    let instructions = parse_instructions(input);
    let mut registers: HashMap<RegisterAddress, RegisterValue> = HashMap::new();

    let largest_during_execution = execute_instructions(instructions, &mut registers);
    let (_, largest_value) = largest_register(&registers);

    println!("Largest register value after instructions: {}", largest_value);
    println!("Largest register during instructions: {}", largest_during_execution);
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in input.lines() {
        instructions.push(Instruction::from(line));
    }

    instructions
}

fn execute_instructions(instructions: Vec<Instruction>, against: &mut HashMap<RegisterAddress, RegisterValue>) -> RegisterValue {
    let mut largest = RegisterValue::min_value();

    for instruction in instructions {
        match instruction.execute(against) {
            Some(value) => {
                if value > largest {
                    largest = value;
                }
            },
            None => continue
        };
    }

    largest
}

fn largest_register(registers: &HashMap<RegisterAddress, RegisterValue>) -> (RegisterAddress, RegisterValue) {
    let mut largest = (String::from(""), RegisterValue::min_value());

    for (register_address, register_value) in registers {
        if *register_value > largest.1 {
            largest = (register_address.clone(), *register_value);
        }
    }

    largest
}

type RegisterAddress = String;

type RegisterValue = i64;

enum RegisterAction {
    Increment {target: RegisterAddress, amount: RegisterValue},
    Decrement {target: RegisterAddress, amount: RegisterValue}
}

impl RegisterAction {
    fn apply(self, to: &mut HashMap<RegisterAddress, RegisterValue>) -> RegisterValue {
        match self {
            RegisterAction::Increment {target, amount} => {
                match to.remove(&target) {
                    Some(value) => {
                        let new_value = value + amount;
                        to.insert(target, new_value);
                        new_value
                    },
                    None => {
                        let new_value = 0 + amount;
                        to.insert(target, new_value);
                        new_value
                    }
                }
            },
            RegisterAction::Decrement {target, amount} => {
                match to.remove(&target) {
                    Some(value) => {
                        let new_value = value - amount;
                        to.insert(target, new_value);
                        new_value
                    },
                    None => {
                        let new_value = 0 - amount;
                        to.insert(target, new_value);
                        new_value
                    }
                }
            }
        }
    }

    fn from(action_text: &str) -> RegisterAction {
        let parts: Vec<&str> = action_text.split_whitespace().collect();

        if parts.len() == 3 {
            let register_address: RegisterAddress = String::from(parts[0]);
            let register_value: RegisterValue = match parts[2].parse::<RegisterValue>() {
                Ok(value) => value,
                Err(err) => panic!("Couldn't parse action value: {}", err)
            };

            return match parts[1] {
                "inc" => {
                    RegisterAction::Increment {
                        target: register_address,
                        amount: register_value
                    }
                },
                "dec" => {
                    RegisterAction::Decrement {
                        target: register_address,
                        amount: register_value
                    }
                },
                _ => {
                    panic!("Unrecognized action type: {}", parts[1]);
                }
            };
        } else {
            panic!("Action not properly formatted: {}", action_text);
        }
    }
}

enum RegisterCondition {
    LessThan {target: RegisterAddress, amount: RegisterValue},
    LessThanOrEqual {target: RegisterAddress, amount: RegisterValue},
    Equal {target: RegisterAddress, amount: RegisterValue},
    NotEqual {target: RegisterAddress, amount: RegisterValue},
    GreaterThanOrEqual {target: RegisterAddress, amount: RegisterValue},
    GreaterThan {target: RegisterAddress, amount: RegisterValue}
}

impl RegisterCondition {
    fn compare(self, to: &HashMap<RegisterAddress, RegisterValue>) -> bool {
        match self {
            RegisterCondition::LessThan {target, amount} => {
                match to.get(&target) {
                    Some(value) => *value < amount,
                    None => 0 < amount
                }
            },
            RegisterCondition::LessThanOrEqual {target, amount} => {
                match to.get(&target) {
                    Some(value) => *value <= amount,
                    None => 0 <= amount
                }
            },
            RegisterCondition::Equal {target, amount} => {
                match to.get(&target) {
                    Some(value) => *value == amount,
                    None => 0 == amount
                }
            },
            RegisterCondition::NotEqual {target, amount} => {
                match to.get(&target) {
                    Some(value) => *value != amount,
                    None => 0 != amount
                }
            },
            RegisterCondition::GreaterThanOrEqual {target, amount} => {
                match to.get(&target) {
                    Some(value) => *value >= amount,
                    None => 0 >= amount
                }
            },
            RegisterCondition::GreaterThan {target, amount} => {
                match to.get(&target) {
                    Some(value) => *value > amount,
                    None => 0 > amount
                }
            }
        }
    }

    fn from(condition_text: &str) -> RegisterCondition {
        let parts: Vec<&str> = condition_text.split_whitespace().collect();

        if parts.len() == 3 {
            let register_address: RegisterAddress = String::from(parts[0]);
            let register_value: RegisterValue = match parts[2].parse::<RegisterValue>() {
                Ok(value) => value,
                Err(err) => panic!("Couldn't parse value: {}", err)
            };

            return match parts[1] {
                "<" => {
                    RegisterCondition::LessThan {
                        target: register_address,
                        amount: register_value
                    }
                },
                "<=" => {
                    RegisterCondition::LessThanOrEqual {
                        target: register_address,
                        amount: register_value
                    }
                },
                "==" => {
                    RegisterCondition::Equal {
                        target: register_address,
                        amount: register_value
                    }
                },
                "!=" => {
                    RegisterCondition::NotEqual {
                        target: register_address,
                        amount: register_value
                    }
                },
                ">=" => {
                    RegisterCondition::GreaterThanOrEqual {
                        target: register_address,
                        amount: register_value
                    }
                },
                ">" => {
                    RegisterCondition::GreaterThan {
                        target: register_address,
                        amount: register_value
                    }
                },
                _ => {
                    panic!("Unrecognized comparison: {}", parts[1]);
                }
            }
        } else {
            panic!("Condition not properly formatted: {}", condition_text);
        }
    }
}

struct Instruction {
    action: RegisterAction,
    condition: RegisterCondition
}

impl Instruction {
    fn execute(self, against: &mut HashMap<RegisterAddress, RegisterValue>) -> Option<RegisterValue> {
        if self.condition.compare(against) {
            Some(self.action.apply(against))
        } else {
            None
        }
    }

    fn from(instruction_text: &str) -> Instruction {
        let parts: Vec<&str> = instruction_text.split("if").map(|part| part.trim()).collect();

        if parts.len() == 2 {
            let action = RegisterAction::from(parts[0]);
            let condition = RegisterCondition::from(parts[1]);

            Instruction {
                action,
                condition
            }
        } else {
            panic!("Instruction not formatted correctly: {}", instruction_text);
        }
    }
}