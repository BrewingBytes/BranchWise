<template>
  <ConfirmationDialog />
</template>

<script lang="ts">
import ConfirmationDialog from '@/components/Dialogs/ConfirmationDialog.vue';
import { defineComponent } from 'vue';

export default defineComponent({
    name: 'DialogComponent',
    components: {
        ConfirmationDialog
    }
})
</script>
