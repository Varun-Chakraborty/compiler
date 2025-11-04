use core::panic;

use args::Args;
use assembler::MyAssembler;
use cpu::MyCPU;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct JsRegisters {
    pub count: u32,
    pub regs: Vec<u8>,
}

#[derive(Serialize)]
pub struct JsFlags {
    pub zero: bool,
    pub carry: bool,
    pub sign: bool,
    pub overflow: bool,
}

#[derive(Serialize)]
pub struct JsMemory {
    pub mem: Vec<u8>,
}

#[derive(Serialize)]
pub struct JsInstruction {
    pub name: String,
    pub operands: Vec<u32>,
}

#[derive(Serialize)]
pub struct MemAccess {
    pub address: u32,
    pub value: u8,
}

#[derive(Serialize)]
pub struct JsCPUState {
    pub program_counter: u32,
    pub registers: JsRegisters,
    pub flags: JsFlags,
    pub program_memory: JsMemory,
    pub data_memory: JsMemory,
    pub stack_pointer: u32,
}

#[derive(Serialize)]
pub struct JsExecutionStep {
    pub instruction: String,
    pub address: u32,
    pub changed_registers: Vec<String>,
    pub changed_flags: Vec<String>,
}

#[wasm_bindgen]
pub struct MyCpuController {
    cpu: MyCPU,
    assembler: MyAssembler,
}

#[wasm_bindgen]
impl MyCpuController {
    #[wasm_bindgen(constructor)]
    pub fn new() -> MyCpuController {
        console_error_panic_hook::set_once();
        let args = Args::default();
        let cpu = MyCPU::new(&args).expect("Failed to create CPU");
        let assembler = MyAssembler::new(&args).expect("Failed to create Assembler");
        MyCpuController { cpu, assembler }
    }

    #[wasm_bindgen(js_name = loadProgram)]
    pub fn load_program(&mut self, assembly_string: String) -> bool {
        match self.assembler.assemble(assembly_string) {
            Ok((binary, _)) => {
                println!("Program assembled");
                self.cpu.load_binary(binary).is_ok()
            },
            Err(_) => false,
        }
    }

    #[wasm_bindgen]
    pub fn step(&mut self) -> JsValue {
        panic!("Executing step function");
        match self.cpu.step() {
            Ok(step_info) => {
                let js_step = JsExecutionStep {
                    instruction: step_info.instruction_str,
                    address: step_info.address,
                    changed_registers: step_info.changed_regs,
                    changed_flags: step_info.changed_flags,
                };

                serde_wasm_bindgen::to_value(&js_step).unwrap()
            }
            Err(_) => JsValue::NULL,
        }
        // panic!("Executing step function");
    }

    #[wasm_bindgen(js_name = getState)]
    pub fn get_state(&self) -> JsValue {
        let state = self.cpu.get_state_struct().clone();
        let jscpustate = JsCPUState {
            program_counter: state.program_counter,
            registers: JsRegisters {
                count: state.registers.count,
                regs: state.registers.regs,
            },
            flags: JsFlags {
                zero: state.flags.zero,
                carry: state.flags.carry,
                sign: state.flags.sign,
                overflow: state.flags.overflow,
            },
            program_memory: JsMemory { mem: state.program_memory.mem },
            data_memory: JsMemory { mem: state.data_memory.mem },
            stack_pointer: state.stack_pointer,
        };

        serde_wasm_bindgen::to_value(&jscpustate).unwrap()
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    #[wasm_bindgen(js_name = isHalted)]
    pub fn is_halted(&self) -> bool {
        self.cpu.is_halted()
    }
}
