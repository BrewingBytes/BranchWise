import { GitError } from "@/types/gitErrors";
import { defineStore } from "pinia";

interface IDialogState {
    snackbar: {
        show: boolean;
        text: string;
        color: string;
        timeout?: number;
    },
    confirmationDialog: {
        isOpen: boolean;
        title: string;
        message: string;
        onConfirm: () => void;
    },
	contextMenu: {
		isOpen: boolean;
		commitHash: string;
		position: {
			x: number;
			y: number;
		};
	}
}

export const useDialogStore = defineStore("dialog", {
	state: (): IDialogState => (
		{
			snackbar: {
				show: false,
				text: "",
				color: "",
				timeout: 5000
			},
			confirmationDialog: {
				isOpen: false,
				title: "",
				message: "",
				onConfirm: () => { return; }
			},
			contextMenu: {
				isOpen: false,
				commitHash: "",
				position: {
					x: 0,
					y: 0
				}
			}
		}),
	actions: {
		showError(error: unknown) {
			const text = GitError[error as keyof typeof GitError];
			console.error(error);

			if (text) {
			    this.openSnackbar({text: text, color: "error"});
			}
		},
		openSnackbar({text, color}: {text: string, color: string}) {
			this.snackbar.show = true;
			this.snackbar.text = text;
			this.snackbar.color = color;
		},
		closeSnackbar() {
			this.snackbar.show = false;
		},
		openConfirmationDialog({title, message, onConfirm}: {title: string, message: string, onConfirm: () => void}) {
			this.confirmationDialog.isOpen = true;
			this.confirmationDialog.title = title;
			this.confirmationDialog.message = message;
			this.confirmationDialog.onConfirm = onConfirm;
		},
		closeConfirmationDialog() {
			this.confirmationDialog.isOpen = false;
		},
		showContextMenu(hash: string, x: number, y: number) {
			this.contextMenu = {
				position: {
					x,
					y
				},
				commitHash: hash,
				isOpen: true
			};
		},
		closeContextMenu() {
			this.contextMenu.isOpen = false;
		}
	}
});
