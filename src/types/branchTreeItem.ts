import { IGitBranch } from "@/types/gitBranch";

export interface IBranchTreeItem {
    id: number;
    title: string;
    branch?: IGitBranch;
    children?: IBranchTreeItem[];
    customIcon?: string;
  }
