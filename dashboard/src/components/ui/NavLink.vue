<script setup lang="ts">
import AppLink from '@/components/AppLink.vue'
</script>

<template>
  <AppLink v-bind="$props"
    class="inline-flex cursor-pointer items-center gap-2 rounded-lg px-3 py-2 text-gray-500 hover:text-gray-700"
    activeClass="bg-gray-100 text-gray-700 hover:bg-gray-100"
    inactiveClass="">
    <slot />
  </AppLink>
</template>

