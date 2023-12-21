<template>
    <div>
        <h1>用量查询</h1>
        <usage-header @on-receive="onReceive" />
        <usage-table v-if="page === 'table'" />
        <usage-chart v-else-if="page === 'chart'" />
    </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
const page = ref('table');
const onReceive = (params: string) => {
    console.log('onReceive', params);
    page.value = params;
};

</script>
<style scoped>
</style>