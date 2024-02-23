<script setup lang="ts"></script>

<template>
  <div class="px-2 cursor-pointer hover:bg-atom-bg-hover text-nowrap">
    <slot>
      <div class="-mx-2 px-2 cursor-default hover:bg-atom-bg-dark">---</div>
    </slot>
  </div>
</template>
