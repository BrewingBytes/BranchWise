<script setup lang="ts">
import { useDialogStore } from "@/stores/dialogs";
import { useProjectStore } from "@/stores/project";
import { TauriCommands } from "@/types/tauri";
import { invoke } from "@tauri-apps/api/core";
import { storeToRefs } from "pinia";
import { computed } from "vue";

const { contextMenu } = storeToRefs(useDialogStore());
const isShowing = computed({
	get: () => contextMenu.value.isOpen,
	set: (value: boolean) => {
		if (!value) {
			useDialogStore().closeContextMenu();
		}
	}
});

const posX = computed(() => contextMenu.value.position.x);
const posY = computed(() => contextMenu.value.position.y);

const closeMenu = () => useDialogStore().closeContextMenu();
const checkout = () =>
	invoke(TauriCommands.CheckoutCommit, {
		project: useProjectStore().getSelectedProject,
		hash: useDialogStore().contextMenu.commitHash
	});
</script>

<template>
  <v-menu 
    :model-value="isShowing" 
    :target="[posX, posY]"
    @update:model-value="closeMenu"
  >
    <v-list>
      <v-list-item>
        <v-list-item-title
          class="pointer"
          @click="checkout"
        >
          Checkout
        </v-list-item-title>
      </v-list-item>
    </v-list>
  </v-menu>
</template>

<style scoped>
.pointer {
	cursor: pointer;
}
</style>
