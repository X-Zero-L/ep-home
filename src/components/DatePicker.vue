<template>
    <div class="demo-date-picker">
      <div class="container">
        <div class="block">
            <span class="demonstration">起始日期</span>
            <el-date-picker
              v-model="start_day"
              type="date"
              placeholder="Pick one or more dates"
            />
          </div>
      </div>
      <div class="container">
        <div class="block">
          <span class="demonstration">结束日期</span>
          <el-date-picker
            v-model="end_day"
            type="date"
            placeholder="Pick one or more dates"
          />
        </div>
      </div>
    </div>
</template>
  <script lang="ts" setup>
  import { ref } from 'vue'
  
  const start_day = ref('')
  const end_day = ref('')

  </script>
  <style scoped>
  .demo-date-picker {
    display: flex;
    width: 100%;
    padding: 0;
    flex-wrap: wrap;
  }
  
  .demo-date-picker .block {
    padding: 30px 0;
    text-align: center;
    border-right: solid 1px var(--el-border-color);
    flex: 1;
  }
  .demo-date-picker .block:last-child {
    border-right: none;
  }
  
  .demo-date-picker .container {
    flex: 1;
    border-right: solid 1px var(--el-border-color);
  }
  .demo-date-picker .container .block {
    border-right: none;
  }
  .demo-date-picker .container .block:last-child {
    border-top: solid 1px var(--el-border-color);
  }
  .demo-date-picker .container:last-child {
    border-right: none;
  }
  
  .demo-date-picker .demonstration {
    display: block;
    color: var(--el-text-color-secondary);
    font-size: 14px;
    margin-bottom: 20px;
  }
  </style>
  