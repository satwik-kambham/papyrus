<script setup lang="ts">
import MenuBar from './components/MenuBar.vue';
import EditorPanel from './components/EditorPanel.vue';
import SidePanel from './components/SidePanel.vue';
import StatusBar from './components/StatusBar.vue';
</script>

<template>
    <div class="bg-slate-700 text-slate-100 h-screen flex flex-col">
        <div class="">
            <MenuBar />
        </div>
        <div class="flex grow">
            <div class="w-1/6">
                <SidePanel />
            </div>
            <div class="grow">
                <EditorPanel />
            </div>
        </div>
        <div class="">
            <StatusBar />
        </div>
    </div>
</template>
