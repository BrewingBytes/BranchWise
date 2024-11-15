export enum BranchType {
    HEADS = "heads",
    REMOTES = "remotes",
    TAGS = "tags",
}

export interface IGitBranch {
    name: string,
    commit: string,
}
