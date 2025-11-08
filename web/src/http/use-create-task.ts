import { useMutation, useQueryClient } from "@tanstack/react-query";
import Cookies from "js-cookie";
import type { CreateTaskRequest } from "./types/create-task-request";

export function useCreateTask() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (data: CreateTaskRequest) => {
      let token = Cookies.get("token");
      if (!token) {
        console.log("user");
        return;
      }
      await fetch("http://localhost:3000/tasks/save", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          authorization: token,
        },

        body: JSON.stringify({ ...data }),
      });

      return;
    },
    async onSuccess() {
      queryClient.invalidateQueries({ queryKey: ["list-user-tasks"] });
    },
    onError(error) {
      console.error("Error saving task:", error.message);
    },
  });
}
