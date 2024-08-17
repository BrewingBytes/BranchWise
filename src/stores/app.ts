import { defineStore } from "pinia";

export const useAppStore = defineStore('app', {
    state: () => ({
        title: "BranchWise",
    }),
    actions: {
        setTitle(title: string) {
            this.title = title;
        }
    }
});
