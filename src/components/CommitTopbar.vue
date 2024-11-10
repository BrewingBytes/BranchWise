<template>
  <v-container
    fluid
    style="color: red"
  >
    <p
      v-for="commit in history"
      :key="commit.message"
    >
      {{ commit.message }}
    </p>
  </v-container>
</template>

<script lang="ts">
import { mapState } from 'pinia';
import { defineComponent } from 'vue';
import { useProjectStore } from '../stores/project';

export default defineComponent({
  name: 'CommitTopbar',
  computed: {
    ...mapState(useProjectStore, ["history"])
  }
});
</script>
