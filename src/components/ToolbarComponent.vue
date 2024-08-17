<template>
  <v-app-bar
    elevation="0"
    app
  >
    <v-toolbar-title>
      <v-row
        fluid
        height="100%"
        no-gutters
        class="mr-6"
        align="center"
      >
        <v-tooltip
          text="Navigating with Precision and Wisdom"
          location="bottom"
        >
          <template #activator="{ props }">
            <v-icon
              class="mr-2"
              icon="mdi:mdi-source-branch"
              v-bind="props"
            />
            <p
              class="text-h5"
              v-bind="props"
            >
              {{ title }}
            </p>
          </template>
        </v-tooltip>
        <v-spacer />
        <v-tooltip
          :text="user.name"
          location="bottom"
        >
          <template #activator="{ props }">
            <v-btn
              class="mr-2"
              icon
              v-bind="props"
            >
              <v-avatar>
                <v-img :src="user.avatar" />
              </v-avatar>
            </v-btn>
          </template>
        </v-tooltip>
        <v-tooltip
          text="Notifications"
          location="bottom"
        >
          <template #activator="{ props }">
            <v-btn
              icon
              class="mr-2"
              v-bind="props"
            >
              <v-icon>mdi:mdi-bell</v-icon>
            </v-btn>
          </template>
        </v-tooltip>
        <v-btn icon>
          <v-icon>mdi:mdi-dots-vertical</v-icon>
          <v-menu activator="parent">
            <v-list>
              <v-list-item
                v-for="(item, index) in menuItems"
                :key="index"
                @click="item.function"
              >
                {{ item.title }}
              </v-list-item>
            </v-list>
          </v-menu>
        </v-btn>
      </v-row>
    </v-toolbar-title>
    <v-dialog
      v-model="showExitDialog"
      persistent
    >
      <v-card>
        <v-card-title class="headline">
          Exit Application
        </v-card-title>
        <v-card-text>Are you sure you want to exit the application?</v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn
            rounded
            @click="showExitDialog = false"
          >
            Cancel
          </v-btn>
          <v-btn
            rounded
            @click="exit"
          >
            Exit
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-app-bar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useAppStore } from "../stores/app";
import { mapState } from "pinia";
import { exit } from "@tauri-apps/api/process";

type ClickCallback = (event: Event) => void;

interface IMenuItem {
    title: string;
    function: ClickCallback;
}

export default defineComponent({
    name: "ToolbarComponent",
    data() {
        return {
            menuItems: [
                { title: "Exit", function: () => this.switchExitDialog() },
            ] as IMenuItem[],
            showExitDialog: false,
        };
    },
    computed: {
        ...mapState(useAppStore, ["title", "user"]),
    },
    methods: {
        switchExitDialog() {
            this.showExitDialog = true;
        },
        exit() {
            exit();
        },
    },
});
</script>

<style scoped>
.v-icon,
.text-h5 {
    cursor: pointer;
}
</style>
