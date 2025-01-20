import staticAdapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import("@sveltejs/kit").Config} */
export default {
  preprocess: vitePreprocess(),

  kit: {
    adapter: staticAdapter({ fallback: "index.html" }),

    files: {
      appTemplate: "frontend/app.html",
      lib: "frontend/lib",
      routes: "frontend/routes",
    },

    alias: {
      "~/types/gen": "./bindings",
    },
  },
};
