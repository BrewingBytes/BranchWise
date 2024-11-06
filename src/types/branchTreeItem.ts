export interface IBranchTreeItem {
    id: number;
    title: string;
    children?: IBranchTreeItem[];
    customIcon?: string;
  }
