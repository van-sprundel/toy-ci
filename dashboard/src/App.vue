<script setup lang="ts">
import { RouterView } from 'vue-router'
import NavLink from '@/components/ui/NavLink.vue'
</script>

<template>
    <header>
        <nav class="flex items-center gap-2 text-sm font-medium">
            <NavLink to="/">Dashboard</NavLink>
        </nav>
    </header>
    <RouterView />
</template>
