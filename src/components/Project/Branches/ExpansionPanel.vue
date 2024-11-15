<template>
  <v-expansion-panel
    static
  >
    <v-expansion-panel-title>{{ title }}</v-expansion-panel-title>
    <v-expansion-panel-text style="max-height: 50vh; overflow: scroll">
      <v-treeview
        :items="branches"
        :expand-icon="expandIcon"
        :collapse-icon="collapseIcon"
        item-key="title"
        item-props
      >
        <template #prepend="{ item }">
          <v-icon
            v-if="!item.children"
            :icon="itemIcon"
            :class="getSelectedClass(item.branch)"
          />
        </template>
        <template #title="{ item }">
          <p
            v-if="!item.children"
            :class="getSelectedClass(item.branch)"
            @click="setBranch(item.branch)"
          >
            {{ item.title }}
          </p>
          <p v-else>
            {{ item.title }}
          </p>
        </template>
      </v-treeview>
    </v-expansion-panel-text>
  </v-expansion-panel>
</template>

<script lang="ts">
import { useProjectStore } from "@/stores/project";
import { IBranchTreeItem } from "@/types/branchTreeItem";
import { IGitBranch } from "@/types/gitBranch";
import { defineComponent } from "vue";

export default defineComponent({
	name: "ExpansionPanel",
	props: {
		branches: {
			type: Array as () => IBranchTreeItem[] | undefined,
			required: true,
		},
		title: {
			type: String,
			required: true,
		},
		expandIcon: {
			type: String,
			default: "mdi-folder",
		},

		collapseIcon: {
			type: String,
			default: "mdi-folder-open",
		},
		itemIcon: {
			type: String,
			default: "mdi-source-branch",
		},
		customIcon: {
			type: String,
			default: "",
		},
	},
	methods: {
		setBranch(branch: IGitBranch | undefined) {
			if (!branch) {
				return;
			}

			useProjectStore().setBranch(branch);
		},
		getSelectedClass(branch: IGitBranch | undefined) {
			return {
				"selected-branch": branch?.name === useProjectStore().getBranch?.name,
			};
		},
	}
});
</script>

<style scoped>
.selected-branch {
  font-weight: bold;
  color: #1976d2;
}
</style>
