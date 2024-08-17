<template>
  <v-app>
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
import { GitError } from "./types/gitErrors";

export default defineComponent({
  name: "AppComponent",
  components: {
    ToolbarComponent,
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
  mounted() {
    provide("showError", this.showError);
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
