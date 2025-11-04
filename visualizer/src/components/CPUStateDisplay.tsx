import type { CPUState } from '../types/cpu';
import { Card } from './ui/card';
import { motion } from 'framer-motion';

interface CPUStateDisplayProps {
  state: CPUState;
  changedRegisters: string[];
  changedFlags: string[];
}

export function CPUStateDisplay({ state, changedRegisters, changedFlags }: CPUStateDisplayProps) {
  const flags = [
    { key: 'zero', label: 'Zero' },
    { key: 'carry', label: 'Carry' },
    { key: 'sign', label: 'Sign' },
    { key: 'overflow', label: 'Overflow' },
  ] as const;

  return (
    <div className="space-y-4">
      {/* Registers */}
      <Card className="p-4 bg-zinc-900 border-zinc-800">
        <h3 className="text-sm mb-3 text-zinc-400">
          Registers ({state.registers.count})
        </h3>
        <div className="grid grid-cols-2 gap-3">
          {state.registers.regs.map((value, index) => {
            const regName = `R${index}`;
            const isChanged = changedRegisters.includes(regName);
            
            return (
              <motion.div
                key={index}
                className={`p-3 rounded-lg border transition-colors ${
                  isChanged
                    ? 'bg-amber-500/20 border-amber-500/50'
                    : 'bg-zinc-950 border-zinc-800'
                }`}
                animate={isChanged ? {
                  scale: [1, 1.05, 1],
                  transition: { duration: 0.3 }
                } : {}}
              >
                <div className="text-xs text-zinc-500 mb-1">{regName}</div>
                <div className="font-mono">
                  0x{value.toString(16).toUpperCase().padStart(2, '0')}
                  <span className="text-xs text-zinc-500 ml-2">({value})</span>
                </div>
              </motion.div>
            );
          })}
        </div>
      </Card>

      {/* Flags */}
      <Card className="p-4 bg-zinc-900 border-zinc-800">
        <h3 className="text-sm mb-3 text-zinc-400">Flags</h3>
        <div className="grid grid-cols-2 gap-3">
          {flags.map(({ key, label }) => {
            const isChanged = changedFlags.includes(key);
            const value = state.flags[key as keyof typeof state.flags];
            
            return (
              <motion.div
                key={key}
                className={`p-3 rounded-lg border transition-colors ${
                  isChanged
                    ? 'bg-amber-500/20 border-amber-500/50'
                    : 'bg-zinc-950 border-zinc-800'
                }`}
                animate={isChanged ? {
                  scale: [1, 1.05, 1],
                  transition: { duration: 0.3 }
                } : {}}
              >
                <div className="text-xs text-zinc-500 mb-1">{label}</div>
                <div className={`font-mono ${value ? 'text-green-400' : 'text-zinc-600'}`}>
                  {value ? '1' : '0'}
                </div>
              </motion.div>
            );
          })}
        </div>
      </Card>

      {/* Program Counter & Stack Pointer */}
      <Card className="p-4 bg-zinc-900 border-zinc-800">
        <h3 className="text-sm mb-3 text-zinc-400">Special Registers</h3>
        <div className="space-y-3">
          <div className="p-3 rounded-lg bg-zinc-950 border border-zinc-800">
            <div className="text-xs text-zinc-500 mb-1">Program Counter (PC)</div>
            <div className="font-mono">
              0x{state.program_counter.toString(16).toUpperCase().padStart(4, '0')}
              <span className="text-xs text-zinc-500 ml-2">({state.program_counter})</span>
            </div>
          </div>
          <div className="p-3 rounded-lg bg-zinc-950 border border-zinc-800">
            <div className="text-xs text-zinc-500 mb-1">Stack Pointer (SP)</div>
            <div className="font-mono">
              0x{state.stack_pointer.toString(16).toUpperCase().padStart(4, '0')}
              <span className="text-xs text-zinc-500 ml-2">({state.stack_pointer})</span>
            </div>
          </div>
        </div>
      </Card>
    </div>
  );
}
