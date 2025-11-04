import { Card } from './ui/card';
import { Tabs, TabsContent, TabsList, TabsTrigger } from './ui/tabs';
import { MemoryGrid } from './MemoryGrid';

interface MemoryViewerProps {
  programMemory: number[];
  dataMemory: number[];
  accessedAddress?: number;
  accessType?: 'read' | 'write';
  activeMemoryType?: 'program' | 'data';
}

export function MemoryViewer({
  programMemory,
  dataMemory,
  accessedAddress,
  accessType,
  activeMemoryType = 'data',
}: MemoryViewerProps) {
  return (
    <Card className="p-4 bg-zinc-900 border-zinc-800 h-full flex flex-col">
      <Tabs defaultValue={activeMemoryType} className="flex-1 flex flex-col">
        <TabsList className="grid w-full grid-cols-2 mb-4">
          <TabsTrigger value="program">Program Memory</TabsTrigger>
          <TabsTrigger value="data">Data Memory</TabsTrigger>
        </TabsList>
        
        <TabsContent value="program" className="flex-1 mt-0">
          <MemoryGrid
            memory={programMemory}
            accessedAddress={activeMemoryType === 'program' ? accessedAddress : undefined}
            accessType={activeMemoryType === 'program' ? accessType : undefined}
            title={`Program Memory (${programMemory.length} bytes)`}
          />
        </TabsContent>
        
        <TabsContent value="data" className="flex-1 mt-0">
          <MemoryGrid
            memory={dataMemory}
            accessedAddress={activeMemoryType === 'data' ? accessedAddress : undefined}
            accessType={activeMemoryType === 'data' ? accessType : undefined}
            title={`Data Memory (${dataMemory.length} bytes)`}
          />
        </TabsContent>
      </Tabs>
    </Card>
  );
}
