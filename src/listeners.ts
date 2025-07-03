import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { TauriListen } from "@/types/tauri";
import { IGitProject } from "@/types/gitProject";
import { useProjectStore } from "@/stores/project";

const listeners: UnlistenFn[] = [];

/**
 * Registers a listener for project update events from Tauri and updates the project store when such events occur.
 */
export async function registerListeners() {
	listeners.push(await listen(TauriListen.ProjectUpdate, (event) => {
		useProjectStore().updateProject(event.payload as IGitProject);
	}));
}

/**
 * Removes all registered event listeners by invoking their unlisten functions.
 */
export function unregisterListeners() {
	listeners.forEach(unlisten => unlisten());
}
