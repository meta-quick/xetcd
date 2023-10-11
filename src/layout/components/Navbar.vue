<template>
    <div class="logo flex">
        <img src="../../assets/logo.png" style="width: 32px;height: 32px;" />
        <div class="title" style="line-height: 32px;">ECTD Manager</div>
        <span class="ml-16 mr-2" style="line-height: 32px;">运行环境:</span>
        <el-select v-model="value" placeholder="Select" class="xx">
            <el-option v-for="item in cities" :key="item.value" :label="item.label" :value="item.value">
                <span style="float: left">{{ item.label }}</span>
                <span style="float: right;color: var(--el-text-color-secondary);font-size: 13px; ">{{ item.value }}</span>
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
import { ref, computed} from 'vue';
import { useRoute, useRouter, RouteLocation } from 'vue-router';

interface CustomRouteLocation extends RouteLocation {
    meta: any;
}

const activeNavMenuIndex = ref()
const $route: CustomRouteLocation = useRoute();
const $router = useRouter();

// const defaultPath = computed(() => {
//     return $route.meta?.activeIndex || $route.fullPath || '';
// });

const naviMenu = computed(() => {
    const nav = $router.options.routes.filter((v) => v.meta?.isNav) || [];
    return nav;
});

//Hanle menu selection
const handleSelect = (path: string) => {
    console.log("@path",path)
    $router.push("/etcdman/index");
};


const value = ref('')
const cities = [
    {
        value: 'Beijing',
        label: 'Beijing',
    },
]

</script>


<style lang="scss" scoped>
.title {
    font-size: 15px;
    color: #100f0f;
}

.enviro {
    font-size: 15px;
    color: #100f0f;
    margin-left: 3px;
}

.logo {
    margin-left: -40px;
    margin-top: -10px;
}

.nav {
    background-color: transparent;
    margin-top: -10px;
    font-size: 15px;
    margin-right: 200px;
    font-weight: bold;
}

.divider {
    margin-top: 8px;
}

::v-deep .xx .el-input__inner {
    box-shadow: none;
}
</style>
  