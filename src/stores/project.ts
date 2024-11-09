import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";
import { IGitBranch } from "../types/gitBranch";
import { IGitProject } from "../types/gitProject";

interface IProjectState {
    projects: IGitProject[];
    selectedProject: IGitProject | null;
    branch: IGitBranch | null;
}

export const useProjectStore = defineStore('project', {
    state: (): IProjectState => (
        {
            projects: [] as IGitProject[],
            selectedProject: null as IGitProject | null,
            branch: null as IGitBranch | null,
        }),
    getters: {
        getProjects(): IGitProject[] {
            return this.projects;
        },
        getSelectedProject(): IGitProject | null {
            return this.selectedProject;
        },
        getBranch(): IGitBranch | null {
            return this.branch;
        },
        getBranches(): IGitBranch[] {
            if (this.selectedProject === null) {
                return [];
            }

            return this.selectedProject.localBranches.concat(this.selectedProject.remoteBranches).concat(this.selectedProject.tags);
        }
    },
    actions: {
        setSelectedProject(project: IGitProject) {
            this.selectedProject = project;
        },
        setBranch(branch: IGitBranch) {
            if (this.selectedProject === null) {
                return;
            }

            const branchObj = this.getBranches.find(b => b.name === branch.name);
            if (branchObj === undefined) {
                return;
            }

            this.branch = branchObj;
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
        updateProject(git: IGitProject) {
            this.removeProject(this.selectedProject);
            this.addProject(git);
            this.setCurrentProject(git);
        },
        setCurrentProject(git: IGitProject | null) {
            this.selectedProject = git;
            invoke("set_current_project", { project: git });
        },
    }
});
