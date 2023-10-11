<script lang="ts" setup>
import zhCn from 'element-plus/es/locale/lang/zh-cn';

const locale = zhCn;

</script>

<template>
  <el-config-provider :locale="locale" class="tauricontainer">
    <router-view/>
  </el-config-provider>
</template>

<style scoped>
.tauricontainer {
  width: 100%;
}
</style>
