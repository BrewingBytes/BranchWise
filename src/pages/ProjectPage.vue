<script setup lang="ts">
import BranchesSidebar from "@/components/Project/BranchesSidebar.vue";
import CommitDetails from "@/components/Project/Commit/CommitDetails.vue";
import CommitHistory from "@/components/Project/CommitHistory.vue";
import { useAppStore } from "@/stores/app";
import { useProjectStore } from "@/stores/project";
import { computed, defineComponent, onMounted } from "vue";

const project = computed(() => useProjectStore().getSelectedProject);
const projectName = computed(() => project.value?.directory.split("/").pop() || "");

onMounted(() => useAppStore().setTitle(projectName.value));
</script>

<template>
  <v-row
    class="fill-height"
    no-gutters
  >
    <v-col
      align="center"
      class="pa-0"
      cols="4"
      style="border-right: 1px solid #e0e0e0;"
    >
      <BranchesSidebar />
    </v-col>
    <v-col
      align="center"
      class="pa-0"
      cols="4"
    >
      <CommitHistory />
    </v-col>
    <v-col
      align="center"
      class="pa-0"
      cols="4"
      style="border-left: 1px solid #e0e0e0;"
    >
      <CommitDetails />
    </v-col>
  </v-row>
</template>

<script lang="ts">
export default defineComponent({
	beforeRouteEnter(_to, _from, next) {
		if (!useProjectStore().getSelectedProject) {
			next("/");
		} else {
			next();
		}
	},
});
</script>

<style scoped>
.v-expansion-panel--active {
  margin: 0 !important;
  border: 0 !important;
}

.v-expansion-panel-text__wrapper {
  padding: 0 !important;
}

.v-expansion-panel-text :deep(.v-expansion-panel-text__wrapper) {
  padding: 0 !important;
}

.v-treeview-item {
  padding: 0 !important;
}

.v-treeview-item--activetable-group-activator {
  padding: 0 !important;
}
</style>
