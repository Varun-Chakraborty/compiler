use args::Args;
use assembler::MyAssembler;
use cpu::MyCPU;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use web_sys::console;

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
pub enum Type {
    Read,
    Write,
}

#[derive(Serialize)]
pub struct MemAccess {
    pub address: u32,
    pub value: u8,
    pub type_: Type,
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
    pub memory_access: Option<MemAccess>,
    pub is_halted: bool,
    pub stack_pointer: u32,
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

    // explicit destructor you should call from JS before re-init/HMR
    #[wasm_bindgen(js_name = free)]
    pub fn free(self) {
        // consumed and dropped here
    }

    #[wasm_bindgen(js_name = loadProgram)]
    pub fn load_program(&mut self, assembly_string: String) -> bool {
        match self.assembler.assemble(assembly_string) {
            Ok((binary, _)) => {
                // produce a human readable binary string and log it to browser console
                let byte_strs: Vec<String> = binary.iter().map(|byte| format!("{:08b}", byte)).collect();
                let binary_str = byte_strs.join(" ");
                console::log_1(&JsValue::from_str(&binary_str));
                console::log_1(&JsValue::from_str("Program assembled"));
                self.cpu.load_binary(binary).is_ok()
            },
            Err(e) => {
                console::error_1(&JsValue::from_str(&format!("assemble error: {:?}", e)));
                false
            },
        }
    }

    #[wasm_bindgen]
    pub fn step(&mut self) -> JsValue {
        // deterministic log to ensure we can see we reached this point
        console::log_1(&JsValue::from_str("Executing step function"));

        match self.cpu.step() {
            Ok(step_info) => {
                // Build owned snapshot for JS
                let js_step = JsExecutionStep {
                    instruction: step_info.instruction_str,
                    address: step_info.address,
                    changed_registers: step_info.changed_regs,
                    changed_flags: step_info.changed_flags,
                    memory_access: step_info.memory_access.map(|ma| MemAccess {
                        address: ma.address,
                        value: ma.value,
                        type_: if ma.type_ == cpu::Type::Read { Type::Read } else { Type::Write },
                    }),
                    is_halted: step_info.is_halted,
                    stack_pointer: step_info.stack_pointer,
                };

                match serde_wasm_bindgen::to_value(&js_step) {
                    Ok(v) => v,
                    Err(e) => {
                        let msg = format!("serialize step error: {}", e);
                        console::error_1(&JsValue::from_str(&msg));
                        JsValue::from_str(&msg)
                    }
                }
            }
            Err(e) => {
                let msg = format!("step error: {:?}", e);
                console::error_1(&JsValue::from_str(&msg));
                JsValue::from_str(&msg)
            }
        }
    }

    #[wasm_bindgen(js_name = getState)]
    pub fn get_state(&self) -> JsValue {
        // clone snapshot from CPU state to avoid exposing internal shared pointers
        let state = self.cpu.get_state_struct().clone();
        let jscpustate = JsCPUState {
            program_counter: state.program_counter,
            registers: JsRegisters {
                count: state.registers.count,
                regs: state.registers.regs.clone(), // <-- clone for owned snapshot
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

        match serde_wasm_bindgen::to_value(&jscpustate) {
            Ok(v) => v,
            Err(e) => {
                let msg = format!("get_state serialize error: {}", e);
                console::error_1(&JsValue::from_str(&msg));
                JsValue::from_str(&msg)
            }
        }
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.cpu.reset();
    }
}
