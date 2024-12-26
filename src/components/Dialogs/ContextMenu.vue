<template>
  <v-menu 
    :model-value="isShowing" 
    :target="[posX, posY]"
    @update:model-value="closeMenu"
  >
    <v-list>
      <v-list-item>
        <v-list-item-title @click="checkout">
          Checkout
        </v-list-item-title>
      </v-list-item>
    </v-list>
  </v-menu>
</template>

<script lang="ts">
import { useDialogStore } from "@/stores/dialogs";
import { useProjectStore } from "@/stores/project";
import { TauriCommands } from "@/types/tauri";
import { invoke } from "@tauri-apps/api/core";
import { mapState } from "pinia";
import { defineComponent } from "vue";

export default defineComponent({
	name: "ContextMenu",
	computed: {
		isShowing: {
			get() {
				return this.contextMenu.isOpen;
			},
			set(value: boolean) {
				if (value === false) {
					useDialogStore().closeContextMenu();
				}
			}
		},
		posX() {
			return useDialogStore().contextMenu.position.x;
		},
		posY() {
			return useDialogStore().contextMenu.position.y;
		},
		...mapState(useDialogStore, ["contextMenu"])
	},
	methods: {
		closeMenu() {
			useDialogStore().closeContextMenu();
		},
		checkout() {
			invoke(TauriCommands.CheckoutCommit, {
				project: useProjectStore().getSelectedProject,
				hash: useDialogStore().contextMenu.commitHash
			});
		}
	}
});
</script>
