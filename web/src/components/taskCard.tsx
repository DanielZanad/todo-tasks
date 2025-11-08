import { ArrowLeft, ArrowRight } from "lucide-react";
import { Button } from "./ui/button";
import { twMerge } from "tailwind-merge";
import { useUpdateTaskStatus } from "@/http/use-update-task-status";

interface TaskCardProps {
  id: string;
  task: string;
  color: string;
}

const colorVariants = {
  "bg-red-500": "border-red-500",
  "bg-blue-500": "border-blue-500",
  "bg-green-500": "border-green-500",
  "bg-yellow-500": "border-yellow-500",
};

export const TaskCard = ({ task, color, id }: TaskCardProps) => {
  const { mutateAsync: updateTaskStatus } = useUpdateTaskStatus();

  const borderColorClass =
    colorVariants[color as keyof typeof colorVariants] || "border-gray-200";

  async function handleNextStatus() {
    await updateTaskStatus({
      action: "next",
      task_id: id,
    });
  }

  async function handlePreviousStatus() {
    await updateTaskStatus({
      action: "previous",
      task_id: id,
    });
  }

  return (
    <div className="w-full flex justify-center mt-6">
      <div
        className={twMerge(
          `flex items-center justify-between w-full h-[200px] rounded-xl p-4 shadow-sm border`,
          borderColorClass
        )}
      >
        <Button
          variant="secondary"
          className="p-2 hover:bg-muted rounded-lg transition cursor-pointer"
          onClick={handlePreviousStatus}
        >
          <ArrowLeft className="h-6 w-6" />
        </Button>

        <div className="flex items-center justify-center text-center text-sm text-muted-foreground cursor-pointer">
          {task}
        </div>

        <Button
          variant="secondary"
          className="p-2 hover:bg-muted rounded-lg transition cursor-pointer"
          onClick={handleNextStatus}
        >
          <ArrowRight className="h-6 w-6" />
        </Button>
      </div>
    </div>
  );
};
