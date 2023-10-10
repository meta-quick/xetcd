<script lang="ts">
import { defineComponent } from 'vue';
import zhCn from 'element-plus/es/locale/lang/zh-cn';

export default defineComponent({
  setup() {
    // 切换为中文
    const locale = zhCn;

    return {
      locale
    };
  }
});

</script>

<template>
  <el-config-provider :locale="locale" namespace="etcd" class="tauricontainer">
    <router-view/>
  </el-config-provider>
</template>

<style scoped>
.tauricontainer {
  width: 100%;
}
</style>
