export interface IUser {
    name: string;
    avatar: string;
}

export const DEFAULT_USER: IUser = {
    name: "Guest",
    avatar: "https://cdn.vuetifyjs.com/images/logos/logo.svg",
};
