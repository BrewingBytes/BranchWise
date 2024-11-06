<template>
  <ConfirmationDialog />
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import ConfirmationDialog from './Dialogs/ConfirmationDialog.vue';

export default defineComponent({
    name: 'DialogComponent',
    components: {
        ConfirmationDialog
    }
})
</script>
