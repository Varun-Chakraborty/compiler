import { Textarea } from "./ui/textarea";
import { Label } from "./ui/label";

interface AssemblyEditorProps {
  code: string;
  onChange: (code: string) => void;
  currentLine?: number;
}

export function AssemblyEditor({ code, onChange, currentLine }: AssemblyEditorProps) {
  const lines = code.split('\n');

  return (
    <div className="flex flex-col h-full">
      <Label className="mb-2">Assembly Code</Label>
      <div className="relative flex-1 flex border rounded-lg overflow-hidden bg-zinc-950">
        {/* Line numbers */}
        <div className="flex flex-col bg-zinc-900 px-3 py-3 text-right select-none">
          {lines.map((_, index) => (
            <div
              key={index}
              className={`text-sm font-mono leading-6 transition-colors ${currentLine === index
                  ? 'text-amber-400'
                  : 'text-zinc-500'
                }`}
            >
              {(index + 1).toString().padStart(2, '0')}
            </div>
          ))}
        </div>

        {/* Code editor */}
        <Textarea
          value={code}
          onChange={(e) => onChange(e.target.value)}
          className="flex-1 font-mono text-sm border-0 rounded-none bg-transparent text-zinc-100 leading-6 resize-none focus-visible:ring-0"
          placeholder="Enter assembly code here...
; Example:
MOVEI R0, 16
MOVEI R1, 6
ADD R0, R1
MOVEM R0, 0
HALT"
          spellCheck={false}
        />
      </div>

      <div className="mt-2 text-xs text-zinc-500">
        Supported instructions: MOV, ADD, SUB, INC, DEC, LOAD, STORE, HLT
      </div>
    </div>
  );
}
