<template>
  <v-navigation-drawer :model-value="isOpen">
    <v-list style="height:100%; display: flex; flex-direction: column">
      <v-list-item>
        <v-row>
          <v-col cols="3">
            <v-icon icon="mdi:mdi-source-branch" />
          </v-col>
          <v-col cols="9">
            <p class="text-h5">
              BranchWise
            </p>
          </v-col>
        </v-row>
      </v-list-item>
      <v-divider />
      <v-spacer />
      <v-list-item @click="closeMe">
        <v-row>
          <v-col cols="3">
            <v-icon icon="mdi:mdi-arrow-left" />
          </v-col>
          <v-col cols="9">
            <p class="text-h5">
              Close
            </p>
          </v-col>
        </v-row>
      </v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useAppStore } from "../stores/app";

export default defineComponent({
  name: "NavbarComponent",
  props: {
    isOpen: {
      type: Boolean,
      required: true,
    }
  },
  methods: {
    closeMe() {
      useAppStore().toggleNavbar();
    }
  }
});
</script>
