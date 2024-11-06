<template>
  <v-list-item>
    <v-row>
      <v-col cols="3">
        <v-icon
          :color="prependColor"
          :icon="prependIcon"
          :class="{disabled: !hasPrependClick}"
          @click="$emit('prependClick')"
        />
      </v-col>
      <v-col :cols="appendIcon ? 6 : 9">
        <p
          :style="{color: textColor}"
          class="text-h5"
        >
          {{ text }}
        </p>
      </v-col>
      <v-col
        v-if="appendIcon"
        cols="3"
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

<script lang="ts">
import { defineComponent, getCurrentInstance } from 'vue';

export default defineComponent({
  name: 'SidebarItem',
  props: {
    prependIcon: {
        type: String,
        required: true
    },
    prependColor: {
        type: String,
        required: true
    },
    text: {
        type: String,
        required: true
    },
    textColor: {
        type: String,
        default: 'white'
    },
    appendIcon: {
        type: String,
        default: ''
    },
    appendColor: {
        type: String,
        default: ''
    }
  },
  emits: ['prependClick', 'appendClick'],
  data() {
    return {
      hasPrependClick: false,
      hasAppendClick: false
    }
  },
  mounted() {
    const allProps = getCurrentInstance()?.vnode.props;
    if (allProps?.onAppendClick) {
      this.hasAppendClick = true;
    }

    if (allProps?.onPrependClick) {
      this.hasPrependClick = true;
    }
  },
});
</script>

<style scoped>
.disabled {
    pointer-events: none;
}
</style>
