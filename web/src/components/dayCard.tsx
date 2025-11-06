import type { Task } from "@/http/types/tasks";
import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "./ui/dialog";
import { Calendar } from "./ui/calendar";
import z from "zod";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { Form } from "./ui/form";
import { useState } from "react";
import { Input } from "./ui/input";
import { Button } from "./ui/button";
import { ArrowLeft, ArrowRight } from "lucide-react";

interface DayCardProps {
  allTasks: Task[];
  day: number;
  dayLetter: string;
  visibleTasks: Task[];
  remainingCount: number;
}

const sendNewTask = z.object({
  content: z.string().min(3, { message: "min 3 characters" }),
  task_date: z.date(),
});

type sendTaskSchemaData = z.infer<typeof sendNewTask>;

export const DayCard = ({
  allTasks,
  day,
  dayLetter,
  visibleTasks,
  remainingCount,
}: DayCardProps) => {
  const form = useForm<sendTaskSchemaData>({
    resolver: zodResolver(sendNewTask),
    defaultValues: {
      content: "",
      task_date: new Date(),
    },
  });

  const [date, setDate] = useState<Date | undefined>(new Date());
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
      <DialogContent className="max-w-xl md:max-w-4xl max-h-[85vh] overflow-y-auto p-6">
        <DialogHeader className="text-center pb-4">
          <DialogTitle className="text-xl font-semibold">
            Gerenciar tarefas
          </DialogTitle>
          <DialogDescription className="text-sm text-muted-foreground">
            Escolha uma data no calendário ou crie uma nova tarefa.
          </DialogDescription>
        </DialogHeader>

        <Form {...form}>
          <form className="grid md:grid-cols-2 gap-6">
            <Calendar
              mode="single"
              selected={date}
              onSelect={setDate}
              className="rounded-lg border"
            />

            <div className="flex flex-col gap-4">
              <Input
                placeholder="Nova tarefa..."
                {...form.register("content")}
              />
              <Button className="w-full">Adicionar</Button>
            </div>
          </form>
        </Form>
        <div className="w-full flex justify-center mt-6">
          <div className="flex items-center justify-between w-full h-[200px] border rounded-xl p-4 shadow-sm">
            <Button
              variant="secondary"
              className="p-2 hover:bg-muted rounded-lg transition"
            >
              <ArrowLeft className="h-6 w-6" />
            </Button>

            <div className="flex items-center justify-center text-center text-sm text-muted-foreground cursor-pointer">
              {/* Put content inside here later (task list, message, etc.) */}
              No tasks selected
            </div>

            <Button
              variant="secondary"
              className="p-2 hover:bg-muted rounded-lg transition cursor-pointer"
            >
              <ArrowRight className="h-6 w-6" />
            </Button>
          </div>
        </div>
        <div className="w-full flex justify-center mt-6">
          <div className="flex items-center justify-between w-full h-[250px] border rounded-xl p-4 shadow-sm">
            <button className="p-2 hover:bg-muted rounded-lg transition">
              <ArrowLeft className="h-6 w-6" />
            </button>

            <div className="flex items-center justify-center text-center text-sm text-muted-foreground">
              {/* Put content inside here later (task list, message, etc.) */}
              No tasks selected
            </div>

            <button className="p-2 hover:bg-muted rounded-lg transition">
              <ArrowRight className="h-6 w-6" />
            </button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
};
