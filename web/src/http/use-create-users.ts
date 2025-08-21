import { useMutation, useQueryClient } from "@tanstack/react-query";
import type { CreateUserRequest } from "./types/create-user-request";
import type { CreateUserResponse } from "./types/create-user-response";

export function useCreateUser() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (data: CreateUserRequest) => {
      console.log("", data.file_key?.type);
      const response = await fetch("http://localhost:3000/users", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ ...data, file_key: data.file_key?.name }),
      });

      const result = await response.json();

      // Make the
      return result;
    },
    async onSuccess(data: CreateUserResponse, variables, context) {
      // const response = await fetch(data.url, {
      //   method: "PUT",
      //   headers: {
      //     "Content-Type": variables.file_key?.type ?? "image/jpeg",
      //   },
      //   body: variables.file_key,
      // });
      // console.log("response", response);
    },
  });
}
