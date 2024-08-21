<template>
  <v-container>
    <v-row no-gutters>
      <v-col
        v-for="project in getProjects"
        :key="project.directory"
        class="mb-4"
        cols="6"
        sm="4"
        md="2"
      >
        <Project :project="project" />
      </v-col>
      <v-col
        class="mb-4"
        cols="6"
        sm="4"
        md="2"
      >
        <AddProject />
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import { defineComponent } from "vue";

import AddProject from "../components/AddProject.vue";
import Project from "../components/Project.vue";
import { mapState } from "pinia";
import { useAppStore } from "../stores/app";

export default defineComponent({
    name: "IndexPage",
    components: {
        AddProject,
        Project
    },
    computed: {
        ...mapState(useAppStore, ["getProjects"])
    }
});
</script>
