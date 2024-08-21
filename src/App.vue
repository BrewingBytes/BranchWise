<template>
  <v-app>
    <NavbarComponent :is-open="isNavbarOpen" />
    <ToolbarComponent />
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
  </v-app>
</template>

<script lang="ts">
import { defineComponent, provide } from "vue";
import ToolbarComponent from "./components/ToolbarComponent.vue";
import NavbarComponent from "./components/NavbarComponent.vue";
import { GitError } from "./types/gitErrors";
import { invoke } from "@tauri-apps/api";
import { useAppStore } from "./stores/app";
import { mapState } from "pinia";

export default defineComponent({
  name: "AppComponent",
  components: {
    ToolbarComponent,
    NavbarComponent,
  },
  data() {
    return {
      snackbar: {
        show: false,
        text: "",
        color: "",
        timeout: 5000,
      },
    };
  },
  computed: {
    ...mapState(useAppStore, ["isNavbarOpen"]),
  },
  async mounted() {
    provide("showError", this.showError);

    try {
      useAppStore().setProjects(await invoke("get_database_projects"));
    } catch (error) {
      console.error(error);
    }
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
