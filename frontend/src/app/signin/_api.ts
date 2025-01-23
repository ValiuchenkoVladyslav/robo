import { useMutation } from "@tanstack/react-query";
import { AuthUser, LoginRequest } from "~/types/gen";
import { API_URL } from "~/utils";

export function useSigninMutation() {
  return useMutation<AuthUser, unknown, LoginRequest>({
    async mutationFn(req) {
      const res = await fetch(API_URL + "/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(req),
      });

      return res.json();
    },
  });
}
