import type { Task } from "@/http/types/tasks";
import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
  DialogClose,
} from "./ui/dialog";
import { Label } from "./ui/label";
import { Input } from "./ui/input";
import { Button } from "./ui/button";

interface DayCardProps {
  allTasks: Task[];
  day: number;
  dayLetter: string;
  visibleTasks: Task[];
  remainingCount: number;
}

export const DayCard = ({
  allTasks,
  day,
  dayLetter,
  visibleTasks,
  remainingCount,
}: DayCardProps) => {
  return (
    <Dialog>
      <DialogTrigger className="flex flex-col gap-2 w-full h-36 p-2 md:p-4 border cursor-pointer hover:border-accent  shadow-lg rounded-xl">
        <div className="flex flex-row justify-between items-center">
          <span className="text-lg font-bold">{day}</span>
          <span className="text-sm font-semibold text-gray-400">
            {dayLetter}
          </span>
        </div>

        <div className="grid grid-cols-3 md:grid-cols-5 h-full gap-1 items-center">
          {visibleTasks.map((event) => (
            <div key={event.id} className="flex items-center gap-1">
              <div className={`w-3 h-3 rounded-full ${event.color}`}></div>
            </div>
          ))}

          {remainingCount > 0 && (
            <div className="text-xs font-semibold text-gray-600">
              +{remainingCount}
            </div>
          )}
        </div>
      </DialogTrigger>
      <DialogContent className="sm:max-w-[720px] md:max-w-[1080px] sm:max-h-[620px] md:max-h-[920px] h-6/12">
        <DialogHeader>
          <DialogTitle>Edit your tasks</DialogTitle>
          <DialogDescription>
            Edit by clicking in the arrows, or create a new task
          </DialogDescription>
          <form action=""></form>
        </DialogHeader>
      </DialogContent>
    </Dialog>
  );
};
