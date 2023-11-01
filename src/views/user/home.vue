<template>
  <div class="dockcontainer">
    <el-container>
      <el-aside width="480px" style="height: calc(100vh - 63px);">
        <div class="card flex justify-content-center">
          <Tree :value="users" class="w-full md:w-30rem" @nodeSelect="onNodeSelect" selectionMode="single" :filter=true
            @contextmenu="ContextRightClick" filterMode="lenient" v-model:selectionKeys="selectedKey" v-model:expandedKeys="expandedKeys"></Tree>
        </div>
        <ContextMenu ref="menu" :model="mitems" />
      </el-aside>
      <el-dialog v-model="showCreateUserDialog" title="新增用户" :style="{ width: '50vw' }">
        <el-form :model="dataForm" label-position="top" label-width="120px" require-asterisk-position="left">
          <el-form-item label="用户名:">
            <el-input v-model="dataForm.name" />
          </el-form-item>
          <el-form-item label="密码">
            <el-input v-model="dataForm.password" />
          </el-form-item>
          <el-form-item label="密码确认">
            <el-input v-model="dataForm.passwordconfirm" />
          </el-form-item>
          <el-form-item label="" class="mt-20">
            <el-button class="ml-20 items-center" type="primary" @click="handleUserCreateSubmit()"
              style="width: 100px;">提交</el-button>
            <el-button class="ml-20 items-center" type="primary" @click="handleUserCreateCancel()"
              style="width: 100px;">取消</el-button>
          </el-form-item>
        </el-form>
      </el-dialog>
      <el-dialog v-model="showRoleGrantDialog" title="角色授权" :style="{ width: '50vw' }">
        <el-form :model="roleForm" label-position="top" label-width="120px" require-asterisk-position="left">
          <el-form-item label="角色名称:" >
            <el-select v-model="roleForm.role" placeholder="请选择权限">
              <el-option
                v-for="item in roleListOptions"
                :key="item.value"
                :label="item.label"
                :value="item.value"
              />
            </el-select>  
          </el-form-item>
          <el-form-item label="" class="mt-20">
            <el-button class="ml-20 items-center" type="primary" @click="handleUserRoleSubmit()"
              style="width: 100px;">提交</el-button>
            <el-button class="ml-20 items-center" type="primary" @click="handleserRoleCancel()"
              style="width: 100px;">取消</el-button>
          </el-form-item>
        </el-form>
      </el-dialog>
      <el-main>
        <el-table :data="roles" style="height: calc(100vh - 63px);width: calc(100vw - 480px);">
          <el-table-column fixed prop="name" label="角色"/>
          <el-table-column width="120">
            <template #default="scope">
              <el-button link type="primary" @click="handleRevokeRole(scope.$index, scope.row)">撤销</el-button>
            </template>
          </el-table-column>
        </el-table>
      </el-main>
    </el-container>
    <el-affix position="bottom" target=".dockcontainer">
      <el-row class="flex items-center mb-4 h-14 bg-gray-100">
        <el-col :span="6">
          <Button type="button" icon="pi pi-plus" text plain label="添加用户" @click="handleAddUser" />
        </el-col>
      </el-row>
    </el-affix>
  </div>
</template>

<script setup  lang="ts">
import { onMounted, ref } from 'vue';

import {
  etcd_user_list,etcd_user_role_list,etcd_user_add,etcd_user_delete,etcd_get_all_roles,
  user_grant_role,user_revoke_role
} from '@/apis/api';

interface RoleNodeValue {
  key: string,
  keypath?: string,
  label: string,
  data?: string,
  icon: string,
}

const menu = ref();
const users = ref();
interface formDataType {
  name: string,
  password: string,
  passwordconfirm: string
}
const dataForm = ref<formDataType>({
  name: "",
  password: "",
  passwordconfirm: ""
})

const expandedKeys = ref<any>({});
const selectedKey = ref<any>(null);
const showRoleGrantDialog = ref(false);
const roleForm =ref<any>();
const roleListOptions =ref<any>();

const handleRoleGrant = async () => {
  let options = await etcd_get_all_roles();
  root = [];
  options.forEach(
    (key: string) => {
      let child: any = {
        key: key,
        label: key,
        value: key,
      }
      root.push(child);
    }
  )

  roleListOptions.value = root;

  if(selectNode.value){
    roleForm.value = {
      role: "",
      user: selectNode.value.key
    }
  }
  showRoleGrantDialog.value = true;
}

const handleDeleteUser = async () => {
  if (selectNode.value == null) {
    return
  }
  if (selectNode.value?.key == "root") {
    return
  }

  await etcd_user_delete(selectNode.value.key);
  await load();
}

const mitems = ref([
  { label: '角色授权', icon: 'pi pi-fw pi-send', command: handleRoleGrant },
  { label: '删除用户', icon: 'pi pi-fw pi-send', command: handleDeleteUser}
]);


var root: RoleNodeValue[] = [];
const load = async () => {
  root = [];
  let user_list = await etcd_user_list();
  user_list.forEach(
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

  users.value = root;
}
onMounted(async () => {
  await load();
});

const roles = ref();

const selectNode = ref<RoleNodeValue>()
const onNodeSelect = async (node: RoleNodeValue) => {
  selectNode.value = node
  root = [];
  if (node) {
    let role_list = await etcd_user_role_list(node.data);
    role_list.forEach(
      (key: string) => {
        let child: RoleNodeValue = {
          name: key
        }
        root.push(child);
      }
    )

    roles.value = root;
  }
}

const ContextRightClick = (event: any) => {
  menu.value.show(event);
};

const showCreateUserDialog = ref(false);
const handleAddUser = async () => {
  showCreateUserDialog.value = true;
  dataForm.value = {
    name: "",
    password: "",
    passwordconfirm: ""
  }
}

const handleUserCreateSubmit =async () => {
  showCreateUserDialog.value = false;
  if(dataForm.value.password != dataForm.value.passwordconfirm){
    return;
  }

  let req = {
    name: dataForm.value.name,
    password: dataForm.value.password,
  };
  await etcd_user_add(req);
  await load();
  dataForm.value = {
    name: "",
    password: "",
    passwordconfirm: ""
  };
}

const handleUserCreateCancel = async () => {
  showCreateUserDialog.value = false;
  dataForm.value = {
    name: "",
    password: "",
    passwordconfirm: ""
  };
}

const handleUserRoleSubmit = async () => {
  await user_grant_role({role: roleForm.value.role,user: roleForm.value.user});
  roleForm.value = {}
  showRoleGrantDialog.value = false;
  let role_list = await etcd_user_role_list(selectNode.value?.key);
  root = [];
  role_list.forEach(
    (key: string) => {
      let child: RoleNodeValue = {
        name: key
      }
      root.push(child);
    }
  )

  roles.value = root;
}

const handleserRoleCancel = async () => {
  showRoleGrantDialog.value = false;
  roleForm.value = {}
}

const handleRevokeRole =async (index: number, row: any) => {
  if(selectNode.value){
    if(selectNode.value.key=="rooe" && row.name=="root"){
      return;
    }
    await user_revoke_role({role: row.name,user: selectNode.value.key})
    let role_list = await etcd_user_role_list(selectNode.value?.key);
    root = [];
    role_list.forEach(
      (key: string) => {
        let child: RoleNodeValue = {
          name: key
        }
        root.push(child);
      }
    )

    roles.value = root;
  }
}

</script>
<style scoped  lang="scss"></style>