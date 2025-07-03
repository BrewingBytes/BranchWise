<script setup lang="ts">
import ExpansionPanel from "@/components/Project/Branches/ExpansionPanel.vue";
import { useProjectStore } from "@/stores/project";
import { IBranchTreeItem } from "@/types/branchTreeItem";
import { IDirectory } from "@/types/directory";
import { BranchType, IGitBranch } from "@/types/gitBranch";
import { computed } from "vue";

const project = computed(() => useProjectStore().getSelectedProject);
const localProjectBranchesTree = computed(() => branchTreeToTreeview(BranchType.HEADS));
const tagsProjectBranchesTree = computed(() => branchTreeToTreeview(BranchType.TAGS));
const remoteProjectBranchesTree = computed(() => {
	const tree = branchTreeToTreeview(BranchType.REMOTES);
	if (!tree) {
		return [];
	}

	tree.forEach((root) => {
		root.customIcon = "mdi:mdi-cloud";
	});

	return tree;
});

function makeBranchesTree(branchType: BranchType) {
	const tree: IDirectory = {
		name: "",
		children: [],
		branches: [],
	};

	let branches: IGitBranch[] = [];
	if (branchType === BranchType.HEADS) {
		branches = project.value?.localBranches || [];
	} else if (branchType === BranchType.REMOTES) {
		branches = project.value?.remoteBranches || [];
	} else if (branchType === BranchType.TAGS) {
		branches = project.value?.tags || [];
	}

	branches.forEach((branch) => {
		const branchParts = branch.name.split("/");
		let currentDirectory = tree;
		for (let i = 0; i < branchParts.length - 1; i++) {
			const directoryName = branchParts[i];
			const existingDirectory = currentDirectory.children.find((child) => child.name === directoryName);
			if (existingDirectory) {
				currentDirectory = existingDirectory;
			} else {
				const newDirectory: IDirectory = {
					name: directoryName,
					children: [],
					branches: [],
				};
				currentDirectory.children.push(newDirectory);
				currentDirectory = newDirectory;
			}
		}
		currentDirectory.branches.push({ ...branch, dir_name: branchParts[branchParts.length - 1] });
	});

	return tree;
};

function branchTreeToTreeview(branchType: BranchType) {
	const treeItems: IBranchTreeItem[] = [];
	const items = makeBranchesTree(branchType);

	const addBranches = (directory: IDirectory, parent: IBranchTreeItem | null = null) => {
		const item: IBranchTreeItem = {
			id: treeItems.length,
			title: directory.name,
			children: [],
		};
		if (parent) {
			parent.children?.push(item);
		} else {
			treeItems.push(item);
		}

		directory.children.forEach((child) => {
			addBranches(child, item);
		});

		directory.branches.forEach((branch) => {
			item.children?.push({
				id: treeItems.length,
				title: branch.dir_name,
				branch: { ...branch }
			});
		});
	};

	addBranches(items);
	return treeItems[0].children;
};
</script>

<template>
  <v-expansion-panels
    variant="accordion"
    style="overflow: hidden;"
  >
    <ExpansionPanel
      title="Branches"
      :branches="localProjectBranchesTree"
    />
    <ExpansionPanel
      title="Remotes"
      :branches="remoteProjectBranchesTree"
      expand-icon="mdi-cloud"
      collapse-icon="mdi-cloud"
    />
    <ExpansionPanel
      title="Tags"
      :branches="tagsProjectBranchesTree"
      expand-icon="mdi-tag-arrow-down"
      collapse-icon="mdi-tag-arrow-up"
      item-icon="mdi-tag"
    />
  </v-expansion-panels>
</template>
