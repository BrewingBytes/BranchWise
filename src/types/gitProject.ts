import { IGitBranch } from "./gitBranch";
import { GitProjectState } from "./gitProjectState";

export enum BranchType {
    LOCAL = "local",
    REMOTE = "remote",
    TAGS = "tags",
}

export interface IGitProject {
    directory: string,
    state: GitProjectState,
    localBranches: IGitBranch[],
    remotes: string[],
    remoteBranches: IGitBranch[],
    tags: IGitBranch[],
}

export const DEFAULT_GIT_PROJECT: IGitProject = {
    directory: "DEFAULT",
    state: GitProjectState.INVALID,
    localBranches: [],
    remotes: [],
    remoteBranches: [],
    tags: []
}
