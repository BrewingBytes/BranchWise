<template>
  <ConfirmationDialog />
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import ConfirmationDialog from '@/components/Dialogs/ConfirmationDialog.vue';

export default defineComponent({
    name: 'DialogComponent',
    components: {
        ConfirmationDialog
    }
})
</script>
