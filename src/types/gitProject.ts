export enum GitProjectState {
    INVALID = "invalid",
    VALID = "valid",
}

export enum BranchType {
    LOCAL = "local",
    REMOTE = "remote",
    TAGS = "tags",
}

export interface IGitProject {
    directory: string,
    state: GitProjectState,
    localBranches: string[],
    remotes: string[],
    remoteBranches: string[],
    tags: string[],
}

export const DEFAULT_GIT_PROJECT: IGitProject = {
    directory: "DEFAULT",
    state: GitProjectState.INVALID,
    localBranches: [],
    remotes: [],
    remoteBranches: [],
    tags: []
}
