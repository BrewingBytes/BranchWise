<script setup lang="ts">
import { useProjectStore } from "@/stores/project";
import { IGitProject } from "@/types/gitProject";
import { computed } from "vue";
import { useRouter } from "vue-router";

const props = defineProps({
  project: {
    type: Object as () => IGitProject,
    required: true
  }
});

const router = useRouter();
const name = computed(() => props.project.directory.split("/").pop());

function openProjectPage() {
  useProjectStore().setCurrentProject(props.project);
  router.push("/project");
};
</script>

<template>
  <v-container
    @click="openProjectPage"
  >
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

<style scoped>
.v-container {
    width: max-content;
    border: 1px solid #e0e0e0;
    border-radius: 10px;
    min-height: 120px;
    max-height: 120px;
    overflow: hidden;
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
