<script setup lang="ts">
import { useAppStore } from "@/stores/app";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { computed, onMounted, useTemplateRef } from "vue";

const appWindow = getCurrentWebviewWindow();
// @ts-ignore
const titleRef = useTemplateRef("title");

const title = computed(() => useAppStore().title);

onMounted(() => titleRef.value?.$el.addEventListener("mousedown", startDragging));
const toggleNavbar = () => useAppStore().toggleNavbar();
async function startDragging(event: Event) {
  if (event.target === titleRef.value?.$el) {
    event.preventDefault();
    await appWindow.startDragging();
  }
};
</script>

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
