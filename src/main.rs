use slurp;
use std::primitive::char;
use strum_macros::EnumString;
use substring::Substring;

const HASH_TABLE_SIZE: usize = 26;
const MAX_ENTRY_SIZE: usize = 11;

#[derive(Debug, PartialEq, EnumString, Clone)]
enum SlotCondition {
    NeverUsed,
    Tombstone,
    Occupied,
}

#[derive(Debug, Clone)]
struct Slot {
    data: Option<String>,
    condition: SlotCondition,
}

#[derive(Debug, Clone)]
struct SpecHashTable {
    slots: Vec<Slot>,
}

impl SpecHashTable {
    fn print_it(&self) {
        let mut first_time = true;

        for slot in &self.slots {
            match &slot.data {
                Some(i) => {
                    if slot.condition == SlotCondition::Occupied {
                        if first_time {
                            print!("{}", i);
                        } else {
                            print!(" {}", i);
                        }
                    }
                    first_time = false;
                }
                _ => {}
            };
        }
    }

    fn find_it(&self, entry: String) -> (bool, usize) {
        let mut found_it: bool = false;
        let mut slot_index: usize = 0;

        for slot in &self.slots {
            // do we have slot that exactly matches
            if let Some(i) = &slot.data {
                // does the slot string equal what we are searching for?
                if i == &entry && matches!(&slot.condition, SlotCondition::Occupied) {
                    found_it = true;
                    break;
                }
            }
            slot_index += 1;
        }

        (found_it, slot_index)
    }

    fn find_insertion_point(&self, one_letter_hash: String) -> u32 {
        let char_vec: Vec<char> = one_letter_hash.chars().collect();
        let mut hash_position = (char_vec[0]) as u32 - 97;

        loop {
            if self.slots[hash_position as usize].condition == SlotCondition::NeverUsed
                || self.slots[hash_position as usize].condition == SlotCondition::Tombstone
            {
                return hash_position;
            }
            hash_position = (hash_position + 1) % HASH_TABLE_SIZE as u32;
        }
    }
}

fn main() {
    let mut sht: SpecHashTable = SpecHashTable {
        slots: vec![
            Slot {
                data: None,
                condition: SlotCondition::NeverUsed
            };
            HASH_TABLE_SIZE
        ],
    };

    let input_file: Vec<String> = slurp::read_all_lines("./input.txt").unwrap();

    for line in input_file.iter() {
        let line_length = line.len();

        if line_length > MAX_ENTRY_SIZE {
            break;
        }

        let one_letter_hash = line.substring(line_length - 1, line_length);
        let entry = line.substring(1, line_length);
        let action = line.substring(0, 1);

        match action {
            "A" => {
                let (found_it, _postion) = sht.find_it(entry.to_string());
                if found_it {
                    break;
                }
                let index = sht.find_insertion_point(one_letter_hash.to_string());
                sht.slots[index as usize] = Slot {
                    data: Some(entry.to_string()),
                    condition: SlotCondition::Occupied,
                };
            }
            "D" => {
                let (found_it, position) = sht.find_it(entry.to_string());
                if !found_it {
                    break;
                }
                sht.slots[position] = Slot {
                    data: Some(entry.to_string()),
                    condition: SlotCondition::Tombstone,
                };
            }
            _ => (),
        };
    }

    sht.print_it();
}
