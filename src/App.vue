<script setup lang="ts">
import ContextMenu from "@/components/Dialogs/ContextMenu.vue";
import DialogComponent from "@/components/DialogComponent.vue";
import SidebarComponent from "@/components/SidebarComponent.vue";
import TopbarComponent from "@/components/TopbarComponent.vue";
import { registerListeners, unregisterListeners } from "@/listeners";
import { useAppStore } from "@/stores/app";
import { useDialogStore } from "@/stores/dialogs";
import { useProjectStore } from "@/stores/project";
import { TauriCommands } from "@/types/tauri";
import { invoke } from "@tauri-apps/api/core";
import { storeToRefs } from "pinia";
import { onMounted, onUnmounted } from "vue";

const { isNavbarOpen } = storeToRefs(useAppStore());
const { snackbar } = storeToRefs(useDialogStore());

onMounted(async () => {
	try {
		useProjectStore().setProjects(await invoke(TauriCommands.GetDatabaseProjects));
	} catch (error) {
		useDialogStore().showError(error);
	}

	registerListeners();
});

onUnmounted(() => unregisterListeners());
</script>

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
    <ContextMenu />
  </v-app>
</template>

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
