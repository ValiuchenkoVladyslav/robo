"use client";

import Link from 'next/link';
import { useSigninMutation } from './_api';
import { useCurrentUser } from "~/current-user";
import { redirect } from 'next/navigation';
import { useForm } from 'react-hook-form';
import { LoginRequest } from '~/types/gen';

export default function RegisterPage() {
  const signinMutation = useSigninMutation();
  const currentUser = useCurrentUser();

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<LoginRequest>();

  function onSubmit(data: LoginRequest) {
    signinMutation.mutate(data);
  }

  if (signinMutation.isSuccess) {
    currentUser.set(signinMutation.data);
    redirect("/");
  }

  return (
    <div className="flex justify-center pt-32">
      <form
        onSubmit={handleSubmit(onSubmit)}
        className="bg-[#0f0f0f] px-6 py-4 rounded-xl w-[max(22vw,330px)]"
      >
        <h2 className="text-sm text-center">Sign up to Robo</h2>

        <section className="mt-4 flex flex-col gap-2 *:bg-white/10 *:rounded-lg *:px-4 *:py-2">
          <input
            {...register("email", { required: true, minLength: 3, maxLength: 99 })}
            aria-invalid={!!errors.email}
            type="email"
            placeholder="you@mail.com"
            className="aria-[invalid=true]:bg-red-500"
          />

          <input
            {...register("password", { required: true, minLength: 8, maxLength: 99 })}
            aria-invalid={!!errors.password}
            type="password"
            placeholder="passw0rd"
            className="aria-[invalid=true]:bg-red-500"
          />
        </section>

        <section className="flex justify-between items-center mt-6">
          <Link href="/register" className="hover:underline">
            Dont have an account?
          </Link>

          <button
            type="submit"
            className="rounded-lg px-4 py-2 bg-blue-500"
          >
            Sign in
          </button>
        </section>
      </form>
    </div>
  );
}
