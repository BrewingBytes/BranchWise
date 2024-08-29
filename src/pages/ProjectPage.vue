<template>
  <v-row class="fill-height" no-gutters>
    <v-col align="center" class="pa-0" cols="3" style="border-right: 1px solid #e0e0e0;">
    </v-col>
    <v-col align="center" class="pa-0" cols="9">
    </v-col>
  </v-row>
</template>

<script lang="ts">
import { defineComponent } from "vue";

import { mapState } from "pinia";
import { useAppStore } from "../stores/app";
import { BranchType } from "../types/gitProject";

interface IDirectory {
  name: string;
  children: IDirectory[];
  branches: string[];
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
      return this.makeBranchesTree(BranchType.LOCAL);
    },
    remoteProjectBranchesTree() {
      return this.makeBranchesTree(BranchType.REMOTE);
    },
    tagsProjectBranchesTree() {
      return this.makeBranchesTree(BranchType.TAGS);
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

      let branches: string[] = [];
      if (branchType === BranchType.LOCAL) {
        branches = this.project?.localBranches || [];
      } else if (branchType === BranchType.REMOTE) {
        branches = this.project?.remoteBranches || [];
      } else if (branchType === BranchType.TAGS) {
        branches = this.project?.tags || [];
      }

      branches.forEach((branch) => {
        const branchParts = branch.split("/");
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
    }
  }
});
</script>
