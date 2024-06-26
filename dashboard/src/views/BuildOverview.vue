<template>
    <div class="bg-gray-900 text-white p-4 rounded-lg shadow-lg">
        <h2 class="text-lg font-semibold mb-4">Build Logs</h2>
        <div class="bg-gray-800 text-gray-200 p-4 rounded-lg overflow-y-scroll max-h-80">
            <pre>
                <code v-if="logs.length" class="whitespace-pre-wrap">
                    <span v-for="(log, index) in logs" :key="index" class="block">{{ log }}</span>
                </code>
                <code v-else>
                    Loading logs...
                </code>
            </pre>
        </div>
    </div>
</template>
<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue';
import { useRoute } from 'vue-router';

const logs = ref<string[]>([]);
let eventSource: EventSource | null = null;

const route = useRoute();
const buildId = ref<string>(route.params.buildId);

onMounted(() => {
    const url = `http://127.0.0.1:3000/builds/${buildId.value}/sse`;

    eventSource = new EventSource(url);

    eventSource.onmessage = (event) => {
        logs.value.push(event.data);
    };

    eventSource.onerror = (error) => {
        console.error("EventSource failed:", error);
        eventSource?.close();
    };
});

onBeforeUnmount(() => {
    if (eventSource) {
        eventSource.close();
    }
});
</script>

