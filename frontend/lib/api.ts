import { dev } from "$app/environment";

export const API_URL = `${dev ? "http://localhost:3000" : ""}/api`;
