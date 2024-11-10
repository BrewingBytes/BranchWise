import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";
import { BranchType, IGitBranch } from "../types/gitBranch";
import { IGitProject } from "../types/gitProject";
import { useDialogStore } from "./dialogs";

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
        async fetchCommitHistory (length: number = 10) {
            let hash = "";

            if (this.branch !== null) {
                hash = this.branch.commit.trim();
            }

            try {
            await invoke("get_commit_history", { project: this.selectedProject, hash, length});
            } catch (error) {
                useDialogStore().openSnackbar({text: error as string, color: "error"});
            }
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

            // invoke("set_current_branch", { branch: branch });
            this.fetchCommitHistory();
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
        setCurrentProject(project: IGitProject | null) {
            this.selectedProject = project;
            invoke("set_current_project", { project: project });

            if (project === null) {
                this.branch = null;
                return;
            }

            if (project.head.Branch !== undefined) {
                const name = project.head.Branch[1];

                switch (project.head.Branch[0].toLowerCase() as BranchType) {
                    case BranchType.HEADS:
                        this.branch = project.localBranches.find(b => b.name === name) || null;
                        break;
                    case BranchType.REMOTES:
                        this.branch = project.remoteBranches.find(b => b.name === name) || null;
                        break;
                    case BranchType.TAGS:
                        this.branch = project.tags.find(b => b.name === name) || null;
                        break;
                }
            }

            this.fetchCommitHistory();
        },
    }
});
