<script lang="ts">
  import { goto } from "$app/navigation";
  import type { AuthUser, LoginRequest } from "~/types/gen";
  import { setUser } from "$lib/state";
  import { API_URL } from "$lib/api";

  let email = $state("");
  let password = $state("");

  function login() {
    if (!email || !password) return;

    fetch(API_URL + "/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ email, password } satisfies LoginRequest),
    })
    .then((res) => res.json())
    .then((data: AuthUser) => {
      localStorage.setItem("JWT_TOKEN", data.token);

      setUser(data.public_user);

      goto("/");
    });
  }
</script>

<div class="flex justify-center pt-32">
  <article class="bg-[#0f0f0f] px-6 py-4 rounded-xl w-[max(22vw,330px)]">
    <h2 class="text-sm text-center">Sign in to your account</h2>

    <section class="mt-4 flex flex-col gap-2 *:bg-white/10 *:rounded-lg *:px-4 *:py-2">
      <input bind:value={email} type="email" placeholder="you@mail.com"/>
      <input bind:value={password} type="password" placeholder="passw0rd"/>
    </section>

    <section class="flex justify-between items-center mt-6">
      <a href="/register" class="hover:underline">
        Dont have an account?
      </a>

      <button
        class="rounded-lg px-4 py-2 bg-blue-500"
        onclick={login}
      >
        Sign in
      </button>
    </section>
  </article>
</div>
