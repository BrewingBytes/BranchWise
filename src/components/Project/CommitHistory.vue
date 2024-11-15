<template>
  <v-virtual-scroll
    :height="getHeight"
    :items="commits"
  >
    <template #default="{ item }">
      <CommitListItem :commit="item" />
    </template>
  </v-virtual-scroll>
</template>

<script lang="ts">
import CommitListItem from '@/components/Project/Commit/CommitListItem.vue';
import { useProjectStore } from '@/stores/project';
import { mapState } from 'pinia';
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'CommitHistory',
  components: {
    CommitListItem,
  },
  computed: {
    getHeight() {
      return window.innerHeight - 64;
    },
    ...mapState(useProjectStore, {
      commits: 'history',
    }),
  },
});
</script>
