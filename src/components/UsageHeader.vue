<template>
    <el-menu
      :default-active="activeIndex"
      class="el-menu-demo"
      mode="horizontal"
      @select="handleSelect"
    >
      <el-menu-item index="1"
      @click="goto_table"
      >表格</el-menu-item>
      <el-menu-item index="2"
      @click="goto_chart"
      >图表</el-menu-item>
    </el-menu>
    <div class="h-6" />
  </template>
  
  <script lang="ts" setup>
  import { ref } from 'vue'
  
  const activeIndex = ref('1')
  const activeIndex2 = ref('1')
  const handleSelect = (key: string, keyPath: string[]) => {
    console.log(key, keyPath)
  }

// 事件类型: onReceive
interface EmitType {
  (e: "onReceive", params: string): void
}

// 通过 emit 函数触发事件
const emit = defineEmits<EmitType>()

const goto_table = () => {
    emit('onReceive', 'table')
}

const goto_chart = () => {
    emit('onReceive', 'chart')
}

  </script>
  