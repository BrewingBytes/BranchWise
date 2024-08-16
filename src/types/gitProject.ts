export enum GitProjectState {
    INVALID = "invalid",
    VALID = "valid",
}

export interface IGitProject {
    directory: string,
    state: GitProjectState,
    localBranches: string[],
    remoteBranches: string[]
}

export const DEFAULT_GIT_PROJECT: IGitProject = {
    directory: "",
    state: GitProjectState.INVALID,
    localBranches: [],
    remoteBranches: []
}
