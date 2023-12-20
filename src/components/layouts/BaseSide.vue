<template>
  <el-menu
    default-active="3"
    class="el-menu-vertical-demo"
    :collapse="isCollapse"
    @open="handleOpen"
    @close="handleClose"
  >
    <el-menu-item index="1"
    @click="goto_chat"
    >
      <template #title>
        <el-icon><ChatDotRound /></el-icon>
        <span>聊天</span>
      </template>
    </el-menu-item>
    <el-menu-item index="2"
    @click="goto_payment"
    >
      <el-icon><Tickets /></el-icon>
      <template #title>缴费</template>
    </el-menu-item>
    <el-menu-item index="3"
    @click="goto_usage"
    >
      <el-icon><Histogram /></el-icon>
      <template #title>用量</template>
    </el-menu-item>
    <el-menu-item index="4"
    @click="goto_setting"
    >
      <el-icon><setting /></el-icon>
      <template #title>设置</template>
    </el-menu-item>
  </el-menu>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import {
  Document,
  Menu as IconMenu,
  Location,
  Setting,
  ChatDotRound,
  Histogram,
  Tickets
  
} from '@element-plus/icons-vue'
import { invoke } from "@tauri-apps/api";

// 事件类型: onReceive
interface EmitType {
  (e: "onReceive", params: string): void
}

// 通过 emit 函数触发事件
const emit = defineEmits<EmitType>()


// 需要控制菜单展开收起的时候，可以使用 collapse 属性
const isCollapse = ref(false)
const handleOpen = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
const handleClose = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}

const goto_setting = () => {
  console.log('goto_setting')
  invoke('greet', { name: 'World' })
    .then((response) => {
        console.log(response);
    })
  emit('onReceive', 'setting')
}

const goto_chat = () => {
  console.log('goto_chat')
  emit('onReceive', 'chat')
}

const goto_payment = () => {
  console.log('goto_payment')
  emit('onReceive', 'payment')
}

const goto_usage = () => {
  console.log('goto_usage')
  emit('onReceive', 'usage')
}

var props: {
  page: {
    type: String,
    default: 'home'
  }
}

</script>

<style>
.el-menu-vertical-demo:not(.el-menu--collapse) {
  width: 200px;
  min-height: 400px;
}
</style>
