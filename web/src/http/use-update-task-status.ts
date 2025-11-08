import { useMutation, useQueryClient } from "@tanstack/react-query";
import Cookies from "js-cookie";
import type { UpdateTaskStatusRequest } from "./types/update-task-status-request";

export function useUpdateTaskStatus() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (data: UpdateTaskStatusRequest) => {
      let token = Cookies.get("token");
      if (!token) {
        console.log("user");
        return;
      }
      await fetch(
        `http://localhost:3000/tasks/update/${data.task_id}/${data.action}`,
        {
          method: "PUT",
          headers: {
            "Content-Type": "application/json",
            authorization: token,
          },
        }
      );

      return;
    },
    async onSuccess() {
      queryClient.invalidateQueries({ queryKey: ["list-user-tasks"] });
    },
    onError(error) {
      console.error("Error updating task:", error.message);
    },
  });
}
