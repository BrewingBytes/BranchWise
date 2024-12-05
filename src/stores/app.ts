import { DEFAULT_USER } from "@/types/user";
import { defineStore } from "pinia";

interface IAppState {
    title: string;
    user: typeof DEFAULT_USER;
    isNavbarOpen: boolean;
    appVersion: string;
}

export const useAppStore = defineStore("app", {
	state: (): IAppState => (
		{
			title: "BranchWise",
			user: DEFAULT_USER,
			isNavbarOpen: false,
			appVersion: "0.0.9",
		}),
	getters: {
		getAppVersion(): string {
			return `v${this.appVersion}`;
		}
	},
	actions: {
		setTitle(title: string) {
			this.title = title;
		},
		toggleNavbar() {
			this.isNavbarOpen = !this.isNavbarOpen;
		},
	}
});
