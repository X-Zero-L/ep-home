<template>
    <div>
        <div :id="props.model_name" style="width: 100%;height:600px;"></div>
    </div>
</template>

<script lang="ts" setup>
import { ref,onMounted, watch, onBeforeUnmount } from "vue";
import * as echarts from 'echarts';

const props = defineProps({
    names: {
        type: Array,
        default: () => ['怡宝', '查理', '三来', 'Sa酱']
    },
    model_name: {
        type: String,
        default: () => 'gpt-3.5-1106'
    },
    x_data: {
        type: Array,
        default: () => ['1', '2', '3', '4', '5', '6', '7']
    },
    series_data: {
        type: Array,
        default: () => [120, 132, 101, 134, 90, 230, 210]
    }
});

var chart: echarts.ECharts;

const initChart = () => {
    const chartDom = document.getElementById(props.model_name);
    if (chartDom) {
        console.log('initChart');
        chart = echarts.init(chartDom);
    }
    else {
        console.log('chartDom is null');
    }
}

const option_ref = ref<any>(
  {
  title: {
    text: props.model_name,
    left: 'center',
  },
  tooltip: {
    trigger: 'axis'
  },
  legend: {
    data: props.names,
    bottom: 'bottom',
    axislabel: {
      rotate: 45,
    },
    type: 'scroll',
  },
  grid: {
    left: '3%',
    right: '4%',
    bottom: '3%',
    containLabel: true
  },
  toolbox: {
    feature: {
      saveAsImage: { show: true },
      dataView: { show: true },
      restore: { show: true },
      magicType: { show: true, type: ['line', 'bar'] },
    }
  },
  xAxis: {
    type: 'category',
    boundaryGap: false,
    data: props.x_data,
    axislabel: {
      interval: 0,
      rotate: 45,
    }
  },
  yAxis: {
    type: 'value'
  },
  series: props.series_data
}
);

const usage_data = ref<any[]>([]);

const usage_by_model = ref<any>({});

onMounted (() => {
    chart = echarts.init(document.getElementById(props.model_name));
    chart.setOption(option_ref.value);
});

// 页面销毁时销毁echarts
onBeforeUnmount(() => {
    if (chart) {
        chart.dispose();
    }
    else {
        console.log('chart is null');
    }
});

// 监听props变化
watch(() => props.series_data, (newVal, oldVal) => {
    console.log('props.series_data changed', newVal, oldVal);
    option_ref.value.series = newVal;
    option_ref.value.legend.data = props.names;
    option_ref.value.title.text = props.model_name;
    option_ref.value.xAxis.data = props.x_data;
    chart = echarts.init(document.getElementById(props.model_name));
    chart.setOption(option_ref.value);
});


</script>

<style scoped>
</style>