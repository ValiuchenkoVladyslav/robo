<script lang="ts">
  import { goto } from "$app/navigation";
  import "../app.css";

	let { children } = $props();

  $effect(() => {
    if (
      location.pathname != "/login" &&
      location.pathname != "/register" &&
      !localStorage.getItem("JWT_TOKEN")
    ) {
      goto("/login");
    }
  });
</script>

{@render children()}
