<template>
  <v-container>
    <AddProject />
  </v-container>
</template>

<script lang="ts">
import { defineComponent } from "vue";

import AddProject from "../components/AddProject.vue";

export default defineComponent({
    name: "IndexPage",
    components: {
        AddProject
    }
});
</script>
