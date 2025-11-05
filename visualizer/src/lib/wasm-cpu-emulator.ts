import type { CPUEmulator, CPUState, ExecutionStep } from '../types/cpu';

import init, { MyCpuController } from './wasm-cpu';

export class WasmCPUEmulator implements CPUEmulator {
  private cpu: MyCpuController | null = null;
  private static wasmInitialized = false;
  private is_halted = false;

  public static async initialize(): Promise<WasmCPUEmulator> {
    if (!WasmCPUEmulator.wasmInitialized) {
      await init(); // This loads the .wasm file
      WasmCPUEmulator.wasmInitialized = true;
    }

    return new WasmCPUEmulator();
  }


  private constructor() {
    this.cpu = new MyCpuController(); // This calls your Rust constructor
  }

  loadProgram(assembly: string): boolean {
    if (!this.cpu) return false;

    try {
      return this.cpu.loadProgram(assembly);
    } catch (error) {
      console.error('Failed to load program:', error);
      return false;
    }
  }

  step(): ExecutionStep | null {
    if (!this.cpu) return null;

    try {
      const result = this.cpu.step();
      console.log(result);
      this.is_halted = result.is_halted;
      return {
        instruction: result.instruction,
        address: result.address,
        changed_registers: result.changed_registers || [],
        changed_flags: result.changed_flags || [],
        is_halted: result.is_halted,
        memory_access: result.memory_access,
        stack_pointer: result.stack_pointer
      };
    } catch (error) {
      console.error('Step execution failed:', error);
      return null;
    }
  }

  reset(): void {
    if (this.cpu) this.cpu.reset();
  }

  getState(): CPUState {
    if (!this.cpu) throw new Error("CPU not initialized");

    return this.cpu.getState() as CPUState;
  }

  isHalted(): boolean {
    if (!this.cpu) return true;

    return this.is_halted;
  }
}
