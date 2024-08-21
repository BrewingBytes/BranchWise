<template>
  <v-container>
    <v-col
      align="center"
      class="pa-0"
    >
      <v-btn
        :ripple="false"
        flat
        base-color="transparent"
        variant="flat"
        icon
      >
        <v-icon>mdi:mdi-source-branch</v-icon>
      </v-btn>
      <p>
        {{ name }}
      </p>
    </v-col>
  </v-container>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { IGitProject } from "../types/gitProject";

export default defineComponent({
    name: "ProjectComponent",
    props: {
        project: {
            type: Object as () => IGitProject,
            required: true
        }
    },
    computed: {
        name() {
            return this.project.directory.split("/").pop();
        }
    }
});
</script>

<style scoped>
.v-container {
    width: max-content;
    border: 1px solid #e0e0e0;
    border-radius: 10px;
}

.v-container:hover,
.v-container:hover .v-btn {
    background-color: #f0f0f0;
    color: #000;
    cursor: pointer;
}

p {
    display: block;
    width: 100px;
    word-wrap: break-word;
    text-align: center;
    font-size: 12px;
}
</style>
