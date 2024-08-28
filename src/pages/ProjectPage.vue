<template>
  <v-container>
    <v-row>
      <v-col>
        <v-card>
          <v-card-title>
            <h1>{{ project.directory }}</h1>
          </v-card-title>
          <v-card-text>
            <p>{{ project.description }}</p>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import { defineComponent } from "vue";

import { mapState } from "pinia";
import { useAppStore } from "../stores/app";

export default defineComponent({
    name: "IndexPage",
    computed: {
        ...mapState(useAppStore, {
            project: "getSelectedProject",
        })
    },
    mounted() {
        if (!this.project) {
            this.$router.push("/");
        }
    }
});
</script>
