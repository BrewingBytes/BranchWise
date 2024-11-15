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
        {{ getCommitName }}
      </p>
      <v-divider />
    </v-col>
  </v-container>
  <v-container
    :height="getMaxHeight"
    style="overflow-y: scroll;"
  >
    <v-row style="overflow-y: scroll;">
      <v-col
        cols="10"
        style="overflow-y: scroll;"
      >
        <CommitDetailItem
          tag="Author"
          :value="getAuthorName"
        />
        <CommitDetailItem
          tag="Author Date"
          :value="getAuthorDate"
        />
        <CommitDetailItem
          tag="Committer"
          :value="getCommitterName"
        />
        <CommitDetailItem
          tag="Committer Date"
          :value="getCommitterDate"
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
          {{ getAuthorName[0] }}
        </v-avatar>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import { useProjectStore } from "@/stores/project";
import { getHash, NO_COMMIT } from "@/types/gitCommit";
import { mapState } from "pinia";
import { defineComponent } from "vue";
import CommitDetailItem from "@/components/Project/Commit/CommitDetailItem.vue";
import { getAuthorDate } from "@/types/gitAuthor";

export default defineComponent({
	name: "CommitDetails",
	components: {
		CommitDetailItem,
	},
	computed: {
		getCommitName() {
			return getHash(this.commit);
		},
		getAuthorName() {
			return this.commit.author.user.name + " <" + this.commit.author.user.email + ">";
		},
		getAuthorDate() {
			return getAuthorDate(this.commit.author, true);
		},
		getCommitterName() {
			return this.commit.committer.user.name + " <" + this.commit.committer.user.email + ">";
		},
		getCommitterDate() {
			return getAuthorDate(this.commit.committer, true);
		},
		commit() {
			if (this.getCommit) {
				return this.getCommit;
			}

			return NO_COMMIT;
		},
		getMaxHeight() {
			return window.innerHeight - 64 - 24;
		},
		...mapState(useProjectStore, ["getCommit"]),
	},
});
</script>
