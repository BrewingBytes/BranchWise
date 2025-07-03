import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { TauriListen } from "@/types/tauri";
import { IGitProject } from "@/types/gitProject";
import { useProjectStore } from "@/stores/project";
import { ref } from "vue";

const listeners = ref([] as UnlistenFn[]);

/**
 * Registers a listener for project update events from Tauri and updates the project store when such events occur.
 */
export async function registerListeners() {
	listeners.value.push(await listen(TauriListen.ProjectUpdate, (event) => {
		useProjectStore().updateProject(event.payload as IGitProject);
	}));
}

/**
 * Removes all registered event listeners by invoking their unlisten functions.
 */
export function unregisterListeners() {
	listeners.value.forEach(unlisten => unlisten());
}
