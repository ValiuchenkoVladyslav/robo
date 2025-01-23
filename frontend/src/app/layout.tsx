import type { Metadata } from "next";
import RQProvider from "./rq-provider";

import "./globals.css";

export const metadata: Metadata = {
  title: "Robo",
  description: "Ollama chat app",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className="bg-black text-white min-h-screen flex flex-col">
        <RQProvider>
          {children}
        </RQProvider>
      </body>
    </html>
  );
}
