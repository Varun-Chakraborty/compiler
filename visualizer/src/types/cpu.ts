// CPU state types for the 8-bit CPU visualizer

export interface CPUState {
  program_counter: number;
  registers: {
    count: number;
    regs: number[];
  };
  flags: {
    zero: boolean;
    carry: boolean;
    sign: boolean;
    overflow: boolean;
  };
  program_memory: {
    mem: number[];
  };
  data_memory: {
    mem: number[];
  };
  stack_pointer: number;
}

export interface ExecutionStep {
  instruction: string;
  address: number;
  changed_registers: string[];
  changed_flags: string[];
  memory_access?: {
    address: number;
    type_: 'read' | 'write';
    value: number;
  };
  stack_pointer: number;
  is_halted: boolean;
}

export interface CPUEmulator {
  loadProgram: (assembly: string) => boolean;
  step: () => ExecutionStep | null;
  reset: () => void;
  getState: () => CPUState;
  isHalted: () => boolean;
}
