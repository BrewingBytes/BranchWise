import { IGitAuthor, NO_AUTHOR } from "@/types/gitAuthor";

export interface IGitCommit {
    tree_hash: string;
    parent_hashes: string[];
    author: IGitAuthor;
    committer: IGitAuthor;
    hash: string;
    message: string;
}

export const NO_COMMIT: IGitCommit = {
	tree_hash: "",
	parent_hashes: [],
	author: NO_AUTHOR,
	committer: NO_AUTHOR,
	hash: "",
	message: "",
};

export function getHash(commit: IGitCommit | null): string {
	if (commit === null) {
		return "";
	}

	return commit.hash.substring(0, 7);
}
