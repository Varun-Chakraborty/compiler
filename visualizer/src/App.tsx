import { useState, useEffect, useRef } from 'react';
import { AssemblyEditor } from './components/AssemblyEditor';
import { CPUStateDisplay } from './components/CPUStateDisplay';
import { MemoryViewer } from './components/MemoryViewer';
import { ControlPanel } from './components/ControlPanel';
import { MockCPUEmulator } from './lib/mock-cpu-emulator';
import { WasmCPUEmulator } from './lib/wasm-cpu-emulator';
import type { CPUState, ExecutionStep, CPUEmulator } from './types/cpu';
import { Card } from './components/ui/card';
import { Cpu, AlertCircle } from 'lucide-react';
import { toast } from 'sonner';
import { Alert, AlertDescription } from './components/ui/alert';

const DEFAULT_PROGRAM = `; Simple program
MOVEI R0, 16
MOVEI R1, 6
ADD R0, R1
MOVEM R0, 0
HALT`;

export default function App() {
  const [code, setCode] = useState(DEFAULT_PROGRAM);
  const [cpuState, setCpuState] = useState<CPUState | null>(null);
  const [currentStep, setCurrentStep] = useState<ExecutionStep | null>(null);
  const [lineNumber, setLineNumber] = useState(0);
  const [isRunning, setIsRunning] = useState(false);
  const [isHalted, setIsHalted] = useState(true);
  const [executionSpeed, setExecutionSpeed] = useState(500);
  const [changedRegisters, setChangedRegisters] = useState<string[]>([]);
  const [changedFlags, setChangedFlags] = useState<string[]>([]);
  const [usingMockEmulator, setUsingMockEmulator] = useState(true);

  const emulatorRef = useRef<CPUEmulator>(new MockCPUEmulator());
  const runIntervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  useEffect(() => {
    // Try to load WASM emulator, fall back to mock if it fails
    const initEmulator = async () => {
      WasmCPUEmulator.initialize().then((wasmEmulator) => {
        if (wasmEmulator) {
          emulatorRef.current = wasmEmulator;
          setUsingMockEmulator(false);
          toast.success('WASM CPU emulator loaded');
        }
      }).catch((error) => {
        console.warn('Failed to load WASM emulator, using mock:', error);
      });

      // Initialize CPU state
      setCpuState(emulatorRef.current.getState());
    };

    initEmulator();
  }, []);

  const handleLoadProgram = () => {
    const success = emulatorRef.current.loadProgram(code);
    if (success) {
      setCpuState(emulatorRef.current.getState());
      setCurrentStep(null);
      setChangedRegisters([]);
      setChangedFlags([]);
      setIsHalted(false);
      toast.success('Program loaded successfully');
    } else {
      toast.error('Failed to load program');
    }
  };

  const handleStep = () => {
    if (isHalted) return;
    const step = emulatorRef.current.step();

    if (!step) return;

    setLineNumber(prev => prev + 1);

    setCurrentStep(step);
    setCpuState(emulatorRef.current.getState());
    setChangedRegisters(step.changed_registers);
    setChangedFlags(step.changed_flags);
    if (step.is_halted) {
      setIsHalted(true);
      setIsRunning(false);
      toast.info('Program halted');
    }

    // Clear highlights after a delay
    setTimeout(() => {
      setChangedRegisters([]);
      setChangedFlags([]);
    }, 800);
  };

  const handleRun = () => {
    if (isHalted) return;

    setIsRunning(true);

    const executeStep = () => {
      const step = emulatorRef.current.step();

      if (!step) return;

      setLineNumber(prev => prev + 1);

      setCurrentStep(step);
      setCpuState(emulatorRef.current.getState());
      setChangedRegisters(step.changed_registers);
      setChangedFlags(step.changed_flags);
      if (step.is_halted) {
        setIsHalted(true);
        setIsRunning(false);
        if (runIntervalRef.current) {
          clearInterval(runIntervalRef.current);
        }
        toast.info('Program halted');
      }

      setTimeout(() => {
        setChangedRegisters([]);
        setChangedFlags([]);
      }, Math.min(300, executionSpeed / 2));

    };

    if (executionSpeed === 0) {
      // Max speed - use requestAnimationFrame
      const animate = () => {
        if (!emulatorRef.current.isHalted()) {
          executeStep();
          requestAnimationFrame(animate);
        }
      };
      requestAnimationFrame(animate);
    } else {
      // Use interval for controlled speed
      runIntervalRef.current = setInterval(executeStep, executionSpeed);
    }
  };

  const handlePause = () => {
    setIsRunning(false);
    if (runIntervalRef.current) {
      clearInterval(runIntervalRef.current);
      runIntervalRef.current = null;
    }
  };

  const handleReset = () => {
    handlePause();
    emulatorRef.current.reset();
    setCpuState(emulatorRef.current.getState());
    setCurrentStep(null);
    setChangedRegisters([]);
    setChangedFlags([]);
    setIsHalted(false);
    toast.success('CPU reset');
  };

  useEffect(() => {
    return () => {
      if (runIntervalRef.current) {
        clearInterval(runIntervalRef.current);
      }
    };
  }, []);

  return (
    <div className="min-h-screen bg-zinc-950 text-zinc-100 p-6">
      <div className="max-w-[1800px] mx-auto space-y-6">
        {/* Header */}
        <div className="flex items-center gap-3">
          <div className="p-3 bg-zinc-900 rounded-lg border border-zinc-800">
            <Cpu className="w-6 h-6 text-amber-400" />
          </div>
          <div className="flex-1">
            <h1 className="text-zinc-100">8-bit CPU Visualizer</h1>
            <p className="text-sm text-zinc-500">
              Interactive assembly code execution and state visualization
            </p>
          </div>
          {usingMockEmulator && (
            <Alert className="w-auto bg-amber-500/10 border-amber-500/30">
              <AlertCircle className="h-4 w-4 text-amber-500" />
              <AlertDescription className="text-xs text-amber-200">
                Using mock emulator
              </AlertDescription>
            </Alert>
          )}
        </div>

        {/* Main content */}
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          {/* Left column - Code editor */}
          <div className="lg:col-span-1">
            <Card className="p-4 bg-zinc-900 border-zinc-800 h-[600px] flex flex-col">
              <AssemblyEditor
                code={code}
                onChange={setCode}
                currentLine={lineNumber}
              />
            </Card>
          </div>

          {/* Middle column - CPU state */}
          <div className="lg:col-span-1 space-y-4">
            <ControlPanel
              onStep={handleStep}
              onRun={handleRun}
              onPause={handlePause}
              onReset={handleReset}
              onLoad={handleLoadProgram}
              isRunning={isRunning}
              isHalted={isHalted}
              executionSpeed={executionSpeed}
              onSpeedChange={setExecutionSpeed}
            />

            {cpuState && (
              <CPUStateDisplay
                state={cpuState}
                changedRegisters={changedRegisters}
                changedFlags={changedFlags}
              />
            )}

            {/* Current instruction */}
            {currentStep && (
              <Card className="p-4 bg-zinc-900 border-zinc-800">
                <h3 className="text-sm mb-2 text-zinc-400">Current Instruction</h3>
                <div className="p-3 bg-zinc-950 rounded-lg border border-amber-500/30">
                  <div className="text-xs text-zinc-500 mb-1">
                    Address: 0x{currentStep.address.toString(16).toUpperCase().padStart(2, '0')}
                  </div>
                  <div className="font-mono text-amber-300">
                    {currentStep.instruction}
                  </div>
                </div>
              </Card>
            )}
          </div>

          {/* Right column - Memory */}
          <div className="lg:col-span-1">
            <div className="h-[600px]">
              {cpuState && (
                <MemoryViewer
                  programMemory={cpuState.program_memory.mem}
                  dataMemory={cpuState.data_memory.mem}
                  accessedAddress={currentStep?.memory_access?.address}
                  accessType={currentStep?.memory_access?.type_}
                  activeMemoryType="data"
                />
              )}
            </div>
          </div>
        </div>

        {/* Footer info */}
        <Card className="p-4 bg-zinc-900 border-zinc-800">
          <h3 className="text-sm mb-2 text-zinc-400">Instructions</h3>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 text-xs text-zinc-500">
            <div>
              <div className="font-mono text-zinc-300 mb-1">MOVEI reg immediate</div>
              <div>Move value to register</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">MOVER reg addr</div>
              <div>Move value from src memory addr to dest register</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">MOVEM reg addr</div>
              <div>Move value from src register to dest memory addr</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">ADD reg1 reg2</div>
              <div>Add two registers</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">ADDI reg immediate</div>
              <div>Add value to register</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">ADC reg1 reg2</div>
              <div>Add two registers with carry</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">ADCI reg immediate</div>
              <div>Add value to register with carry</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">SUB reg1 reg2</div>
              <div>Subtract two registers</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">SUBI reg immediate</div>
              <div>Subtract value from register</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">SBC reg1 reg2</div>
              <div>Subtract two registers with carry</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">SBCI reg immediate</div>
              <div>Subtract value from register with carry</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">MULT reg1 reg2</div>
              <div>Multiply two registers</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">MULTI reg immediate</div>
              <div>Multiply register by value</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">MULT_16 reg</div>
              <div>Multiply a 16 bit super register (currently fixed to be R1:R0) by a 8 bit register</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">JMP addr</div>
              <div>Unconditionally jump to address addr in program memory</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">JZ addr</div>
              <div>If zero flag is set, jump to address addr</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">JNZ addr</div>
              <div>If zero flag is not set, jump to address addr</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">CALL addr</div>
              <div>Push the current program counter onto the stack and jump to address addr</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">RET</div>
              <div>Jump to the address on the top of the stack</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">PUSH reg</div>
              <div>Push the value of a register onto the stack</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">POP reg</div>
              <div>Pop the value from the top of the stack into a register</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">HALT</div>
              <div>Halt execution</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">XXX</div>
              <div>Immediate value (integer)</div>
            </div>
          </div>
        </Card>
      </div>
    </div>
  );
}
