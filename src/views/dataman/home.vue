<template>
  <div class="common-layout">
    <el-container>
      <el-aside width="480px" class="bg-light-500">
        <Tree :value="nodes" :filter="true" filterMode="lenient" class="w-full md:w-30rem"></Tree>
      </el-aside>
      <el-main class="bg-light-500">
        <div>
        <Calendar v-model="date" />
        <Rating v-model="value" />
        <Button v-ripple v-tooltip="'Click to proceed'" class="ml-6">hello</Button>
    </div>
      </el-main>
    </el-container>
  </div>
</template>

<script setup  lang="ts">

import { ref } from "vue";
const value = ref(null);

const nodes = ref(
  [
        {
            "key": "0",
            "label": "Documents",
            "data": "Documents Folder",
            "icon": "pi pi-fw pi-inbox",
            "children": [{
                "key": "0-0",
                "label": "Work",
                "data": "Work Folder",
                "icon": "pi pi-fw pi-cog",
                "children": [{ "key": "0-0-0", "label": "Expenses.doc", "icon": "pi pi-fw pi-file", "data": "Expenses Document" }, { "key": "0-0-1", "label": "Resume.doc", "icon": "pi pi-fw pi-file", "data": "Resume Document" }]
            },
            {
                "key": "0-1",
                "label": "Home",
                "data": "Home Folder",
                "icon": "pi pi-fw pi-home",
                "children": [{ "key": "0-1-0", "label": "Invoices.txt", "icon": "pi pi-fw pi-file", "data": "Invoices for this month" }]
            }]
        },
        {
            "key": "1",
            "label": "Events",
            "data": "Events Folder",
            "icon": "pi pi-fw pi-calendar",
            "children": [
                { "key": "1-0", "label": "Meeting", "icon": "pi pi-fw pi-calendar-plus", "data": "Meeting" },
                { "key": "1-1", "label": "Product Launch", "icon": "pi pi-fw pi-calendar-plus", "data": "Product Launch" },
                { "key": "1-2", "label": "Report Review", "icon": "pi pi-fw pi-calendar-plus", "data": "Report Review" }]
        },
        {
            "key": "2",
            "label": "Movies",
            "data": "Movies Folder",
            "icon": "pi pi-fw pi-star",
            "children": [{
                "key": "2-0",
                "icon": "pi pi-fw pi-star",
                "label": "Al Pacino",
                "data": "Pacino Movies",
                "children": [{ "key": "2-0-0", "label": "Scarface", "icon": "pi pi-fw pi-video", "data": "Scarface Movie" }, { "key": "2-0-1", "label": "Serpico", "icon": "pi pi-fw pi-video", "data": "Serpico Movie" }]
            },
            {
                "key": "2-1",
                "label": "Robert De Niro",
                "icon": "pi pi-fw pi-star",
                "data": "De Niro Movies",
                "children": [{ "key": "2-1-0", "label": "Goodfellas", "icon": "pi pi-fw pi-video", "data": "Goodfellas Movie" }, { "key": "2-1-1", "label": "Untouchables", "icon": "pi pi-fw pi-video", "data": "Untouchables Movie" }]
            }]
        },
        {
            "key": "3",
            "label": "Movies",
            "data": "Movies Folder",
            "icon": "pi pi-fw pi-star",
            "children": [{
                "key": "3-0",
                "icon": "pi pi-fw pi-star",
                "label": "Al Pacino",
                "data": "Pacino Movies",
                "children": [{ "key": "2-0-0", "label": "Scarface", "icon": "pi pi-fw pi-video", "data": "Scarface Movie" }, { "key": "2-0-1", "label": "Serpico", "icon": "pi pi-fw pi-video", "data": "Serpico Movie" }]
            },
            {
                "key": "3-1",
                "label": "Robert De Niro",
                "icon": "pi pi-fw pi-star",
                "data": "De Niro Movies",
                "children": [{ "key": "3-1-0", "label": "Goodfellas", "icon": "pi pi-fw pi-video", "data": "Goodfellas Movie" }, { "key": "2-1-1", "label": "Untouchables", "icon": "pi pi-fw pi-video", "data": "Untouchables Movie" }]
            }]
        }
    ]
);


const count = ref(0)

const date = ref(null);
</script>
<style scoped  lang="scss">

::v-deep  input {
  border-color: blue;
  border-width: 1px;
}

</style>