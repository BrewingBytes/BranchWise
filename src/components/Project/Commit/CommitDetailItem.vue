<template>
  <v-container class="pa-0">
    <v-row
      align="center"
      class="pa-0"
      no-gutters
    >
      <v-col
        align="center"
        class="pa-0 pr-2 text-blue-grey"
        cols="4"
        style="text-align: right; text-overflow: clip;"
      >
        {{ tag }}
      </v-col>
      <v-col
        align="center"
        class="pa-0 pl-2 text-blue-lighten-4"
        cols="8"
        style="text-align: left; text-overflow: clip; overflow: hidden;"
      >
        {{ value }}
      </v-col>
    </v-row>
    <v-divider />
  </v-container>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'CommitDetailItem',
  props: {
    tag: {
      type: String,
      required: true,
    },
    value: {
      type: String,
      required: true,
    },
  },
});
</script>
