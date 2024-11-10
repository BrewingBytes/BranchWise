import { IGitAuthor } from "./gitAuthor";

export interface IGitCommit {
    tree_hash: string;
    parent_hashes: string[];
    author: IGitAuthor;
    committer: IGitAuthor;
    message: string;
}
