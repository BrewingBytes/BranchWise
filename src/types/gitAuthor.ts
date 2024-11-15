import { IGitUser, NO_USER } from "@/types/gitUser";
import moment from "moment-timezone";

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

export const NO_AUTHOR: IGitAuthor = {
    user: NO_USER,
    date_seconds: 0,
    timezone: "",
    type_: GitCommitAuthorType.AUTHOR,
};

export function getAuthorDate(author: IGitAuthor | null, hour_format: boolean = false): string {
    if (author === null) {
        return "";
    }

    const timezone = author.timezone.substring(0, 3) + ':' + author.timezone.substring(3);
    const momentString = moment.unix(author.date_seconds).utcOffset(timezone);

    if (hour_format) {
        return momentString.format("YYYY-MM-DD HH:mm:ss");
    }

    return momentString.format("YYYY-MM-DD");
}
