<template>
  <v-infinite-scroll
    :height="getHeight"
    :items="commits"
    @load="fetchMore"
  >
    <template
      v-for="commit in commits"
      :key="commit.hash"
    >
      <CommitListItem :commit="commit" />
    </template>
  </v-infinite-scroll>
</template>

<script lang="ts">
import CommitListItem from '@/components/Project/Commit/CommitListItem.vue';
import { useDialogStore } from '@/stores/dialogs';
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
      commits: 'getHistory',
    }),
  },
  methods: {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    async fetchMore({ done }: { done: any }) {
      try {
      await useProjectStore().fetchCommitHistory(30, this.commits[this.commits.length - 1].hash);
      done('ok');
      } catch (e) {
        done('fail');

        useDialogStore().showError(e);
      }
    }
  },
});
</script>
