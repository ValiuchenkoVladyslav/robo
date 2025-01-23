// why not just scripts in package.json? we need command parallelism

import { $ } from "bun";

if (Bun.argv[2] === "build") {
  // generate types
  await $`cargo test`;

  await Promise.all([
    // build frontend
    $`cd frontend && bun run build`,

    // build backend
    $`cargo build --release`,
  ]);
} else if (Bun.argv[2] === "dev") {
  await Promise.all([
    // start frontend dev server
    $`cd frontend && bun run dev`,

    // run backend in watch mode
    $`cargo watch -c -w src -x test -x run`,
  ]);
} else {
  console.error("Unknown command");
}
