import { IGitBranch } from "./gitBranch";

export interface IBranchTreeItem {
    id: number;
    title: string;
    branch?: IGitBranch;
    children?: IBranchTreeItem[];
    customIcon?: string;
  }
