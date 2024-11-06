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
        @append-click="showExitDialog = true"
      />
    </v-list>
  </v-navigation-drawer>
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
          @click="exit(0)"
        >
          Exit
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { exit } from "@tauri-apps/plugin-process";
import { mapState } from "pinia";
import { defineComponent } from "vue";
import { PrependVariant } from "../enums/prependVariant";
import { useAppStore } from "../stores/app";
import SidebarItem from "./Sidebar/SidebarItem.vue";

export default defineComponent({
  name: "SidebarComponent",
  components: {
    SidebarItem,
  },
  props: {
    isOpen: {
      type: Boolean,
      required: true,
    }
  },
  data() {
    return {
      showExitDialog: false,
      PrependVariant,
    };
  },
  computed: {
    isProjectSelected() {
      return useAppStore().getSelectedProject !== null;
    },
    appVersion() {
      return `Version ${this.getAppVersion}`;
    },
    ...mapState(useAppStore, ["user", "getAppVersion"])
  },
  methods: {
    closeMe() {
      useAppStore().toggleNavbar();
    },
    goHome() {
      this.$router.push("/");
    },
    async deleteProject() {
      try {
        await invoke("remove_database_project", {
          project: useAppStore().getSelectedProject,
        });

        useAppStore().removeProject();
        this.$router.push("/");
      } catch (e) {
        console.error(e);
      }
    },
    exit
  }
});
</script>
