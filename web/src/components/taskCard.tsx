import { ArrowLeft, ArrowRight } from "lucide-react";
import { Button } from "./ui/button";
import { twMerge } from "tailwind-merge";

interface TaskCardProps {
  id: string;
  task: string;
  color: string;
  day: number;
  month: string;
  year: string;
}

const colorVariants = {
  "bg-red-500": "border-red-500",
  "bg-blue-500": "border-blue-500",
  "bg-green-500": "border-green-500",
  "bg-yellow-500": "border-yellow-500",
};
const monthMap: Record<string, number> = {
  janeiro: 0,
  fevereiro: 1,
  março: 2,
  abril: 3,
  maio: 4,
  junho: 5,
  julho: 6,
  agosto: 7,
  setembro: 8,
  outubro: 9,
  novembro: 10,
  dezembro: 11,
};

export const TaskCard = ({
  task,
  color,
  day,
  month,
  year,
  id,
}: TaskCardProps) => {
  const borderColorClass =
    colorVariants[color as keyof typeof colorVariants] || "border-gray-200";

  const date = new Date(Number(year), monthMap[month.toLowerCase()], day);
  console.log(date);
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
        >
          <ArrowLeft className="h-6 w-6" />
        </Button>

        <div className="flex items-center justify-center text-center text-sm text-muted-foreground cursor-pointer">
          {task}
        </div>

        <Button
          variant="secondary"
          className="p-2 hover:bg-muted rounded-lg transition cursor-pointer"
        >
          <ArrowRight className="h-6 w-6" />
        </Button>
      </div>
    </div>
  );
};
