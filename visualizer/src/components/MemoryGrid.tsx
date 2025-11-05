import { ScrollArea } from './ui/scroll-area';
import { motion } from 'framer-motion';
import { useState } from 'react';

interface MemoryGridProps {
  memory: number[];
  accessedAddress?: number;
  accessType?: 'read' | 'write';
  title?: string;
}

export function MemoryGrid({ memory, accessedAddress, accessType, title }: MemoryGridProps) {
  const [hoveredAddress, setHoveredAddress] = useState<number | null>(null);
  
  // Display memory in rows of 16 bytes
  const rows = Math.ceil(memory.length / 16);

  return (
    <div className="h-full flex flex-col">
      <div className="flex items-center justify-between mb-3 px-2">
        <h3 className="text-sm text-zinc-400">{title || `Memory (${memory.length} bytes)`}</h3>
        {hoveredAddress !== null && (
          <div className="text-xs text-zinc-500 font-mono">
            Address: 0x{hoveredAddress.toString(16).toUpperCase().padStart(4, '0')} 
            <span className="ml-2">Value: 0x{memory[hoveredAddress].toString(16).toUpperCase().padStart(2, '0')}</span>
          </div>
        )}
      </div>
      
      <ScrollArea className="flex-1 h-full">
        <div className="space-y-1">
          {Array.from({ length: rows }, (_, rowIndex) => {
            const startAddr = rowIndex * 16;
            
            return (
              <div key={rowIndex} className="flex items-center gap-2">
                {/* Address label */}
                <div className="text-xs text-zinc-600 font-mono w-8">
                  {startAddr.toString(16).toUpperCase().padStart(2, '0')}:
                </div>
                
                {/* Memory cells */}
                <div className="flex gap-1 flex-wrap">
                  {Array.from({ length: 16 }, (_, colIndex) => {
                    const addr = startAddr + colIndex;
                    if (addr >= memory.length) return null;
                    
                    const value = memory[addr];
                    const isAccessed = accessedAddress === addr;
                    const isNonZero = value !== 0;
                    
                    return (
                      <motion.div
                        key={addr}
                        className={`
                          w-8 h-8 flex items-center justify-center text-xs font-mono rounded border
                          transition-colors cursor-default
                          ${isAccessed && accessType === 'write'
                            ? 'bg-green-500/30 border-green-500/50 text-green-200'
                            : isAccessed && accessType === 'read'
                            ? 'bg-blue-500/30 border-blue-500/50 text-blue-200'
                            : isNonZero
                            ? 'bg-zinc-800 border-zinc-700 text-zinc-300'
                            : 'bg-zinc-950 border-zinc-800 text-zinc-600'
                          }
                        `}
                        animate={isAccessed ? {
                          scale: [1, 1.15, 1],
                          transition: { duration: 0.3 }
                        } : {}}
                        onMouseEnter={() => setHoveredAddress(addr)}
                        onMouseLeave={() => setHoveredAddress(null)}
                      >
                        {value.toString(16).toUpperCase().padStart(2, '0')}
                      </motion.div>
                    );
                  })}
                </div>
              </div>
            );
          })}
        </div>
      </ScrollArea>
      
      <div className="mt-3 flex gap-4 text-xs text-zinc-500">
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 rounded border border-green-500/50 bg-green-500/30"></div>
          <span>Write</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 rounded border border-blue-500/50 bg-blue-500/30"></div>
          <span>Read</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 rounded border border-zinc-700 bg-zinc-800"></div>
          <span>Non-zero</span>
        </div>
      </div>
    </div>
  );
}
