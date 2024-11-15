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
			}
		}),
	actions: {
		showError(error: unknown) {
			const text = GitError[error as keyof typeof GitError];

			this.openSnackbar({text: text, color: "error"});
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
		}
	}
});
