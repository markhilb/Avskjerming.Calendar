import { createAction, props } from '@ngrx/store';
import { ChangePassword, Login } from 'src/generated/openapi';

export const login = createAction('[Auth] Login', props<{ login: Login }>());
export const loginOk = createAction('[Auth] Login Ok');
export const loginFailed = createAction('[Auth] Login Failed');

export const logout = createAction('[Auth] Logout');
export const logoutOk = createAction('[Auth] Logout Ok');

export const loggedIn = createAction('[Auth] Logged in');
export const loggedInOk = createAction('[Auth] Logged in Ok', props<{ loggedIn: boolean }>());

export const changePassword = createAction('[Auth] Change password', props<{ changePassword: ChangePassword }>());
export const changePasswordOk = createAction('[Auth] Change password Ok');
export const changePasswordFailed = createAction('[Auth] Change password Failed');
