<template>
  <div class="bg-dark-500">
  <el-button type="primary" @click="toggleDark()" class="ml-8">
    <i inline-block align-middle i="dark:carbon-moon carbon-sun" />
    <span >{{ isDark ? 'Dark' : 'Light' }}</span>
  </el-button>
  <el-button>Default</el-button>
  <el-button type="primary" class="bg-dark-400">Primary</el-button>
  <el-button type="success">Success</el-button>
  <el-button type="info">Info</el-button>
  <el-button type="warning">Warning</el-button>
  <el-button type="danger" class="float-right mr-8">Danger</el-button>
</div>
</template>


<script setup lang="ts">
// import { ref } from "vue";
// import { invoke } from "@tauri-apps/api/tauri";

import { useDark, useToggle } from '@vueuse/core'

const isDark = useDark({
  selector: 'body',
  attribute: 'color-scheme',
  valueDark: 'dark',
  valueLight: 'light',
})
const toggleDark = useToggle(isDark)






// async function greet() {
//   greetMsg.value = await invoke("greet", { name: name.value });
// }


</script>

<style scoped>
</style>


