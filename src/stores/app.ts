import { defineStore } from "pinia";
import { DEFAULT_USER } from "../types/user";
import { IGitProject } from "../types/gitProject";

export const useAppStore = defineStore('app', {
    state: () => (
        {
        title: "BranchWise",
        user: DEFAULT_USER,
        projects: [] as IGitProject[],
    }),
    getters: {
        getProjects(): IGitProject[] {
            return this.projects;
        }
    },
    actions: {
        setTitle(title: string) {
            this.title = title;
        },
        addProject(git: IGitProject) {
            this.projects.push(git);
        }
    }
});
