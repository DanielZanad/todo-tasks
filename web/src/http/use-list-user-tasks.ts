import { useQuery } from "@tanstack/react-query";
import Cookies from "js-cookie";
import type { ListUserTasksResponse } from "./types/list-user-tasks-response";
import type { Task } from "./types/tasks";

export function useListUserTasks() {
  return useQuery({
    queryKey: ["list-user-tasks"],
    queryFn: async () => {
      let token = Cookies.get("token");
      if (!token) {
        console.log("user");
        return;
      }

      const response = await fetch("http://localhost:3000/tasks/list", {
        method: "GET",
        headers: {
          authorization: token,
        },
      });

      const result: ListUserTasksResponse = await response.json();

      const tasks: Array<Task> = result.tasks.map((tasks) => {
        let color: string = "";
        let task_date = new Date(tasks.task_date);
        let created_at = new Date(tasks.created_at);

        if (tasks.status === "ToStart") {
          color = "bg-red-500";
        } else if (tasks.status === "Started") {
          color = "bg-yellow-500";
        } else if (tasks.status === "Completed") {
          color = "bg-green-500";
        }
        console.log(task_date);
        console.log("tasks:", tasks.task_date);
        return {
          id: tasks.id,
          user_id: tasks.user_id,
          status: tasks.status,
          color,
          task_date,
          created_at,
        };
      });

      return tasks;
    },
  });
}
