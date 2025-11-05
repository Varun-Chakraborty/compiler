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
              <div className="font-mono text-zinc-300 mb-1">MOV dest src</div>
              <div>Move value to register</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">ADD dest src</div>
              <div>Add src to dest</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">SUB dest src</div>
              <div>Subtract src from dest</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">INC reg / DEC reg</div>
              <div>Increment/decrement register</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">LOAD reg addr</div>
              <div>Load from memory to register</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">STORE reg addr</div>
              <div>Store register to memory</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">HLT</div>
              <div>Halt execution</div>
            </div>
            <div>
              <div className="font-mono text-zinc-300 mb-1">#XX</div>
              <div>Immediate value (hex)</div>
            </div>
          </div>
        </Card>
      </div>
    </div>
  );
}
