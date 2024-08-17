import { defineStore } from "pinia";
import { DEFAULT_USER } from "../types/user";

export const useAppStore = defineStore('app', {
    state: () => ({
        title: "BranchWise",
        user: DEFAULT_USER,
    }),
    actions: {
        setTitle(title: string) {
            this.title = title;
        }
    }
});
