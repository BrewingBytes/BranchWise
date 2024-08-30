<template>
  <v-navigation-drawer :model-value="isOpen">
    <v-list style="height:100%; display: flex; flex-direction: column">
      <v-list-item>
        <v-row>
          <v-col cols="3">
            <v-icon icon="mdi:mdi-source-branch" />
          </v-col>
          <v-col cols="9">
            <p class="text-h5">
              BranchWise
            </p>
          </v-col>
        </v-row>
      </v-list-item>
      <v-divider />
      <v-list-item @click="goHome">
        <v-row>
          <v-col cols="3">
            <v-icon icon="mdi:mdi-home" />
          </v-col>
          <v-col cols="9">
            <p class="text-h5">
              Projects
            </p>
          </v-col>
        </v-row>
      </v-list-item>
      <v-divider />
      <template v-if="isProjectSelected">
        <v-list-item>
          <v-row>
            <v-col cols="3">
              <v-icon icon="mdi:mdi-source-branch" />
            </v-col>
            <v-col cols="9">
              <p class="text-h5">
                Branches
              </p>
            </v-col>
          </v-row>
        </v-list-item>
      </template>
      <v-spacer />
      <v-list-item @click="deleteProject">
        <v-row>
          <v-col cols="3">
            <v-icon icon="mdi:mdi-delete" />
          </v-col>
          <v-col cols="9">
            <p class="text-h5">
              Delete Project
            </p>
          </v-col>
        </v-row>
      </v-list-item>
      <v-divider />
      <v-list-item @click="closeMe">
        <v-row>
          <v-col cols="3">
            <v-icon icon="mdi:mdi-arrow-left" />
          </v-col>
          <v-col cols="9">
            <p class="text-h5">
              Close
            </p>
          </v-col>
        </v-row>
      </v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useAppStore } from "../stores/app";
import { invoke } from "@tauri-apps/api";

export default defineComponent({
  name: "NavbarComponent",
  props: {
    isOpen: {
      type: Boolean,
      required: true,
    }
  },
  computed: {
    isProjectSelected() {
      return useAppStore().getSelectedProject !== null;
    }
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
    }
  }
});
</script>
