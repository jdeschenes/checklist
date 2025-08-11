import * as React from 'react'
import { format } from 'date-fns'
import { CalendarIcon } from 'lucide-react'

import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import { Calendar } from '@/components/ui/calendar'
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from '@/components/ui/popover'

interface DatePickerProps {
    date?: Date
    onDateChange?: (date: Date | undefined) => void
}

export function DatePicker({ date, onDateChange }: DatePickerProps) {
    return (
        <Popover>
            <PopoverTrigger
                className={cn(
                    'inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*="size-"])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]',
                    'border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50',
                    'h-9 px-4 py-2',
                    'w-[240px] justify-start text-left font-normal',
                    !date && 'text-muted-foreground'
                )}
            >
                <CalendarIcon />
                {date ? format(date, 'PPP') : <span>Pick a date</span>}
            </PopoverTrigger>
            <PopoverContent className="w-auto p-0" align="start">
                <Calendar
                    mode="single"
                    selected={date}
                    onSelect={onDateChange}
                    initialFocus
                    disabled={(date) =>
                        date < new Date(new Date().setHours(0, 0, 0, 0))
                    }
                />
            </PopoverContent>
        </Popover>
    )
}
