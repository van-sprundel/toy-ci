<script setup lang="ts">
import { RouterView } from 'vue-router'
import NavLink from '@/components/ui/NavLink.vue'
</script>

<template>
    <header>
        <div class="wrapper">
            <nav>
                <NavLink to="/">Dashboard</NavLink>
            </nav>
        </div>
    </header>
    <RouterView />
</template>

