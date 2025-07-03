<script setup lang="ts">
import { PrependVariant } from "@/enums/prependVariant";
import { getCurrentInstance, ref } from "vue";

defineEmits(["prependClick", "appendClick"]);
defineProps({
  prependIcon: {
    type: String,
    required: true
  },
  prependVariant: {
    type: String as () => PrependVariant,
    default: PrependVariant.ICON
  },
  prependColor: {
    type: String,
    default: "white"
  },
  text: {
    type: String,
    required: true
  },
  subtitle: {
    type: String,
    default: ""
  },
  textColor: {
    type: String,
    default: "white"
  },
  appendIcon: {
    type: String,
    default: ""
  },
  appendColor: {
    type: String,
    default: "white"
  }
});

const hasPrependClick = ref(false);
const hasAppendClick = ref(false);

const allProps = getCurrentInstance()?.vnode.props;
if (allProps?.onAppendClick) {
  hasAppendClick.value = true;
}

if (allProps?.onPrependClick) {
  hasPrependClick.value = true;
}
</script>

<template>
  <v-list-item class="pa-0 pt-4 pb-4">
    <v-row
      no-gutters
      align="center"
    >
      <v-col
        cols="3"
        justify="center"
        align="center"
      >
        <template v-if="prependVariant === PrependVariant.ICON">
          <v-icon
            :color="prependColor"
            :icon="prependIcon"
            :class="{disabled: !hasPrependClick}"
            @click="$emit('prependClick')"
          />
        </template>
        <template v-else>
          <v-avatar
            :class="{disabled: !hasPrependClick}"
            @click="$emit('prependClick')"
          >
            <v-img :src="prependIcon" />
          </v-avatar>
        </template>
      </v-col>
      <v-col
        :cols="appendIcon ? 6 : 9"
        :align="appendIcon ? 'center' : 'flex-start'"
      >
        <template v-if="!subtitle">
          <p
            :style="{color: textColor}"
            class="text-h5"
          >
            {{ text }}
          </p>
        </template>
        <template v-else>
          <p
            :style="{color: textColor}"
            class="text-subtitle-1"
          >
            {{ text }}
          </p>
          <p
            :style="{color: textColor}"
            class="text-caption font-weight-thin"
          >
            {{ subtitle }}
          </p>
        </template>
      </v-col>
      <v-col
        v-if="appendIcon"
        cols="3"
        align="center"
        justify="center"
      >
        <v-icon
          :color="appendColor"
          :icon="appendIcon"
          :class="{disabled: !hasAppendClick}"
          @click="$emit('appendClick')"
        />
      </v-col>
    </v-row>
  </v-list-item>
</template>

<style scoped>
.disabled {
    pointer-events: none;
}
</style>
