import * as React from "react";
import { Input } from "./input";
import { Button } from "./button";
import { cn } from "@/lib/utils";
import { RecurrenceInterval } from "@/api";

interface IntervalPickerProps {
  value: RecurrenceInterval;
  onChange: (interval: RecurrenceInterval) => void;
  className?: string;
}

type IntervalUnit = "days" | "weeks" | "months";

const presetIntervals: { label: string; interval: RecurrenceInterval }[] = [
  { label: "Daily", interval: { days: 1 } },
  { label: "Weekly", interval: { days: 7 } },
  { label: "Monthly", interval: { months: 1 } },
  { label: "Yearly", interval: { months: 12 } },
];

export function IntervalPicker({
  value,
  onChange,
  className,
}: IntervalPickerProps) {
  const [selectedUnit, setSelectedUnit] = React.useState<IntervalUnit>("days");
  const [customValue, setCustomValue] = React.useState<number>(1);
  const [isCustom, setIsCustom] = React.useState(false);
  const [userSelectedCustom, setUserSelectedCustom] = React.useState(false);

  React.useEffect(() => {
    if (value.months && value.months > 0) {
      setSelectedUnit("months");
      setCustomValue(value.months);
    } else if (value.days && value.days > 0) {
      // Check if days is divisible by 7 and not a preset to show as weeks
      if (value.days % 7 === 0 && value.days > 7) {
        setSelectedUnit("weeks");
        setCustomValue(value.days / 7);
      } else {
        setSelectedUnit("days");
        setCustomValue(value.days);
      }
    }

    const isPreset = presetIntervals.some(
      (preset) =>
        preset.interval.days === value.days &&
        preset.interval.months === value.months
    );
    // Only auto-set to preset if user hasn't explicitly chosen custom
    if (!userSelectedCustom) {
      setIsCustom(!isPreset);
    }
  }, [value, userSelectedCustom]);

  const handlePresetClick = (interval: RecurrenceInterval) => {
    setIsCustom(false);
    setUserSelectedCustom(false);
    onChange(interval);
  };

  const handleCustomValueChange = (newValue: number) => {
    setCustomValue(newValue);
    const newInterval: RecurrenceInterval = {};
    if (selectedUnit === "days") {
      newInterval.days = newValue;
    } else if (selectedUnit === "weeks") {
      newInterval.days = newValue * 7;
    } else {
      newInterval.months = newValue;
    }
    onChange(newInterval);
  };

  return (
    <div className={cn("space-y-4", className)}>
      <div>
        <label className="text-sm font-medium">Recurrence Interval</label>
        <div className="mt-2 grid grid-cols-2 gap-2 sm:grid-cols-4">
          {presetIntervals.map((preset) => (
            <Button
              key={preset.label}
              type="button"
              variant={
                !isCustom &&
                preset.interval.days === value.days &&
                preset.interval.months === value.months
                  ? "default"
                  : "outline"
              }
              size="sm"
              onClick={() => handlePresetClick(preset.interval)}
              className="text-xs"
            >
              {preset.label}
            </Button>
          ))}
        </div>
      </div>

      <div>
        <Button
          type="button"
          variant={isCustom ? "default" : "outline"}
          size="sm"
          onClick={() => {
            if (!isCustom) {
              setIsCustom(true);
              setUserSelectedCustom(true);
              const newInterval: RecurrenceInterval = {};
              if (selectedUnit === "days") {
                newInterval.days = customValue;
              } else if (selectedUnit === "weeks") {
                newInterval.days = customValue * 7;
              } else {
                newInterval.months = customValue;
              }
              onChange(newInterval);
            }
          }}
          className="mb-2"
        >
          Custom
        </Button>

        {isCustom && (
          <div className="flex items-center space-x-2">
            <label className="text-sm">Every</label>
            <Input
              type="number"
              min="1"
              max="365"
              value={customValue}
              onChange={(e) =>
                handleCustomValueChange(parseInt(e.target.value) || 1)
              }
              className="w-20"
            />
            <select
              value={selectedUnit}
              onChange={(e) => {
                const newUnit = e.target.value as IntervalUnit;
                setSelectedUnit(newUnit);
                const newInterval: RecurrenceInterval = {};
                if (newUnit === "days") {
                  newInterval.days = customValue;
                } else if (newUnit === "weeks") {
                  newInterval.days = customValue * 7;
                } else {
                  newInterval.months = customValue;
                }
                onChange(newInterval);
              }}
              className="rounded-md border border-input bg-background px-3 py-1 text-sm"
            >
              <option value="days">days</option>
              <option value="weeks">weeks</option>
              <option value="months">months</option>
            </select>
          </div>
        )}
      </div>
    </div>
  );
}
