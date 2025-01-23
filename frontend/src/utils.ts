const env = process.env.NODE_ENV;

export const API_URL = `${env === "production" ? "http://localhost:3000" : ""}/api`;

export function withAuth(headers?: HeadersInit) {
  return {
    ...headers,
    Authorization: `Bearer ${localStorage.getItem("JWT_TOKEN")}`,
  };
}
