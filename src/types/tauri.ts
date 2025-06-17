export enum TauriCommands {
    GetCommitHistory = "get_commit_history",
    GetDatabaseProjects = "get_database_projects",
    OpenGitProject = "open_git_project",
    RemoveDatabaseProject = "remove_database_project",
    SetCurrentBranch = "set_current_branch",
    SetCurrentProject = "set_current_project",
    CheckoutBranch = "checkout_branch",
    CheckoutCommit = "checkout_commit",
}

export enum TauriListen {
    ProjectUpdate = "project_update",
}
