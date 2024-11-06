<template>
  <v-navigation-drawer :model-value="isOpen">
    <v-list style="height:100%; display: flex; flex-direction: column">
      <v-list-item>
        <v-row
          align="center"
          no-gutters
        >
          <v-btn
            variant="plain"
            icon
          >
            <v-icon @click="closeMe">
              mdi-arrow-left
            </v-icon>
          </v-btn>
          <v-spacer />
          <v-btn
            variant="plain"
            icon
          >
            <v-icon>mdi-bell</v-icon>
          </v-btn>
        </v-row>
      </v-list-item>
      <v-divider />
      <v-list-item @click="goHome">
        <v-row>
          <v-col cols="3">
            <v-icon
              color="green"
              icon="mdi:mdi-home"
            />
          </v-col>
          <v-col cols="9">
            <p class="text-h5">
              Projects
            </p>
          </v-col>
        </v-row>
      </v-list-item>
      <template v-if="isProjectSelected">
        <v-list-item>
          <v-row>
            <v-col cols="3">
              <v-icon
                color="blue"
                icon="mdi:mdi-source-branch"
              />
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
      <v-list-item
        v-if="isProjectSelected" 
        @click="deleteProject"
      >
        <v-row>
          <v-col cols="3">
            <v-icon
              color="red"
              icon="mdi:mdi-delete"
            />
          </v-col>
          <v-col cols="9">
            <p class="text-h5">
              Delete Project
            </p>
          </v-col>
        </v-row>
      </v-list-item>
      <v-divider />
      <v-list-item class="pa-0 pt-4"> 
        <v-row no-gutters>
          <v-col
            cols="3"
            align="center"
          >
            <v-avatar>
              <v-img :src="user.avatar" />
            </v-avatar>
          </v-col>
          <v-col cols="6">
            <p class="text-subtitle-1">
              {{ user.name }}
            </p>
            <p class="text-caption font-weight-thin">
              Version {{ getAppVersion }}
            </p>
          </v-col>
          <v-col
            cols="3"
            align="center"
          >
            <v-btn
              size="small"
              icon
              @click="showExitDialog = true"
            >
              <v-icon color="red">
                mdi:mdi-power
              </v-icon>
            </v-btn>
          </v-col>
        </v-row>
      </v-list-item>
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
import { useAppStore } from "../stores/app";

export default defineComponent({
  name: "SidebarComponent",
  props: {
    isOpen: {
      type: Boolean,
      required: true,
    }
  },
  data() {
    return {
      showExitDialog: false,
    };
  },
  computed: {
    isProjectSelected() {
      return useAppStore().getSelectedProject !== null;
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
