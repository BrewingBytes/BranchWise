import { IGitUser } from "./gitUser";

export enum GitCommitAuthorType {
    AUTHOR = "author",
    COMMITTER = "committer",
    TAGGER = "tagger",
}

export interface IGitAuthor {
    user: IGitUser;
    date_seconds: number;
    timezone: string;
    type_: GitCommitAuthorType;
}
