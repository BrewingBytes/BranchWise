<script setup lang="ts">
import CommitListItem from "@/components/Project/Commit/CommitListItem.vue";
import { useDialogStore } from "@/stores/dialogs";
import { useProjectStore } from "@/stores/project";
import { computed } from "vue";
import { InfiniteScrollStatus } from "vuetify/lib/components/VInfiniteScroll/VInfiniteScroll.mjs";

const commits = computed(() => useProjectStore().getHistory);
const branch = computed(() => useProjectStore().getBranch?.name);
const height = computed(() => window.innerHeight - 64 - 24);

async function fetchMore({ done }: { done: (status: InfiniteScrollStatus) => void }) {
	try {
		await useProjectStore().fetchCommitHistory(30, commits.value[commits.value.length - 1].hash);
		done("ok");
	} catch (e) {
		done("error");

		useDialogStore().showError(e);
	}
};
</script>

<template>
  <v-container
    class="pa-0"
    height="24px"
  >
    <v-col
      class="pa-0"
      no-gutters
    >
      <p class="text-blue-grey">
        {{ branch }}
      </p>
      <v-divider />
    </v-col>
  </v-container>
  <v-infinite-scroll
    :height="height"
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
