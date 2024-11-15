import { IGitBranch } from "@/types/gitBranch";
import { GitHead } from "@/types/gitHead";
import { GitProjectState } from "@/types/gitProjectState";

export interface IGitProject {
    directory: string,
    state: GitProjectState,
    head: GitHead,
    localBranches: IGitBranch[],
    remotes: string[],
    remoteBranches: IGitBranch[],
    tags: IGitBranch[],
}

export const DEFAULT_GIT_PROJECT: IGitProject = {
	directory: "DEFAULT",
	state: GitProjectState.INVALID,
	head: {},
	localBranches: [],
	remotes: [],
	remoteBranches: [],
	tags: []
};
