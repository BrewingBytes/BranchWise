<script setup lang="ts">
import { useDialogStore } from "@/stores/dialogs";
import { useProjectStore } from "@/stores/project";
import { IGitProject } from "@/types/gitProject";
import { TauriCommands } from "@/types/tauri";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

async function openNewProject(): Promise<void> {
  const result = await open({
      directory: true,
      multiple: false
    });

    if (!result) {
      return;
    }

    try {
      const project: IGitProject = await invoke(TauriCommands.OpenGitProject, { directory: result });
      useProjectStore().addProject(project);
    } catch (error) {
      useDialogStore().showError(error);
    }
}
</script>

<template>
  <v-container @click="openNewProject">
    <v-col
      align="center"
      class="pa-0"
    >
      <v-btn
        :ripple="false"
        flat
        base-color="transparent"
        variant="flat"
        icon
      >
        <v-icon>mdi:mdi-plus</v-icon>
      </v-btn>
      <p>
        Add Project
      </p>
    </v-col>
  </v-container>
</template>

<style scoped>
.v-container {
    width: max-content;
    border: 1px solid #e0e0e0;
    border-radius: 10px;
    min-height: 120px;
    max-height: 120px;
    overflow: hidden;
}

.v-container:hover,
.v-container:hover .v-btn {
    background-color: #f0f0f0;
    color: #000;
    cursor: pointer;
}

p {
    display: block;
    width: 100px;
    word-wrap:break-word;
    text-align: center;
    font-size: 12px;
}
</style>
