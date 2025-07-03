<script setup lang="ts">
import { useProjectStore } from "@/stores/project";
import { getAuthorDate } from "@/types/gitAuthor";
import { getHash, IGitCommit } from "@/types/gitCommit";
import { computed } from "vue";

const props = defineProps({
  commit: {
    type: Object as () => IGitCommit,
    required: true,
  }
});

const authorName = computed(() => props.commit.author.user.name);
const date = computed(() => getAuthorDate(props.commit.author));
const hash = computed(() => getHash(props.commit));
const message = computed(() => {
  // Get the first line of the commit message based on the screen size
  const message = props.commit.message.split("\n")[0];

  const maxMessageLength = screen.width / 80;
  return message.length > maxMessageLength ? message.slice(0, maxMessageLength) + "..." : message;
});

const commitClass = computed(() => {
  let className = "hoverable pa-2 ml-2 mr-1 mb-2";

    if (props.commit.hash === useProjectStore().getCommit?.hash) {
      className += " selected";
    }

    return className;
});

function setCommit() {
  useProjectStore().setCommit(props.commit.hash);
}
</script>

<template>
  <v-row
    no-gutters
    align="center"
    style="height: 10vh;"
    :class="commitClass"
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
        {{ authorName[0] }}
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
          {{ authorName }}
        </p>
        <v-spacer />
        <p class="text-blue-lighten-3">
          {{ date }}
        </p>
      </v-row>
      <v-row
        no-gutters
        style="height: 50%; overflow-y: hidden;"
      >
        <p class="text-blue-grey">
          {{ hash }}
        </p>
        <v-spacer />
        
        {{ message }}
      </v-row>
    </v-col>
  </v-row>
</template>

<style scoped>
.hoverable:hover {
    background-color: black;
    cursor: pointer;
    border-radius: 25px;
}

.selected {
    background-color: #112233;
    border-radius: 25px;
}

.selected:hover {
    background-color: #112233;
    cursor: default !important;
}

</style>
