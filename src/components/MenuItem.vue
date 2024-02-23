<script setup lang="ts">
import { Popover, PopoverButton, PopoverPanel } from "@headlessui/vue";
const props = defineProps<{
  name: string;
}>();
</script>

<template>
  <div class="">
    <Popover class="relative">
      <PopoverButton>
        <div class="px-4 py-1 cursor-pointer hover:bg-atom-bg-hover">
          {{ props.name }}
        </div>
      </PopoverButton>
      <PopoverPanel class="absolute z-50 bg-atom-bg-dark">
        <PopoverButton><slot></slot></PopoverButton>
      </PopoverPanel>
    </Popover>
  </div>
</template>
