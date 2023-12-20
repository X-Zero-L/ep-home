<template>
  <el-config-provider namespace="ep">
   <!-- <BaseHeader />
  -->  <div class="flex main-container">
      <BaseSide @onReceive="onReceive" />
      <div w="full" py="4">
      <!--  <Logos my="4" />
-->      <!--  <HelloWorld msg="Hello Vue 3 + Element Plus + Vite" /> -->
        <Home v-if="page === 'home'" />
        <Chat v-else-if="page === 'chat'" />
        <Usage v-else-if="page === 'usage'" />
        <Payment v-else-if="page === 'payment'" />
        <Setting v-else-if="page === 'setting'" />
      </div>
    </div>
  </el-config-provider>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const page = ref('home');

const onReceive = (params: string) => {
  console.log('onReceive', params);
  page.value = params;
};

</script>

<style>
#app {
  text-align: center;
  color: var(--ep-text-color-primary);
}

.main-container {
  height: calc(100vh - var(--ep-menu-item-height) - 3px);
}
</style>
