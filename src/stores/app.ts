import { defineStore } from "pinia";
import { DEFAULT_USER } from "../types/user";
import { IGitProject } from "../types/gitProject";
import { invoke } from "@tauri-apps/api/core";

export const useAppStore = defineStore('app', {
    state: () => (
        {
            title: "BranchWise",
            user: DEFAULT_USER,
            projects: [] as IGitProject[],
            isNavbarOpen: false,
            selectedProject: null as IGitProject | null,
            appVersion: "0.0.5",
        }),
    getters: {
        getProjects(): IGitProject[] {
            return this.projects;
        },
        getSelectedProject(): IGitProject | null {
            return this.selectedProject;
        },
        getAppVersion(): string {
            return `v${this.appVersion}`;
        }
    },
    actions: {
        setTitle(title: string) {
            this.title = title;
        },
        addProject(git: IGitProject) {
            this.projects.push(git);
        },
        setProjects(projects: IGitProject[]) {
            this.projects = projects;
        },
        removeProject(git: IGitProject | null = null) {
            if (git === null && this.selectedProject !== null) {
                git = this.selectedProject;
            } else if (this.selectedProject === null) {
                return;
            }

            const index = this.projects.indexOf(git as IGitProject);
            if (index > -1) {
                this.projects.splice(index, 1);
            }
        },
        toggleNavbar() {
            this.isNavbarOpen = !this.isNavbarOpen;
        },
        setCurrentProject(git: IGitProject | null) {
            this.selectedProject = git;
            invoke("set_current_project", { project: git });
        },
        updateProject(git: IGitProject) {
            this.removeProject(this.selectedProject);
            this.addProject(git);
            this.setCurrentProject(git);
        }
    }
});
