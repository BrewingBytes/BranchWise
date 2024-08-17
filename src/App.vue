<template>
  <v-app>
    <v-app-bar elevation="0" app>
      <v-toolbar-title>
        <v-row fluid height="100%" no-gutters class="mr-6">
          <v-icon class="mr-2" icon="mdi:mdi-source-branch"></v-icon>
          <p>{{ title }}</p>
          <v-spacer />
          <v-btn class="mr-2" icon>
            <v-avatar>
              <v-img src="https://cdn.vuetifyjs.com/images/logos/logo.svg" />
            </v-avatar>
          </v-btn>
          <v-btn icon class="mr-2">
            <v-icon>mdi:mdi-bell</v-icon>
          </v-btn>
          <v-btn icon>
            <v-icon>mdi:mdi-dots-vertical</v-icon>
          </v-btn>
        </v-row>
      </v-toolbar-title>
    </v-app-bar>
    <v-main>
      <router-view />
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useAppStore } from "./stores/app";
import { mapState } from "pinia";

export default defineComponent({
  name: "AppComponent",
  computed: {
    ...mapState(useAppStore, ["title"]),
  }
});
</script>
