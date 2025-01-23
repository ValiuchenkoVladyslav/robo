import { create } from 'zustand';
import { AuthUser, PublicUser } from '~/types/gen';

type CurrentUserStore = {
  user?: PublicUser;
  set: (user: AuthUser) => void;
}

export const useCurrentUser = create<CurrentUserStore>()((set) => ({
  user: (() => {
    if (typeof window !== 'undefined') {
      const user = localStorage.getItem('USER');

      if (user) {
        return JSON.parse(user);
      }
    }

    return undefined;
  })(),

  set(user) {
    localStorage.setItem("JWT_TOKEN", user.token);
    localStorage.setItem("USER", JSON.stringify(user.public_user));

    set({ user: user.public_user });
  },
}));
