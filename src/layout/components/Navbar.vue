<template>
    <div class="flex mt-2">
        <img src="../../assets/logo.png" class="logo" style="width: 32px;height: 32px;" />
        <div class="title" style="line-height: 32px;">ECTD Manager</div>
        <span class="title ml-6" style="line-height: 32px;">运行环境:</span>
        <el-select class="title ml-2" v-model="selectedValue" @change="handleSelectChange" placeholder="Select" default-first-option="true" filterable="true">
            <el-option v-for="item in tableData.etcd" style="width: 600px;" :key="item.name" :label="item.name" :value="item.name">
                <span style="float: left">{{ item.name }}</span>
                <span style="float: right;color: var(--el-text-color-secondary);font-size: 13px; ">{{ item.address }}</span>
            </el-option>
        </el-select>
        <el-divider direction="vertical" class="divider" />
        <el-menu :default-active="activeNavMenuIndex" class="nav" mode="horizontal" @select="handleSelect">
            <template v-for="item in naviMenu" :key="item.path">
                <el-menu-item :index="item.path">
                    {{ item.meta?.title }}
                </el-menu-item>
            </template>
        </el-menu>
    </div>
</template>
<script lang="ts" setup>
import { ref, computed, onMounted} from 'vue';
import { useRouter } from 'vue-router';
import { query_cluster,queryinstance,saveinstance } from '@/apis/api';


const activeNavMenuIndex = ref('/runtime/index')
const $router = useRouter();

const naviMenu = computed(() => {
    const nav = $router.options.routes.filter((v) => v.meta?.isNav) || [];
    return nav;
});

//Hanle menu selection
const handleSelect = (path: string) => {
    $router.push(path);
};


const selectedValue = ref('')

const tableData = ref({
  etcd: [
    {
      name: '开发环境',
      authway: '无',
      address: '192.168.1.1:2379;192.168.1.2:2379;',
      desc: '开发调试',
    },
    {
      name: '生成环境',
      authway: '无',
      address: '192.168.1.1:2379;192.168.1.2:2379;',
      desc: '生产环境，小心操作',
    },
  ]
}
);


onMounted(async () => {
    let data: string =  await query_cluster();
    tableData.value.etcd  = JSON.parse(data);

    //query existing instance
    let instance = await queryinstance({});
    if(instance != "Err") {
        let record = JSON.parse(instance);
        selectedValue.value = record[0].name;
    } else {
        if(tableData.value.etcd.length>0){
            selectedValue.value = tableData.value.etcd[0].name;
        }
    }
})

const handleSelectChange = async (data: any) => {
    let record = tableData.value.etcd.filter((row)=> row.name == data);

    //save current record to backend
    if(record){
        await saveinstance(record);
    }
}

</script>


<style lang="scss" scoped>
.title {
    font-size: 15px;
    color: #100f0f;
    margin-top: 12px;
}

.logo {
    margin-top: 11px;
}

.nav {
    background-color: transparent;
    font-size: 15px;
    margin-right: 200px;
    font-weight: bold;
}

.divider {
    margin-top: 18px;
}

// ::v-deep .model .el-input__inner {

// }
</style>
  