<script setup lang="ts">
import SidebarItem from "@/components/Sidebar/SidebarItem.vue";
import { PrependVariant } from "@/enums/prependVariant";
import { useAppStore } from "@/stores/app";
import { useDialogStore } from "@/stores/dialogs";
import { useProjectStore } from "@/stores/project";
import { TauriCommands } from "@/types/tauri";
import { invoke } from "@tauri-apps/api/core";
import { exit } from "@tauri-apps/plugin-process";
import { storeToRefs } from "pinia";
import { computed } from "vue";
import { useRouter } from "vue-router";

defineProps({
	isOpen: {
		type: Boolean,
		required: true
	}
});

const router = useRouter();
const { user, getAppVersion } = storeToRefs(useAppStore());

const isProjectSelected = computed(() => useProjectStore().getSelectedProject !== null);
const appVersion = computed(() => `Version ${getAppVersion.value}`);

const closeMe = () => useAppStore().toggleNavbar();
const goHome = () => router.push("/");
const confirmExit = () => 
	useDialogStore().openConfirmationDialog({
		title: "Exit",
		message: "Are you sure you want to exit?",
		onConfirm: () => {
			exit();
		},
	});

async function deleteProject() {
	try {
		await invoke(TauriCommands.RemoveDatabaseProject, {
			project: useProjectStore().getSelectedProject,
		});

		useProjectStore().removeProject();
		router.push("/");
	} catch (e) {
		useDialogStore().showError(e);
	}
};
</script>

<template>
  <v-navigation-drawer :model-value="isOpen">
    <v-list style="height:100%; display: flex; flex-direction: column">
      <SidebarItem
        prepend-icon="mdi-arrow-left"
        text=""
        append-icon="mdi-bell"
        @prepend-click="closeMe"
      />
      <v-divider />
      <SidebarItem
        prepend-color="red"
        prepend-icon="mdi-home"
        text="Projects"
        @click="goHome"
      />
      <SidebarItem
        v-if="isProjectSelected"
        prepend-color="blue"
        prepend-icon="mdi-source-branch"
        text="Branches"
      />
      <v-spacer />
      <SidebarItem
        v-if="isProjectSelected"
        prepend-color="red"
        prepend-icon="mdi-delete"
        text="Delete Project"
        @click="deleteProject"
      />
      <v-divider />
      <SidebarItem
        :prepend-icon="user.avatar"
        :prepend-variant="PrependVariant.AVATAR"
        :text="user.name"
        :subtitle="appVersion"
        append-icon="mdi-power"
        append-color="red"
        @append-click="confirmExit"
      />
    </v-list>
  </v-navigation-drawer>
</template>
