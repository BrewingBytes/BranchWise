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
      <v-expansion-panels>
        <v-expansion-panel static>
          <v-expansion-panel-title>Branches</v-expansion-panel-title>
          <v-expansion-panel-text>
            <v-treeview
              :items="localProjectBranchesTree"
              expand-icon="mdi:mdi-folder"
              collapse-icon="mdi:mdi-folder-open"
              item-key="title"
              item-props
            >
              <template #prepend="{ item }">
                <v-icon
                  v-if="!item.children"
                  icon="mdi:mdi-source-branch"
                />
              </template>
            </v-treeview>
          </v-expansion-panel-text>
        </v-expansion-panel>
        <v-expansion-panel static>
          <v-expansion-panel-title>Remotes</v-expansion-panel-title>
          <v-expansion-panel-text>
            <v-treeview
              :items="remoteProjectBranchesTree"
              expand-icon=""
              collapse-icon=""
              item-key="title"
              item-props
            >
              <template #prepend="{ isActive, item }">
                <v-icon
                  v-if="!item.children"
                  icon="mdi:mdi-source-branch"
                />
                <v-icon
                  v-else-if="item.customIcon"
                  :icon="item.customIcon"
                />
                <v-icon
                  v-else-if="isActive"
                  icon="mdi:mdi-folder-open"
                />
                <v-icon
                  v-else
                  icon="mdi:mdi-folder"
                />
              </template>
            </v-treeview>
          </v-expansion-panel-text>
        </v-expansion-panel>
        <v-expansion-panel static>
          <v-expansion-panel-title>Tags</v-expansion-panel-title>
          <v-expansion-panel-text>
            <v-treeview
              :items="tagsProjectBranchesTree"
              expand-icon="mdi:mdi-tag-arrow-down"
              collapse-icon="mdi:mdi-tag-arrow-up"
              item-key="title"
              item-props
            >
              <template #prepend="{ item }">
                <v-icon
                  v-if="!item.children"
                  icon="mdi:mdi-tag"
                />
              </template>
            </v-treeview>
          </v-expansion-panel-text>
        </v-expansion-panel>
      </v-expansion-panels>
    </v-col>
    <v-col
      align="center"
      class="pa-0"
      cols="8"
    />
  </v-row>
</template>

<script lang="ts">
import { defineComponent } from "vue";

import { mapState } from "pinia";
import { useAppStore } from "../stores/app";
import { BranchType } from "../types/gitProject";
import { IGitBranch } from "../types/gitBranch";

interface IDirectory {
  name: string;
  children: IDirectory[];
  branches: string[];
}

interface IBranchTreeItem {
  id: number;
  title: string;
  children?: IBranchTreeItem[];
  customIcon?: string;
}

export default defineComponent({
  name: "ProjectPage",
  computed: {
    projectName(): string {
      return this.project?.directory.split("/").pop() || "";
    },
    ...mapState(useAppStore, {
      project: "getSelectedProject",
    }),
    localProjectBranchesTree() {
      return this.branchTreeToTreeview(BranchType.LOCAL);
    },
    remoteProjectBranchesTree() {
      const tree = this.branchTreeToTreeview(BranchType.REMOTE);
      if (!tree) {
        return [];
      }
      
      tree.forEach((root) => {
        root.customIcon = "mdi:mdi-cloud";
      });

      return tree;
    },
    tagsProjectBranchesTree() {
      return this.branchTreeToTreeview(BranchType.TAGS);
    },
  },
  mounted() {
    if (!this.project) {
      this.$router.push("/");
    } else {
      useAppStore().setTitle(this.projectName);
    }
  },
  methods: {
    makeBranchesTree(branchType: BranchType) {
      let tree: IDirectory = {
        name: "",
        children: [],
        branches: []
      };

      let branches: IGitBranch[] = [];
      if (branchType === BranchType.LOCAL) {
        branches = this.project?.localBranches || [];
      } else if (branchType === BranchType.REMOTE) {
        branches = this.project?.remoteBranches || [];
      } else if (branchType === BranchType.TAGS) {
        branches = this.project?.tags || [];
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
              branches: []
            };
            currentDirectory.children.push(newDirectory);
            currentDirectory = newDirectory;
          }
        }
        currentDirectory.branches.push(branchParts[branchParts.length - 1]);
      });

      return tree;
    },
    branchTreeToTreeview(branchType: BranchType) {
      const treeItems: IBranchTreeItem[] = [];
      const items = this.makeBranchesTree(branchType);

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
            title: branch,
          });
        });
      };

      addBranches(items);
      return treeItems[0].children;
    },
  }
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
