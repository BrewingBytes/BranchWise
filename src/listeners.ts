import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { TauriListen } from "@/types/tauri";
import { IGitProject } from "@/types/gitProject";
import { useProjectStore } from "@/stores/project";
import { ref } from "vue";

const listeners = ref([] as UnlistenFn[]);

export async function registerListeners() {
	listeners.value.push(await listen(TauriListen.ProjectUpdate, (event) => {
		useProjectStore().updateProject(event.payload as IGitProject);
	}));
}

export function unregisterListeners() {
	listeners.value.forEach(unlisten => unlisten());
}
