<template>
    <div>
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
        <div class="button-group">
            <el-button @click="get_usage">按日期查询用量</el-button>
            <el-button @click="get_all_usage">总用量(扣除已缴费)</el-button>
            <el-button @click="get_raw_usage">总用量(税前,缴费前)</el-button>
        </div>
        <el-table :data="usage_data" style="width: 100%">
          <el-table-column prop="payer_qq" label="头像" width="180">
            <template #default="scope">
              <img :src="get_qq_avatar(scope.row.payer_qq)" style="width: 100px; height: 100px;" />
            </template>
          </el-table-column>
            <el-table-column prop="user_id" label="邮箱" width="180"></el-table-column>
            <el-table-column prop="name" label="用户" width="180"></el-table-column>
            <el-table-column prop="usage" label="用量($)" :formatter="row => parseFloat(row.usage).toFixed(3)"></el-table-column>
        </el-table>
    </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import dayjs from 'dayjs'
import { el } from "element-plus/es/locale";
import { ref } from 'vue';
import { ElMessage } from 'element-plus';

const start_day = ref('')
const end_day = ref('')

if (dayjs().hour() < 8) {
    start_day.value = dayjs().subtract(2, 'day').format('YYYY-MM-DD')
}
else
{
    start_day.value = dayjs().subtract(1, 'day').format('YYYY-MM-DD')
}

end_day.value = start_day.value
var usage_data = ref<any[]>([]);

const handleUsageResponse = (response: any) => {
    if (typeof response === 'object' && response !== null && 'data' in response && Array.isArray(response['data'])) {
        usage_data.value = response['data']
        console.log(usage_data.value)
    } else {
        console.error('Response is not an object or does not have a data property');
    }
}

const get_usage = () => {
    let start = is_all_usage.value ? 'all' : is_raw_usage.value ? 'raw' : dayjs(start_day.value).format('YYYY-MM-DD')
    let end = is_all_usage.value ? null : is_raw_usage.value ? null : dayjs(end_day.value).format('YYYY-MM-DD')

    if (start !== 'all' && start !== 'raw') {
        if (dayjs().hour() >= 8) {
            if (dayjs(start).isAfter(dayjs().subtract(1, 'day'))) {
                ElMessage({
                    message: '起始日期不合法, 用量数据更新时间为每天8点',
                    type: 'error'
                })
                return
            }
        }
        else {
            if (dayjs(start).isAfter(dayjs().subtract(2, 'day'))) {
                ElMessage({
                    message: '起始日期不合法, 用量数据更新时间为每天8点',
                    type: 'error'
                })
                return
            }
        }
        // 判断起始日期是否大于结束日期
        if (dayjs(start).isAfter(dayjs(end))) {
            ElMessage({
                message: '起始日期不得大于结束日期',
                type: 'error'
            })
            return
        }
    }

    ElMessage({
        message: '正在查询用量数据',
        type: 'info',
        duration: 500
    })

    invoke('get_usage', { start, end })
    .then((response) => {
        handleUsageResponse(response);
        ElMessage({
            message: '用量数据查询成功',
            type: 'success',
            duration: 500
        })
    })
    .catch((error) => {
        console.error('Error occurred while fetching usage:', error);
    });
    is_all_usage.value = false
    is_raw_usage.value = false
}

const is_all_usage = ref(false)
const is_raw_usage = ref(false)

const get_all_usage = () => {
    is_all_usage.value = true
    get_usage();
}

const get_raw_usage = () => {
    is_raw_usage.value = true
    get_usage();
}

// 一个函数，传入qq，返回qq头像的url
const get_qq_avatar = (qq: string) => {
    return `https://q1.qlogo.cn/g?b=qq&nk=${qq}&s=100`
}

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