<template>
  <v-container>
    <v-row>
      <v-col>
        <v-card>
          <v-card-title>
            <h1>{{ project!.directory }}</h1>
          </v-card-title>
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
        projectName(): string {
            return this.project?.directory.split("/").pop() || "";
        },
        ...mapState(useAppStore, {
            project: "getSelectedProject",
        })
    },
    mounted() {
        if (!this.project) {
            this.$router.push("/");
        } else {
          useAppStore().setTitle(this.projectName);
        }
    }
});
</script>
