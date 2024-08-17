<template>
  <v-container @click="openNewProject">
    <v-col align="center" class="pa-0">
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
import { defineComponent, inject } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";

export default defineComponent({
    name: "AddProject",
    data() {
        const showError: (event: any) => void = (e) => {
            console.error(e);
        };

        return {
            showError
        }
    },
    mounted() {
        const showError = inject<(event: any) => void>("showError");
        if (showError) {
            this.showError = showError;
        } else {
            throw new Error("showError not provided");
        }
    },
    methods: {
        async openNewProject() {
            const result = await open({
                directory: true,
                multiple: false
            });

            if (result) {
                try {
                    await invoke("open_git_project", { directory: result });
                } catch (error) {
                    this.showError(error);
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
}

.v-container:hover,
.v-container:hover .v-btn {
    background-color: #f0f0f0;
    color: #000;
    cursor: pointer;
}

p {
    display: block;
    width:50px;
    word-wrap:break-word;
    text-align: center;
    font-size: 12px;
}
</style>
