<template>
    <el-container class="contain_wrapper">
      <el-container>
        hello
        <!-- <el-aside><sidebar/></el-aside>
        <el-main>
          <app-main />
        </el-main> -->
      </el-container>
    </el-container>
  </template>
  
  <script lang="ts" setup>
//   import Sidebar from './components/Sidebar/index.vue';
//   import AppMain from './components/AppMain.vue';
  </script>
  
  <style lang="scss" scoped>
  .contain_wrapper {
    width: 100%;
    height: 100%;
    background-color: #ecf6f6;
  
    .gateway-aside {
      width: 200px;
    }
    :deep(.micro-app) {
      // padding: 24px;
      .page {
        // height: calc(100vh - 98px);
      }
    }
  }
  </style>
  