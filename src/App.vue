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
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { mapState } from "pinia";
import { defineComponent, provide } from "vue";
import DialogComponent from "./components/DialogComponent.vue";
import SidebarComponent from "./components/SidebarComponent.vue";
import TopbarComponent from "./components/TopbarComponent.vue";
import { useAppStore } from "./stores/app";
import { GitError } from "./types/gitErrors";
import { IGitProject } from "./types/gitProject";
import { useProjectStore } from "./stores/project";

export default defineComponent({
  name: "AppComponent",
  components: {
    SidebarComponent,
    TopbarComponent,
    DialogComponent
  },
  data() {
    return {
      snackbar: {
        show: false,
        text: "",
        color: "",
        timeout: 5000,
      },
      listeners: [] as UnlistenFn[],
    };
  },
  computed: {
    ...mapState(useAppStore, ["isNavbarOpen"]),
  },
  async mounted() {
    provide("showError", this.showError);

    try {
      useProjectStore().setProjects(await invoke("get_database_projects"));
    } catch (error) {
      this.showError(error as string);
    }

    const unlisten = await listen("project_update", (event) => {
      useProjectStore().updateProject(event.payload as IGitProject);
    });

    this.listeners.push(unlisten);
  },
  unmounted() {
    this.listeners.forEach((unlisten) => unlisten());
  },
  methods: {
    showError(error: string) {
      this.snackbar.show = true;
      this.snackbar.text = GitError[error as keyof typeof GitError];
      this.snackbar.color = "red";
    },
  },
});
</script>

<style>
@import '@vue-flow/core/dist/style.css';
@import '@vue-flow/core/dist/theme-default.css';
</style>
