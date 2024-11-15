import { IGitAuthor } from "@/types/gitAuthor";

export interface IGitCommit {
    tree_hash: string;
    parent_hashes: string[];
    author: IGitAuthor;
    committer: IGitAuthor;
    hash: string;
    message: string;
}

export function getHash(commit: IGitCommit): string {
    return commit.hash.substring(0, 7);
}
