import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";
import { BranchType, IGitBranch } from "@/types/gitBranch";
import { IGitCommit } from "@/types/gitCommit";
import { IGitProject } from "@/types/gitProject";
import { useDialogStore } from "@/stores/dialogs";
import { TauriCommands } from "@/types/tauri";

interface IProjectState {
    projects: IGitProject[];
    selectedProject: IGitProject | null;
    branch: IGitBranch | null;
    commit: IGitCommit | null;
    history: IGitCommit[];
}

export const useProjectStore = defineStore("project", {
	state: (): IProjectState => (
		{
			projects: [] as IGitProject[],
			selectedProject: null as IGitProject | null,
			branch: null as IGitBranch | null,
			commit: null as IGitCommit | null,
			history: [] as IGitCommit[],
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
		},
		getCommit(): IGitCommit | null {
			return this.commit;
		},
		getHistory(): IGitCommit[] {
			return this.history;
		}
	},
	actions: {
		async fetchCommitHistory (length = 30, fromHash = ""): number {
			let hash = "";

			if (this.branch !== null && fromHash === "") {
				hash = this.branch.commit.trim();
			} else {
				hash = fromHash;
			}

			try {
				const history: IGitCommit[] = await invoke(TauriCommands.GetCommitHistory, { project: this.selectedProject, hash, length});

				if (fromHash === "") {
					this.history = history;
				} else {
					this.history = this.history.concat(history.slice(1));
				}
			} catch (error) {
				useDialogStore().showError(error);

				return 0;
			}

			return history.length;
		},
		async setBranch(branch: IGitBranch | null) {
			if (branch === null) {
				this.branch = null;
				return;
			}

			if (this.selectedProject === null) {
				return;
			}

			const branchObj = this.getBranches.find(b => b.name === branch.name);
			if (branchObj === undefined) {
				return;
			}

			if (this.branch !== null) {
				if (this.branch.commit === branch.commit && this.branch.name === branch.name) {
					return;
				}
			}

			this.branch = branchObj;

			// invoke(TauriCommands.SetCurrentBranch, { branch: branch });
			await this.fetchCommitHistory();
			this.setCommit(branch.commit);
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
			if (this.selectedProject !== null && project !== null) {
				if (this.selectedProject.directory === project.directory) {
					return;
				}
			}

			this.selectedProject = project;
			invoke(TauriCommands.SetCurrentProject, { project: project });

			if (project === null) {
				this.setBranch(null);
				return;
			}

			if (project.head.Branch !== undefined) {
				const name = project.head.Branch[1];

				switch (project.head.Branch[0].toLowerCase() as BranchType) {
					case BranchType.HEADS:
						this.setBranch(project.localBranches.find(b => b.name === name) || null);
						break;
					case BranchType.REMOTES:
						this.setBranch(project.remoteBranches.find(b => b.name === name) || null);
						break;
					case BranchType.TAGS:
						this.setBranch(project.tags.find(b => b.name === name) || null);
						break;
				}
			}
		},
		setCommit(hash: string) {
			hash = hash.trim();
            
			if (this.commit !== null) {
				if (this.commit.hash === hash) {
					return;
				}
			}

			this.commit = this.history.find(c => c.hash === hash) ?? null;
		}
	}
});
