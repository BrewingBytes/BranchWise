<script setup lang="ts">
import { useDialogStore } from "@/stores/dialogs";
import { storeToRefs } from "pinia";
import { computed } from "vue";

const isShowing = computed({
	get: () => useDialogStore().confirmationDialog.isOpen,
	set: (val: boolean) => {
		if (!val) {
			useDialogStore().closeConfirmationDialog();
		}
	}
});

const { confirmationDialog } = storeToRefs(useDialogStore());
</script>

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
