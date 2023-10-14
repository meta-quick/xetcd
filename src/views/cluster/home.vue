<template>
  <div>
    <el-button class="mb-1 ml-6" @click="handleRefreshEtcdCluster">刷新列表</el-button>
    <el-button class="mb-1 ml-6" @click="handleCreateEtcdCluster">添加集群</el-button>
    <el-dialog v-model="showClusterDialog" :title="titleText" width="67%" destroy-on-close center>
      <el-form :model="formData.etcd" label-width="120px">
        <el-form-item label="集群名称">
          <el-input v-model="formData.etcd.name" />
        </el-form-item>
        <el-form-item label="认证方式">
          <el-select v-model="formData.etcd.authway" placeholder="请选择认证方式">
            <el-option label="无" value="无" />
            <el-option label="证书" value="证书" />
            <el-option label="密码" value="密码" />
          </el-select>
        </el-form-item>
        <el-form-item label="集群地址">
          <el-input v-model="formData.etcd.address" placeholder="请用英文分号分割地址,如: 192.168.1.1:2379;192.168.1.2:2379;" />
        </el-form-item>
        <el-form-item label="说明">
          <el-input v-model="formData.etcd.desc" type="textarea" />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="handleSubmitEtcdCluster">{{ submitText }}</el-button>
          <el-button @click="handleCancleEtcdCluster">取消</el-button>
        </el-form-item>
      </el-form>
    </el-dialog>
    <el-table :data="tableData.etcd" style="width: 100%">
      <el-table-column fixed prop="name" label="集群名称" width="120" />
      <el-table-column prop="address" label="集群地址" width="680" />
      <el-table-column prop="authway" label="认证方式" width="120"/>
      <el-table-column prop="desc" label="用途说明" width="450" overflow="..."/>

      <el-table-column>
        <template #default="scope">
          <el-button link type="primary" size="small" @click="handleEditRow(scope.$index,scope.row)">修改</el-button>
          <el-button link type="primary" size="small" @click="handleDeleteRow(scope.$index)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>

<script lang="ts" setup>
import { onMounted, ref } from 'vue';
import {save_cluster,query_cluster} from '@/apis/api'

const showClusterDialog = ref(false);
const titleText = ref("添加ETCD集群环境");
const submitText = ref("添加集群");


const tableData = ref({
  rowindex: 0,
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


onMounted(async ()=>{
  let data: string =  await query_cluster();
  tableData.value.etcd = JSON.parse(data);
});


const formData = ref({
  etcd: {
    name: '',
    authway: '',
    address: '',
    desc: ''
  }
});


const handleCreateEtcdCluster = async () => {
  showClusterDialog.value = true;
  EditRowState = false;
  formData.value.etcd = {
    name: '',
    authway: '',
    address: '',
    desc: ''
  };

  titleText.value = "添加ETCD集群环境"
  submitText.value = "添加集群";
}

const handleRefreshEtcdCluster =async () => {
  let data: string = await query_cluster();
  tableData.value.etcd = JSON.parse(data);
}

const handleSubmitEtcdCluster = async () => {
  const rs = {
        name: formData.value.etcd.name,
        authway: formData.value.etcd.authway,
        address: formData.value.etcd.address,
        desc: formData.value.etcd.desc,
      };
  tableData.value.etcd.push(rs);

  if (EditRowState) {
    tableData.value.etcd.splice(RowIndex, 1)
  }

  await save_cluster(tableData.value.etcd);
  showClusterDialog.value = false;
}

const handleDeleteRow = async (index:number) => {
  tableData.value.etcd.splice(index, 1)
  await save_cluster(tableData.value.etcd);
}
let EditRowState = false;
let RowIndex = 0;
const handleEditRow =async (index:number,row: any) => {
  formData.value.etcd = row;
  EditRowState = true;
  RowIndex = index;

  //Update title and SubmitText
  titleText.value = "修改ETCD集群环境"
  submitText.value = "保存集群";

  showClusterDialog.value = true;
}

const handleCancleEtcdCluster = async () => {
  showClusterDialog.value = false;
  EditRowState = false;
}

</script>

<style lang="scss" scoped>
</style>
