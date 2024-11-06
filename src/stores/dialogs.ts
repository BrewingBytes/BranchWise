import { defineStore } from "pinia";

interface IDialogState {
    confirmationDialog: {
        isOpen: boolean;
        title: string;
        message: string;
        onConfirm: () => void;
    }
}

export const useDialogStore = defineStore('dialog', {
    state: (): IDialogState => (
        {
            confirmationDialog: {
                isOpen: false,
                title: "",
                message: "",
                onConfirm: () => { }
            }
        }),
    actions: {
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
