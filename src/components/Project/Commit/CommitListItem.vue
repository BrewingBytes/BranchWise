<template>
  <v-row
    no-gutters
    align="center"
    style="height: 10vh;"
    class="hoverable pa-2"
    @click="setCommit"
  >
    <v-col
      style="height: 85%;"
      cols="2"
    >
      <v-avatar
        size="40"
        color="blue"
        class="text-white"
      >
        {{ getAuthorName[0] }}
      </v-avatar>
    </v-col>
    <v-col
      style="height: 95%;"
      cols="10"
    >
      <v-row
        no-gutters
        style="height: 50%;"
      >
        <p class="text-blue-grey">
          {{ getAuthorName }}
        </p>
        <v-spacer />
        <p class="text-blue-lighten-3">
          {{ getDate }}
        </p>
      </v-row>
      <v-row
        no-gutters
        style="height: 50%; overflow-y: hidden;"
      >
        <p class="text-blue-grey">
          {{ getHash }}
        </p>
        <v-spacer />
        
        {{ getMessage }}
      </v-row>
    </v-col>
  </v-row>
</template>

<script lang="ts">
import { useProjectStore } from '@/stores/project';
import { getHash, IGitCommit } from '@/types/gitCommit';
import moment from 'moment-timezone';
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'CommitListItem',
  props: {
    commit: {
      type: Object as () => IGitCommit,
      required: true,
    },
  },
  computed: {
    getAuthorName() {
      return this.commit.author.user.name;
    },
    getDate() {
        const timezone = this.commit.author.timezone.substring(0, 3) + ':' + this.commit.author.timezone.substring(3);
        return moment.unix(this.commit.author.date_seconds).utcOffset(timezone).format('YYYY-MM-DD');
    },
    getHash() {
      return getHash(this.commit);
    },
    getMessage() {
        // Get the first line of the commit message based on the screen size
        const message = this.commit.message.split('\n')[0];

        let maxMessageLength = screen.width / 80;
        return message.length > maxMessageLength ? message.slice(0, maxMessageLength) + '...' : message;
    }
  },
  methods: {
    setCommit() {
        useProjectStore().setCommit(this.commit.hash);
    }
  }
});
</script>

<style scoped>
.hoverable:hover {
    background-color: black;
    cursor: pointer;
    border-radius: 25px;
}
</style>
