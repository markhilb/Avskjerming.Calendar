export interface AuthState {
  loggedIn?: boolean;
}

export const initialAuthState: AuthState = {
  loggedIn: undefined,
};
