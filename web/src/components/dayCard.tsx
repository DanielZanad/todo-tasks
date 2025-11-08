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
import { Form, FormControl, FormField, FormItem, FormLabel } from "./ui/form";
import { useState } from "react";
import { Input } from "./ui/input";
import { Button } from "./ui/button";
import { ArrowLeft, ArrowRight } from "lucide-react";
import { TaskCard } from "./taskCard";
import { useCreateTask } from "@/http/use-create-task";

interface DayCardProps {
  allTasks: Task[];
  day: number;
  month: string;
  year: string;
  dayLetter: string;
  visibleTasks: Task[];
  remainingCount: number;
}

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

const sendNewTask = z.object({
  content: z.string().min(3, { message: "min 3 characters" }),
});

type sendTaskSchemaData = z.infer<typeof sendNewTask>;

export const DayCard = ({
  allTasks,
  day,
  dayLetter,
  visibleTasks,
  remainingCount,
  month,
  year,
}: DayCardProps) => {
  const { mutateAsync: createTask } = useCreateTask();
  const form = useForm<sendTaskSchemaData>({
    resolver: zodResolver(sendNewTask),
    defaultValues: {
      content: "",
    },
  });

  async function handleCreateTask(task: sendTaskSchemaData) {
    const date = new Date(Number(year), monthMap[month.toLowerCase()], day);
    const result = await createTask({
      content: task.content,
      task_date: date,
    });
  }

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
          <form
            onSubmit={form.handleSubmit(handleCreateTask)}
            className="flex w-full flex-col gap-6"
          >
            <div className="flex flex-col gap-4">
              <FormField
                control={form.control}
                name="content"
                render={({ field }) => {
                  return (
                    <FormItem>
                      <FormControl>
                        <Input placeholder="Nova tarefa..." {...field} />
                      </FormControl>
                    </FormItem>
                  );
                }}
              ></FormField>
              <Button className="w-full">Adicionar</Button>
            </div>
          </form>
        </Form>
        {allTasks.map((task) => {
          return (
            <TaskCard
              task={task.content}
              color={task.color}
              day={day}
              month={month}
              year={year}
            />
          );
        })}
      </DialogContent>
    </Dialog>
  );
};
