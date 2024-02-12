<script setup lang="ts"></script>

<template>
  <div
    class="bg-atom-bg-dark h-full select-none flex flex-col border-x-[1px] border-atom-black"
  >
    <div class="border-atom-bg border-b-[0.5px] text-center p-1">Project</div>
    <div class="grow overflow-auto custom-scrollbar">
      <slot></slot>
    </div>
  </div>
</template>
