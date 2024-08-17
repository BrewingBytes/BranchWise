<template>
  <v-app>
    <ToolbarComponent />
    <v-main>
      <router-view />
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ToolbarComponent from "./components/ToolbarComponent.vue";

export default defineComponent({
  name: "AppComponent",
  components: {
    ToolbarComponent,
  },
});
</script>
