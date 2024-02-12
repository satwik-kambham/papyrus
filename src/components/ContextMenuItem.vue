<script setup lang="ts"></script>

<template>
  <div class="cursor-pointer hover:bg-atom-bg text-nowrap">
    <slot>
      <div class="cursor-default hover:bg-atom-bg-dark">---</div>
    </slot>
  </div>
</template>
