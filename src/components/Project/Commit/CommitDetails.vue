<script setup lang="ts">
import CommitDetailItem from "@/components/Project/Commit/CommitDetailItem.vue";
import { useProjectStore } from "@/stores/project";
import { getAuthorDate } from "@/types/gitAuthor";
import { getShortHash, NO_COMMIT } from "@/types/gitCommit";
import { storeToRefs } from "pinia";
import { computed } from "vue";

const { getCommit } = storeToRefs(useProjectStore());

const commit = computed(() => getCommit.value ?? NO_COMMIT);
const commitName = computed(() => getShortHash(commit.value));

const authorName = computed(() => `${commit.value.author.user.name} <${commit.value.author.user.email}>`);
const authorDate = computed(() => getAuthorDate(commit.value.author, true));

const committerName = computed(() => `${commit.value.committer.user.name} <${commit.value.committer.user.email}>`);
const committerDate = computed(() => getAuthorDate(commit.value.committer, true));

const maxHeight = computed(() => window.innerHeight - 64 - 24);
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
        {{ commitName }}
      </p>
      <v-divider />
    </v-col>
  </v-container>
  <v-container
    :height="maxHeight"
    style="overflow-y: scroll;"
  >
    <v-row style="overflow-y: scroll;">
      <v-col
        cols="10"
        style="overflow-y: scroll;"
      >
        <CommitDetailItem
          tag="Author"
          :value="authorName"
        />
        <CommitDetailItem
          tag="Author Date"
          :value="authorDate"
        />
        <CommitDetailItem
          tag="Committer"
          :value="committerName"
        />
        <CommitDetailItem
          tag="Committer Date"
          :value="committerDate"
        />
        <CommitDetailItem
          tag="Hash"
          :value="commit.hash"
        />
        <CommitDetailItem
          v-for="parentHash in commit.parent_hashes"
          :key="parentHash"
          tag="Parent Hash"
          :value="parentHash"
        />
        <CommitDetailItem
          tag="Tree Hash"
          :value="commit.tree_hash"
        />
        <CommitDetailItem
          tag="Message"
          :value="commit.message"
        />
      </v-col>
      <v-col cols="2">
        <v-avatar
          size="40"
          color="blue"
          class="text-white"
        >
          {{ authorName[0] }}
        </v-avatar>
      </v-col>
    </v-row>
  </v-container>
</template>
