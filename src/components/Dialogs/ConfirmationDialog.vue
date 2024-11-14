<template>
  <v-dialog
    v-model="isShowing"
    persistent
  >
    <v-card>
      <v-card-title class="headline">
        {{ confirmationDialog.title }}
      </v-card-title>
      <v-card-text>{{ confirmationDialog.message }}</v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn
          rounded
          @click="isShowing = false"
        >
          Cancel
        </v-btn>
        <v-btn
          rounded
          @click="confirmationDialog.onConfirm"
        >
          Confirm
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script lang="ts">
import { useDialogStore } from '@/stores/dialogs';
import { mapState } from 'pinia';
import { defineComponent } from 'vue';

export default defineComponent({
    name: 'ConfirmationDialog',
    computed: {
        isShowing: {
            get() {
                return this.confirmationDialog.isOpen;
            },
            set(value: boolean) {
                if (value === false) {
                useDialogStore().closeConfirmationDialog();
                }
            }
        },
        ...mapState(useDialogStore, ["confirmationDialog"])
    }
})
</script>
