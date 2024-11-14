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
        @append-click="confirmExit"
      />
    </v-list>
  </v-navigation-drawer>
</template>

<script lang="ts">
import SidebarItem from "@/components/Sidebar/SidebarItem.vue";
import { PrependVariant } from "@/enums/prependVariant";
import { useAppStore } from "@/stores/app";
import { useDialogStore } from "@/stores/dialogs";
import { useProjectStore } from "@/stores/project";
import { invoke } from "@tauri-apps/api/core";
import { exit } from "@tauri-apps/plugin-process";
import { mapState } from "pinia";
import { defineComponent } from "vue";

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
      return useProjectStore().getSelectedProject !== null;
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
          project: useProjectStore().getSelectedProject,
        });

        useProjectStore().removeProject();
        this.$router.push("/");
      } catch (e) {
        console.error(e);
      }
    },
    confirmExit() {
      useDialogStore().openConfirmationDialog({
        title: "Exit",
        message: "Are you sure you want to exit?",
        onConfirm: () => {
          exit();
        },
      });
    }
  }
});
</script>
