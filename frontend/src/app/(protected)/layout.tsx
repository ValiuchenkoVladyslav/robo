"use client";

import { useEffect } from "react";
import { useRouter } from 'next/navigation';

export default function AuthProtectedLayout({ children }: React.PropsWithChildren) {
  const router = useRouter();

  useEffect(() => {
    if (!localStorage.getItem("JWT_TOKEN")) {
      router.push("/signin");
      router.refresh();
    }
  }, [router]);

  return children;
}
