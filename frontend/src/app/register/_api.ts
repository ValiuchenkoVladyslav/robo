import { useMutation } from "@tanstack/react-query";
import { AuthUser, RegisterRequest } from "~/types/gen";
import { API_URL } from "~/utils";

export function useRegisterMutation() {
  return useMutation<AuthUser, unknown, RegisterRequest>({
    async mutationFn(req) {
      const res = await fetch(API_URL + "/register", {
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
