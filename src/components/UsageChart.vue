<template>
    <div>
        <p>支持按日期或月份查询图表,选择任意日期或月份后即可获取图表</p>
        <div class="chart-date-picker">
            <div class="container">
              <div class="block">
                  <span class="demonstration"></span>
                  <el-date-picker
                    v-model="day"
                    type="date"
                    placeholder="选择要查询的日期"
                    @update:model-value="get_daily_usage"
                  />
                </div>
            </div>
            <div class="container">
              <div class="block">
                <span class="demonstration"></span>
                <el-date-picker
                  v-model="month"
                  type="month"
                  placeholder="选择要查询的月份"
                  @update:model-value="get_monthly_usage"
                />
              </div>
            </div>
        </div>
    </div>
    <div>
        <el-row :gutter="20">
            <el-col :span="12" v-for="model in models_data" :key="model.model">
                <GPTUsageChart :model_name="model.model" :names="model.names" :x_data="model.x_data" :series_data="model.series_data" />
            </el-col>
        </el-row>
    </div>
</template>
<script lang="ts" setup>
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";
import { ElMessage, dayjs } from 'element-plus';

const day = ref('')
const month = ref('')

const models_data = ref<any[]>([]);

const get_daily_usage = async () => {
    // 8点后支持查询前一天的用量,8点前支持查询前两天的用量
    if (dayjs().hour() < 8) {
        if (dayjs(day.value).isAfter(dayjs().subtract(2, 'day'), 'day')) {
            ElMessage.error('日期非法,每日8点更新上一日用量')
            day.value = ""
            return;
        }
    }
    else
    {
        if (dayjs(day.value).isAfter(dayjs().subtract(1, 'day'), 'day')) {
            ElMessage.error('日期非法,每日8点更新上一日用量')
            day.value = ""
            return;
        }
    }
    ElMessage(
        {
            message: '正在查询,请稍后...',
            type: 'info',
            duration: 500
        }
    )
    let day_str = dayjs(day.value).format('YYYY-MM-DD')
    const res = await invoke("get_daily_usage", { day: day_str});
    await handleUsageResponse(res)
    ElMessage(
        {
            message: '查询成功',
            type: 'success',
            duration: 500
        }
    )
    return res;
};

const get_monthly_usage = async () => {
    // 月份不能超过当前月份,且不能早于2023-03
    if (dayjs(month.value).isAfter(dayjs(), 'month') || dayjs(month.value).isBefore(dayjs('2023-03'))) {
        month.value = ""
        ElMessage.error('不能超过当前月份,且不能早于2023年3月')
        return;
    }
    ElMessage(
        {
            message: '正在查询,请稍后...',
            type: 'info',
            duration: 500
        }
    )
    let month_str = dayjs(month.value).format('YYYY-MM')
    let res = await invoke("get_monthly_usage", { month: month_str});
    res = await handleUsageResponse(res)
    ElMessage(
        {
            message: '查询成功',
            type: 'success',
            duration: 500
        }
    )
    return res;
};

// 处理用量数据
const handleUsageResponse = async (response: any) => {
    if (typeof response === 'object' && response !== null && 'data' in response && Array.isArray(response['data'])) {
        let data = response['data']
        let names: any[] = []
        let x_data = new Set()
        let series_data: { [key: string]: { [key: string]: number[] } } = {};
        let models = new Set()
        for (let i = 0; i < data.length; i++) {
            let model = data[i].logs
            let uname = data[i].name
            names.push(uname)
            for (let key in model) {
                models.add(key)
                for (let j = 0; j < model[key].length; j++) {
                    x_data.add(model[key][j].x_axis)
                    if (series_data[key] === undefined) {
                          series_data[key] = {};  
                    }
                    if (series_data[key][uname] === undefined) {
                        series_data[key][uname] = [];
                    }
                    series_data[key][uname].push(model[key][j].n_generated_tokens_total + model[key][j].n_context_tokens_total);
                }
            }
        }
        models_data.value = []
        await Promise.resolve()
        for (let key in series_data) {
            let series = []
            for (let uname in series_data[key]) {
                series.push({
                    name: uname,
                    type: 'line',
                    stack: 'tokens',
                    data: series_data[key][uname]
                })
            }
            models_data.value.push({
                model: key,
                names: names,
                x_data: Array.from(x_data),
                series_data: series
            })
        }
        models_data.value.sort((a, b) => {
            return a.model.localeCompare(b.model)
        })
    } else {
        console.error('Response is not an object or does not have a data property');
    }
}


</script>

<style scoped>
.chart-date-picker {
    display: flex;
    width: 100%;
    padding: 0;
    flex-wrap: wrap;
  }
  
  .chart-date-picker .block {
    padding: 30px 0;
    text-align: center;
    border-right: solid 1px var(--el-border-color);
    flex: 1;
  }
  .chart-date-picker .block:last-child {
    border-right: none;
  }
  
  .chart-date-picker .container {
    flex: 1;
    border-right: solid 1px var(--el-border-color);
  }
  .chart-date-picker .container .block {
    border-right: none;
  }
  .chart-date-picker .container .block:last-child {
    border-top: solid 1px var(--el-border-color);
  }
  .chart-date-picker .container:last-child {
    border-right: none;
  }
  
  .chart-date-picker .demonstration {
    display: block;
    color: var(--el-text-color-secondary);
    font-size: 14px;
    margin-bottom: 20px;
  }
</style>