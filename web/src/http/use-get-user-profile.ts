import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import Cookies from "js-cookie";
import type { GetUserProfileRequest } from "./types/get-user-profile-request";

export function useGetUserProfile() {
  const queryClient = useQueryClient();
  return useQuery({
    queryKey: ["user-profile"],
    queryFn: async () => {
      let token = Cookies.get("token");
      if (!token) {
        console.log("user");
        return;
      }

      const response = await fetch("http://localhost:3000/users/profile", {
        method: "GET",
        headers: {
          authorization: token,
        },
      });

      const x = await response.json();

      return x;
    },
  });
}
