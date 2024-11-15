export interface IGitUser {
    name: string;
    email: string;
}

export const NO_USER: IGitUser = {
    name: "",
    email: "",
};
