// Copyright 2016 risc-v-emulator Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

pub mod encoding;
pub mod rv32i;
pub mod rv32m;

use std::fmt::Debug;
use cpu::CPU;

pub trait Instruction: Debug {
    fn execute(&self, cpu: &mut CPU);
    fn to_raw(&self) -> u32;
}

pub fn parse(instruction: u32) -> Option<Box<Instruction>> {
    match encoding::get_opcode(instruction) {
        0x03 => rv32i::Load::parse(instruction).map(|i| Box::new(i) as Box<Instruction>),
        // 0x07 => load_fp::LoadFp::parse(instruction).map(|i| Box::new(i) as Box<Instruction>),
        // 0x0F => misc_mem::MiscMem::parse(instruction).map(|i| Box::new(i) as Box<Instruction>),
        0x13 => rv32i::OpImm::parse(instruction).map(|i| Box::new(i) as Box<Instruction>),
        0x17 => rv32i::Auipc::parse(instruction).map(|i| Box::new(i) as Box<Instruction>),
        0x23 => rv32i::Store::parse(instruction).map(|i| Box::new(i) as Box<Instruction>),
        // 0x27 => store_fp::StoreFp::parse(instruction).map(|i| Box::new(i) as Box<Instruction>),
        // 0x2F => amo::Amo::parse(instruction).map(|i| Box::new(i) as Box<Instruction>),
        0x33 => {
            match encoding::get_funct7(instruction) {
                0x00 | 0x20 => {
                    rv32i::Op::parse(instruction).map(|i| Box::new(i) as Box<Instruction>)
                }
                0x01 => rv32m::Op::parse(instruction).map(|i| Box::new(i) as Box<Instruction>),
                _ => None,
            }
        }
        0x37 => rv32i::Lui::parse(instruction).map(|i| Box::new(i) as Box<Instruction>),
        // 0x43 => madd::Madd::parse(instruction).map(|i| Box::new(i) as Box<Instruction>),
        // 0x47 => msub::Msub::parse(instruction).map(|i| Box::new(i) as Box<Instruction>),
        // 0x4B => nmsub::Nmsub::parse(instruction).map(|i| Box::new(i) as Box<Instruction>),
        // 0x4F => nmadd::Nmadd::parse(instruction).map(|i| Box::new(i) as Box<Instruction>),
        // 0x53 => op_fp::OpFp::parse(instruction).map(|i| Box::new(i) as Box<Instruction>),
        0x63 => rv32i::Branch::parse(instruction).map(|i| Box::new(i) as Box<Instruction>),
        0x67 => rv32i::Jalr::parse(instruction).map(|i| Box::new(i) as Box<Instruction>),
        0x6F => rv32i::Jal::parse(instruction).map(|i| Box::new(i) as Box<Instruction>),
        // 0x73 => system::System::parse(instruction).map(|i| Box::new(i) as Box<Instruction>),
        op => {
            println!("unknown opcode: {:02x}", op);
            None
        }
    }
}
