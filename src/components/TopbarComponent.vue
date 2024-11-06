<template>
  <v-app-bar
    elevation="0"
    app
  >
    <v-toolbar-title>
      <v-row
        ref="title"
        fluid
        height="100%"
        no-gutters
        class="mr-6"
        align="center"
      >
        <v-icon
          class="mr-2"
          icon="mdi:mdi-menu"
          @click="toggleNavbar"
        />
        <v-spacer />
        <v-icon
          class="mr-2"
          icon="mdi:mdi-source-branch"
        />
        <p
          class="text-h5"
        >
          {{ title }}
        </p>
        <v-spacer />
      </v-row>
    </v-toolbar-title>
  </v-app-bar>
</template>

<script lang="ts">
import { CreateComponentPublicInstance, defineComponent } from "vue";
import { useAppStore } from "../stores/app";
import { mapState } from "pinia";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
const appWindow = getCurrentWebviewWindow()

export default defineComponent({
    name: "TopbarComponent",
    data() {
        return {
            x: 0,
            y: 0,
        };
    },
    computed: {
        ...mapState(useAppStore, ["title"]),
    },
    mounted() {
        (this.$refs.title as CreateComponentPublicInstance).$el.addEventListener("mousedown", this.startDragging);
    },
    methods: {
        async startDragging(event: Event) {
            if (event.target === (this.$refs.title as CreateComponentPublicInstance).$el) {
                event.preventDefault();
                await appWindow.startDragging();
            }
        },
        toggleNavbar() {
            useAppStore().toggleNavbar();
        },
    },
});
</script>
