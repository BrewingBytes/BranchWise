import { IGitBranch } from "./gitBranch";

export interface IDirectoryBranch extends IGitBranch {
  dir_name: string;
}

export interface IDirectory {
    name: string;
    children: IDirectory[];
    branches: IDirectoryBranch[];
  }
