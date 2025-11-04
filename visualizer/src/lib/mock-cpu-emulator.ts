// Mock CPU emulator - Replace this with your actual WASM module
// This simulates a simple 8-bit CPU for demonstration

import type { CPUState, ExecutionStep, CPUEmulator } from '../types/cpu';

export class MockCPUEmulator implements CPUEmulator {
  private state: CPUState;
  private program: string[];
  private halted: boolean;

  constructor() {
    this.state = this.getInitialState();
    this.program = [];
    this.halted = false;
  }

  private getInitialState(): CPUState {
    return {
      program_counter: 0,
      registers: {
        count: 4,
        regs: [0, 0, 0, 0],
      },
      flags: {
        zero: false,
        carry: false,
        sign: false,
        overflow: false,
      },
      program_memory: {
        mem: new Array(256).fill(0),
      },
      data_memory: {
        mem: new Array(256).fill(0),
      },
      stack_pointer: 0xFF,
    };
  }

  loadProgram(assembly: string): boolean {
    try {
      this.program = assembly
        .split('\n')
        .map(line => line.trim())
        .filter(line => line && !line.startsWith(';'));
      this.reset();
      return true;
    } catch (error) {
      console.error('Failed to load program:', error);
      return false;
    }
  }

  step(): ExecutionStep | null {
    if (this.halted || this.state.program_counter >= this.program.length) {
      this.halted = true;
      return null;
    }

    const instruction = this.program[this.state.program_counter];
    const address = this.state.program_counter;
    const changed_registers: string[] = [];
    const changed_flags: string[] = [];
    let memoryAccess: ExecutionStep['memoryAccess'] = undefined;

    // Simple instruction parser
    const parts = instruction.toUpperCase().split(/\s+/);
    const opcode = parts[0];

    const prevRegs = [...this.state.registers.regs];
    const prevFlags = { ...this.state.flags };

    const getRegIndex = (name: string): number => {
      const regNames = ['A', 'B', 'C', 'D'];
      return regNames.indexOf(name);
    };

    const getRegValue = (src: string): number => {
      if (src.startsWith('#')) {
        return parseInt(src.substring(1), 16);
      }
      const idx = getRegIndex(src);
      return idx >= 0 ? this.state.registers.regs[idx] : 0;
    };

    switch (opcode) {
      case 'MOV':
        if (parts.length === 3) {
          const [, dest, src] = parts;
          const destIdx = getRegIndex(dest);
          if (destIdx >= 0) {
            this.state.registers.regs[destIdx] = getRegValue(src) & 0xFF;
            changed_registers.push(`R${destIdx}`);
          }
        }
        break;

      case 'ADD':
        if (parts.length === 3) {
          const [, dest, src] = parts;
          const destIdx = getRegIndex(dest);
          if (destIdx >= 0) {
            const result = this.state.registers.regs[destIdx] + getRegValue(src);
            this.state.flags.carry = result > 0xFF;
            this.state.registers.regs[destIdx] = result & 0xFF;
            this.state.flags.zero = this.state.registers.regs[destIdx] === 0;
            changed_registers.push(`R${destIdx}`);
            changed_flags.push('carry', 'zero');
          }
        }
        break;

      case 'SUB':
        if (parts.length === 3) {
          const [, dest, src] = parts;
          const destIdx = getRegIndex(dest);
          if (destIdx >= 0) {
            const result = this.state.registers.regs[destIdx] - getRegValue(src);
            this.state.flags.carry = result < 0;
            this.state.registers.regs[destIdx] = result & 0xFF;
            this.state.flags.zero = this.state.registers.regs[destIdx] === 0;
            this.state.flags.sign = (this.state.registers.regs[destIdx] & 0x80) !== 0;
            changed_registers.push(`R${destIdx}`);
            changed_flags.push('carry', 'zero', 'sign');
          }
        }
        break;

      case 'INC':
        if (parts.length === 2) {
          const reg = parts[1];
          const idx = getRegIndex(reg);
          if (idx >= 0) {
            this.state.registers.regs[idx] = (this.state.registers.regs[idx] + 1) & 0xFF;
            this.state.flags.zero = this.state.registers.regs[idx] === 0;
            changed_registers.push(`R${idx}`);
            changed_flags.push('zero');
          }
        }
        break;

      case 'DEC':
        if (parts.length === 2) {
          const reg = parts[1];
          const idx = getRegIndex(reg);
          if (idx >= 0) {
            this.state.registers.regs[idx] = (this.state.registers.regs[idx] - 1) & 0xFF;
            this.state.flags.zero = this.state.registers.regs[idx] === 0;
            changed_registers.push(`R${idx}`);
            changed_flags.push('zero');
          }
        }
        break;

      case 'LOAD':
        if (parts.length === 3) {
          const [, reg, addr] = parts;
          const address = addr.startsWith('#') 
            ? parseInt(addr.substring(1), 16) 
            : parseInt(addr, 16);
          const idx = getRegIndex(reg);
          
          if (idx >= 0 && address < 256) {
            this.state.registers.regs[idx] = this.state.data_memory.mem[address];
            memoryAccess = { address, type: 'read', value: this.state.data_memory.mem[address] };
            changed_registers.push(`R${idx}`);
          }
        }
        break;

      case 'STORE':
        if (parts.length === 3) {
          const [, reg, addr] = parts;
          const address = addr.startsWith('#') 
            ? parseInt(addr.substring(1), 16) 
            : parseInt(addr, 16);
          const idx = getRegIndex(reg);
          
          if (idx >= 0 && address < 256) {
            const value = this.state.registers.regs[idx];
            this.state.data_memory.mem[address] = value;
            memoryAccess = { address, type: 'write', value };
          }
        }
        break;

      case 'HLT':
      case 'HALT':
        this.halted = true;
        break;
    }

    // Filter out unchanged flags
    const actualChangedFlags = changed_flags.filter(flag => 
      prevFlags[flag as keyof typeof prevFlags] !== this.state.flags[flag as keyof typeof this.state.flags]
    );

    this.state.program_counter++;

    return {
      instruction,
      address,
      changed_registers,
      changed_flags: actualChangedFlags,
      memoryAccess,
    };
  }

  reset(): void {
    this.state = this.getInitialState();
    this.halted = false;
  }

  getState(): CPUState {
    return { ...this.state };
  }

  isHalted(): boolean {
    return this.halted;
  }
}
