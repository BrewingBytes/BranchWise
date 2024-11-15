<template>
  <v-app>
    <SidebarComponent :is-open="isNavbarOpen" />
    <TopbarComponent />
    <v-main>
      <router-view />
    </v-main>
    <v-snackbar
      v-model="snackbar.show"
      :color="snackbar.color"
      :timeout="snackbar.timeout"
    >
      {{ snackbar.text }}
    </v-snackbar>
    <DialogComponent />
  </v-app>
</template>

<script lang="ts">
import DialogComponent from "@/components/DialogComponent.vue";
import SidebarComponent from "@/components/SidebarComponent.vue";
import TopbarComponent from "@/components/TopbarComponent.vue";
import { registerListeners, unregisterListeners } from "@/listeners";
import { useAppStore } from "@/stores/app";
import { useDialogStore } from "@/stores/dialogs";
import { useProjectStore } from "@/stores/project";
import { TauriCommands } from "@/types/tauri";
import { invoke } from "@tauri-apps/api/core";
import { UnlistenFn } from "@tauri-apps/api/event";
import { mapState } from "pinia";
import { defineComponent } from "vue";

export default defineComponent({
	name: "AppComponent",
	components: {
		SidebarComponent,
		TopbarComponent,
		DialogComponent
	},
	data() {
		return {
			listeners: [] as UnlistenFn[],
		};
	},
	computed: {
		...mapState(useAppStore, ["isNavbarOpen"]),
		...mapState(useDialogStore, ["snackbar"]),
	},
	async mounted() {
		try {
			useProjectStore().setProjects(await invoke(TauriCommands.GetDatabaseProjects));
		} catch (error) {
			useDialogStore().showError(error);
		}

		registerListeners();
	},
	unmounted() {
		unregisterListeners();
	},
});
</script>

<style>
html {
    overflow: scroll;
    overflow-x: hidden;
	overflow-y: hidden;
}

::-webkit-scrollbar {
    width: 0;
    background: transparent;
	display: none;
}
</style>
