import { useMutation, useQueryClient } from "@tanstack/react-query";
import Cookies from "js-cookie";
import type { CreateSessionRequest } from "./types/create-session-request";
import type { CreateSessionResponse } from "./types/create-session-response";

export function useCreateSession() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (data: CreateSessionRequest) => {
      const response = await fetch("http://localhost:3000/session", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ ...data }),
      });

      const token = await response.json();

      return token;
    },
    async onSuccess(data: CreateSessionResponse, variables, context) {
      const { token } = data;

      if (token) {
        Cookies.set("token", token, {
          expires: 7,
          path: "/",
          secure: true,
          sameSite: "strict",
        });
      }
    },
    onError(error) {
      console.error("Login failed:", error.message);
    },
  });
}
