// Virtual Machine: iitbcpu (IIT Bombay CPU)
// This crate provides a simple way to emulate the IITB RISC-V Processor.
// Reference for emulating a pipline processor: https://patents.google.com/patent/US9483405B2/en

use std::collections::HashMap;

pub struct RAM {
    // implemented as a HashMap to store the memory addresses and their values.
    // if the memory address is not present in the HashMap, it is assumed to be 0.
    pub memory: HashMap<i16, i16>, // Address are of 16 bits hence 2^16 = 65536 bits of memory.
}

impl RAM {
    // Function to read from the memory.
    pub fn read(&self, address: i16) -> i16 {
        match self.memory.get(&address) {
            Some(value) => *value,
            None => 0,
        }
    }

    // Function to write to the memory.
    pub fn write(&mut self, address: i16, value: i16) {
        self.memory.insert(address, value);
    }
}
