import { IGitBranch } from "./gitBranch";
import { GitHead } from "./gitHead";
import { GitProjectState } from "./gitProjectState";

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
}
