import { Button } from './ui/button';
import { Play, Pause, SkipForward, RotateCcw, Upload } from 'lucide-react';
import { Card } from './ui/card';
import { Slider } from './ui/slider';
import { Label } from './ui/label';

interface ControlPanelProps {
  onStep: () => void;
  onRun: () => void;
  onPause: () => void;
  onReset: () => void;
  onLoad: () => void;
  isRunning: boolean;
  isHalted: boolean;
  executionSpeed: number;
  onSpeedChange: (speed: number) => void;
}

export function ControlPanel({
  onStep,
  onRun,
  onPause,
  onReset,
  onLoad,
  isRunning,
  isHalted,
  executionSpeed,
  onSpeedChange,
}: ControlPanelProps) {
  return (
    <Card className="p-4 bg-zinc-900 border-zinc-800">
      <div className="flex flex-col gap-4">
        {/* Main controls */}
        <div className="flex gap-2">
          <Button
            onClick={onLoad}
            variant="outline"
            size="sm"
            className="flex-1"
            disabled={isRunning}
          >
            <Upload className="w-4 h-4 mr-2" />
            Load Program
          </Button>
          
          <Button
            onClick={onReset}
            variant="outline"
            size="sm"
            disabled={isRunning}
          >
            <RotateCcw className="w-4 h-4" />
          </Button>
        </div>

        <div className="flex gap-2">
          <Button
            onClick={onStep}
            variant="outline"
            size="sm"
            className="flex-1"
            disabled={isRunning || isHalted}
          >
            <SkipForward className="w-4 h-4 mr-2" />
            Step
          </Button>
          
          {!isRunning ? (
            <Button
              onClick={onRun}
              size="sm"
              className="flex-1"
              disabled={isHalted}
            >
              <Play className="w-4 h-4 mr-2" />
              Run
            </Button>
          ) : (
            <Button
              onClick={onPause}
              variant="destructive"
              size="sm"
              className="flex-1"
            >
              <Pause className="w-4 h-4 mr-2" />
              Pause
            </Button>
          )}
        </div>

        {/* Speed control */}
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <Label className="text-xs text-zinc-400">Execution Speed</Label>
            <span className="text-xs text-zinc-500 font-mono">
              {executionSpeed === 0 ? 'Max' : `${1000 / executionSpeed} Hz`}
            </span>
          </div>
          <Slider
            value={[executionSpeed]}
            onValueChange={([value]) => onSpeedChange(value)}
            min={0}
            max={2000}
            step={100}
            className="w-full"
          />
        </div>

        {/* Status */}
        <div className="pt-2 border-t border-zinc-800">
          <div className="flex items-center justify-between text-xs">
            <span className="text-zinc-500">Status:</span>
            <span className={`font-mono ${
              isHalted ? 'text-red-400' : isRunning ? 'text-green-400' : 'text-amber-400'
            }`}>
              {isHalted ? 'HALTED' : isRunning ? 'RUNNING' : 'READY'}
            </span>
          </div>
        </div>
      </div>
    </Card>
  );
}
