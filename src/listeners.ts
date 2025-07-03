import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { TauriListen } from "@/types/tauri";
import { IGitProject } from "@/types/gitProject";
import { useProjectStore } from "@/stores/project";

const listeners: UnlistenFn[] = [];

export async function registerListeners() {
	listeners.push(await listen(TauriListen.ProjectUpdate, (event) => {
		useProjectStore().updateProject(event.payload as IGitProject);
	}));
}

export function unregisterListeners() {
	listeners.forEach(unlisten => unlisten());
}
