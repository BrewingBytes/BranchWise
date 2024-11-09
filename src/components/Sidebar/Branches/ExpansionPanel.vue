<template>
  <v-expansion-panel static>
    <v-expansion-panel-title>{{ title }}</v-expansion-panel-title>
    <v-expansion-panel-text>
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
          />
        </template>
        <template #title="{ item }">
          <p
            v-if="!item.children"
            @click="setCurrentBranch"
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
import { defineComponent } from 'vue';
import { IBranchTreeItem } from '../../../types/branchTreeItem';

export default defineComponent({
  name: 'ExpansionPanel',
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
      default: 'mdi-folder',
    },

    collapseIcon: {
      type: String,
      default: 'mdi-folder-open',
    },
    itemIcon: {
      type: String,
      default: 'mdi-source-branch',
    },
    customIcon: {
      type: String,
      default: '',
    },
  }
});
</script>
