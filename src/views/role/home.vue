<template>
  <div class="dockcontainer">
    <el-container>
      <el-aside width="480px" style="height: calc(100vh - 63px);">
        <div class="card flex justify-content-center">
          <Tree :value="roles" class="w-full md:w-30rem" @nodeSelect="onNodeSelect" selectionMode="single" :filter=true
            @contextmenu="ContextRightClick" filterMode="lenient" v-model:selectionKeys="selectedKey" v-model:expandedKeys="expandedKeys"></Tree>
        </div>
        <ContextMenu ref="menu" :model="mitems" />
      </el-aside>
      <el-dialog v-model="showCreateRoleDialog" title="新增角色" :style="{ width: '50vw' }">
        <el-form :model="dataForm" label-position="top" label-width="120px" require-asterisk-position="left">
          <el-form-item label="角色名称:">
            <el-input v-model="dataForm.name" />
          </el-form-item>
          <el-form-item label="" class="mt-20">
            <el-button class="ml-20 items-center" type="primary" @click="handleRoleCreateSubmit()"
              style="width: 100px;">提交</el-button>
            <el-button class="ml-20 items-center" type="primary" @click="handleRoleCreateCancel()"
              style="width: 100px;">取消</el-button>
          </el-form-item>
        </el-form>
      </el-dialog>
      <el-dialog v-model="showRoleGrantDialog" title="角色授权" :style="{ width: '50vw' }">
        <el-form :model="roleForm" label-position="top" label-width="120px" require-asterisk-position="left">
          <el-form-item label="角色名称:"  class="bg-gray-200">
            <el-input v-model="roleForm.role" disabled readonly=true />
          </el-form-item>
          <el-form-item label="权限">
            <el-select v-model="roleForm.ty" placeholder="请选择权限">
              <el-option label="读权限" value="read" />
              <el-option label="写权限" value="write" />
              <el-option label="读写权限" value="readwrite" />
            </el-select>
          </el-form-item>
          <el-form-item label="节点">
            <el-input v-model="roleForm.pathx" />
          </el-form-item>
          <el-form-item label="" class="mt-20">
            <el-button class="ml-20 items-center" type="primary" @click="handlePermissionSubmit()"
              style="width: 100px;">提交</el-button>
            <el-button class="ml-20 items-center" type="primary" @click="handlePermissionCancel()"
              style="width: 100px;">取消</el-button>
          </el-form-item>
        </el-form>
      </el-dialog>
      <el-main>
        <el-table :data="permisions" style="height: calc(100vh - 63px);width: calc(100vw - 480px);">
          <el-table-column fixed prop="key" label="节点路径" width="120" />
          <el-table-column prop="ty" label="权限">
            <template #default="scope">
               {{ transtype(scope.row.ty) }}
            </template>
          </el-table-column>
          <el-table-column prop="range_end" label="结束路径" width="120" />
          <el-table-column prop="with_prefix" label="启用前缀" width="120" />
          <el-table-column prop="with_from_key" label="启用FROMKEY" width="450" overflow="..." />
          <el-table-column>
            <template #default="scope">
              <el-button link type="primary" @click="handleRevokePermission(scope.$index, scope.row)">撤销</el-button>
            </template>
          </el-table-column>
        </el-table>
      </el-main>
    </el-container>
    <el-affix position="bottom" target=".dockcontainer">
      <el-row class="flex items-center mb-4 h-14 bg-gray-100">
        <el-col :span="6">
          <Button type="button" icon="pi pi-plus" text plain label="添加角色" @click="handleAddRole" />
        </el-col>
      </el-row>
    </el-affix>
  </div>
</template>

<script setup  lang="ts">
import { onMounted, ref } from 'vue';

import {
  etcd_get_all_roles, etcd_get_role_permissions, etcd_add_role,
  etcd_del_role,etcd_role_grant_perimssions,etcd_role_revoke_perimssions
} from '@/apis/api';

interface RoleNodeValue {
  key: string,
  keypath?: string,
  label: string,
  data?: string,
  icon: string,
}

const menu = ref();
const roles = ref();
interface formDataType {
  name: string
}
const dataForm = ref<formDataType>({
  name: "root"
})

interface roleDataType {
  role: string,
  ty: string,
  pathx: string,
}
const roleForm = ref<roleDataType>({
  role: "",
  ty: "",
  pathx: ""
});

const expandedKeys = ref<any>({});
const selectedKey = ref<any>(null);
const showRoleGrantDialog = ref(false);
const handlePermissionGrant = async () => {
  showRoleGrantDialog.value = true;
  if (selectNode.value ){
    roleForm.value.role = selectNode.value.data;
  }
}
const handleDeleteRole = async () => {
  if (selectNode.value == null) {
    return
  }
  if (selectNode.value?.key == "root") {
    return
  }

  await etcd_del_role(selectNode.value.key);
  await load();
}
const mitems = ref([
  { label: '授权路径', icon: 'pi pi-fw pi-send', command: handlePermissionGrant },
  { label: '删除角色', icon: 'pi pi-fw pi-send', command: handleDeleteRole }
]);


var root: RoleNodeValue[] = [];
const load = async () => {
  root = [];
  let role_list = await etcd_get_all_roles();
  role_list.forEach(
    (key: string) => {
      let child: RoleNodeValue = {
        key: key,
        keypath: key,
        label: key,
        data: key,
        icon: "pi pi-fw pi-cog",
      }
      root.push(child);
    }
  )

  roles.value = root;
}
onMounted(async () => {
  await load();
});

function transtype(ty: number): any {
    switch(ty) { 
      case 1: { 
          return "写"
      } 
      case 0: { 
          return "读"
      } 
      default: { 
          return "读写"
      } 
    }
}

const permisions = ref();

const selectNode = ref<RoleNodeValue>()
const onNodeSelect = async (node: RoleNodeValue) => {
  selectNode.value = node
  if (node) {
    let perm = await etcd_get_role_permissions(node.data);
    permisions.value = perm;
  }
}

const ContextRightClick = (event: any) => {
  menu.value.show(event);
};

const showCreateRoleDialog = ref(false);
const handleAddRole = async () => {
  showCreateRoleDialog.value = true;
  dataForm.value.name = "";
}

const handleRoleCreateSubmit = async () => {
  let res = await etcd_add_role(dataForm.value.name);
  console.log(res);
  dataForm.value.name = "";
  showCreateRoleDialog.value = false;
  await load();
}

const handleRoleCreateCancel = async () => {
  showCreateRoleDialog.value = false;
}

const handlePermissionSubmit =async () => {
  showRoleGrantDialog.value = false;
  await etcd_role_grant_perimssions(roleForm.value)
  let perm = await etcd_get_role_permissions(selectNode.value?.data);
  permisions.value = perm;
}
const handlePermissionCancel =async () => {
  showRoleGrantDialog.value = false;
}

const handleRevokePermission =async (index: any, row: any) => {
  let req = {
    role: selectNode.value?.data,
    ty: "read",
    pathx: row.key,
    rangeEnd: row.range_end
  };
  await etcd_role_revoke_perimssions(req);
  let perm = await etcd_get_role_permissions(selectNode.value?.data);
  permisions.value = perm;
}

</script>
<style scoped  lang="scss"></style>