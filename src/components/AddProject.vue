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

<script lang="ts">
import { useDialogStore } from "@/stores/dialogs";
import { useProjectStore } from "@/stores/project";
import { IGitProject } from "@/types/gitProject";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { defineComponent } from "vue";

export default defineComponent({
    name: "AddProject",
    methods: {
        async openNewProject() {
            const result = await open({
                directory: true,
                multiple: false
            });

            if (result) {
                try {
                    const project: IGitProject = await invoke("open_git_project", { directory: result });
                    useProjectStore().addProject(project);
                } catch (error) {
                    useDialogStore().openSnackbar({ text: error as string, color: "error" });
                }
            }
        }
    }
});
</script>

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
