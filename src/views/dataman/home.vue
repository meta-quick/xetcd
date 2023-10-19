<template>
  <div class="dockcontainer">
    <el-container>
      <el-aside width="480px" style="height: calc(100vh - 63px);">
        <Tree :value="nodes" v-model:selectionKeys="selectedKey" v-model:expandedKeys="expandedKeys"
          @nodeSelect="onNodeSelect" @contextmenu="onTreeContextRightClick" selectionMode="single" :filter="true"
          filterMode="lenient" class="w-full">
        </Tree>
        <ContextMenu ref="menu" :model="mitems" />
      </el-aside>
      <el-main>
        <div class="flex toolBar">
          <span class="ml-10">{{ selectedKeyPath }}</span>
          <div>
            <Dropdown v-model="selectedLang" :options="languageList" @change="handleSwitchLanguage" optionLabel="name"
              placeholder="Select a language" />
            <Button class="ml-1 mr-2" @click="handleContentForrmat">格式化</Button>
            <Button class="ml-1 mr-2" @click="handleContentSave">保存</Button>
          </div>
        </div>
        <div id="monacoeditor" class="monacocontainer mt-2 pt-2"
          style="height: calc(100% - 63px);width: calc(100vw - 480px);" ref="main"></div>
      </el-main>
      <el-dialog v-model="showCreateDialog"  title="新增节点" size="large" class="mr-10 ml-10 p-3" :style="{ width: '50vw' }">
        <el-form :model="dataForm" :label-position="labelPosition" label-width="120px" 
           ref="ruleFormRef"
           :rules="rules"
           require-asterisk-position="left">
          <el-form-item label="根路径" class="bg-gray-200">
            <el-input v-model="dataForm.root" readonly="true" disabled/>
          </el-form-item>
          <el-form-item label="名称">
            <el-input v-model="dataForm.name" autocomplete="off"/>
          </el-form-item>
          <el-form-item label="是否目录">
            <el-select v-model="dataForm.isdir" placeholder="Select" size="large">
                <el-option
                  v-for="item in inputSelectOptions"
                  :key="item.value"
                  :label="item.label"
                  :value="item.value"
                />
            </el-select>
          </el-form-item>
          <el-form-item label="TTL" autocomplete="off">
            <el-input v-model="dataForm.ttl"/>
          </el-form-item>
          <el-form-item label="内容">
            <el-input type="textarea" autosize autocomplete="off"  v-model="dataForm.value" :disabled="dataForm.isdir" :rows="3"/>
          </el-form-item>
          <el-form-item label="">
            <el-button class="ml-20 items-center" type="primary" @click="handleNodeCreateSubmit(ruleFormRef)" style="width: 100px;">提交</el-button>
            <el-button class="ml-20 items-center" type="primary"  @click="handleNodeCreateCancel(ruleFormRef)" style="width: 100px;">取消</el-button>
          </el-form-item>
        </el-form>
      </el-dialog>
      <el-dialog v-model="showDeleteDialog"  title="删除节点" size="large" class="mr-10 ml-10 p-3" :style="{ width: '50vw' }">
        <el-form :model="dataFormDelete" :label-position="labelPosition" label-width="120px">
          <el-form-item label="节点路径" class="bg-gray-200">
            <el-input v-model="dataFormDelete.root" readonly="true" disabled/>
          </el-form-item>
          <el-form-item label="是否目录">
            <el-select v-model="dataFormDelete.isdir" placeholder="Select" disabled size="large">
                <el-option
                  v-for="item in inputSelectOptions"
                  :key="item.value"
                  :label="item.label"
                  :value="item.value"
                />
            </el-select>
          </el-form-item>
          <el-form-item label="内容" v-show="!dataFormDelete.isdir">
            <el-input type="textarea" v-model="dataFormDelete.value" disabled :rows="3"/>
          </el-form-item>
          <el-form-item label="">
            <span>如确认需求删除，请输入:'{{randomConfirmString}}'</span>
            <el-input v-model="randomConfirmInputString"/>
          </el-form-item>
          <el-form-item label="">
            <el-button class="ml-20 items-center" type="primary" style="width: 100px;" @click="handleDeleteNodeConfirm">确定</el-button>
            <el-button class="ml-20 items-center" style="width: 100px;" @click="handleNodeDeleteCancel">取消</el-button>
          </el-form-item>
        </el-form>
      </el-dialog>
    </el-container>
    <el-affix position="bottom" target=".dockcontainer">
      <el-row class="flex items-center mb-4 h-14 bg-gray-100">
        <el-col :span="6">
          <Button type="button" icon="pi pi-plus" text plain label="展开节点" @click="expandAll" />
          <Button type="button" icon="pi pi-minus" text plain label="收起" @click="collapseAll" />
        </el-col>
        <el-col :span="6"><span>Version: {{ versionRef }}</span></el-col>
        <el-col :span="4"><span>Modification Version: {{ modversionRef }}</span></el-col>
        <el-col :span="4"><span>Creatation Version: {{ createversionRef }}</span></el-col>
        <el-col :span="4"><span>lease: {{ leaseRef }}</span></el-col>
      </el-row>
    </el-affix>
  </div>
</template>

<script setup  lang="ts">
import { onMounted, ref, reactive } from "vue";
import type { FormInstance, FormRules } from 'element-plus'
import * as monaco from 'monaco-editor';
import { get_all_keys,etc_delete_key,etc_put_key,etcd_simpleput_key } from '@/apis/api';
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker';
import cssWorker from 'monaco-editor/esm/vs/language/css/css.worker?worker';
import htmlWorker from 'monaco-editor/esm/vs/language/html/html.worker?worker';
import tsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker';
import YamlWorker from 'monaco-yaml/yaml.worker.js?worker';

const main = ref<HTMLElement>();

const expandedKeys = ref<any>({});
const selectedKeyPath = ref<string>("");
const selectedValue = ref<string>("");
const selectedKey = ref<any>(null);
const versionRef = ref<any>();
const modversionRef = ref<any>();
const createversionRef = ref<any>();
const leaseRef = ref<any>();
const showCreateDialog = ref<boolean>(false);
const showDeleteDialog = ref<boolean>(false);
const labelPosition = ref("top");
const randomConfirmString = ref("");
const randomConfirmInputString = ref("");
const menu = ref();

interface ValueModel {
  root: string,
  name: string,
  ttl: number,
  isdir: boolean,
  value: string,
};

const ruleFormRef = ref<FormInstance>()
const dataForm = ref<ValueModel>({
  root: "",
  name: "",
  ttl: 0,
  isdir: false,
  value: "",
});

const rules = reactive<FormRules<ValueModel>>({
  name: [
    { required: true, message: 'Please input Activity name', trigger: 'blur' },
    { min: 3, max: 5, message: 'Length should be 3 to 5', trigger: 'blur' },
  ],
});

const onTreeContextRightClick = (event: any) => {
  menu.value.show(event);
};

const handleCreateNode = async () => {
  if(dataForm.value && selectedKeyPath.value){
    dataForm.value =  {
      root: selectedKeyPath.value,
      name: "",
      ttl: 0,
      isdir: false,
      value: "",
    };
  }
  showCreateDialog.value = true;
}

const handleDeleteNode = async () => {
  let randstr = (Math.random() + 1).toString(36).substring(7);
  if(dataFormDelete.value && selectedKeyPath.value){
    dataFormDelete.value.root = selectedKeyPath.value;
    dataFormDelete.value.isdir = false;
    randomConfirmString.value = randstr;
    randomConfirmInputString.value = "";
    if(selectedKeyPath.value.endsWith("/")){
      dataFormDelete.value.isdir = true;
      selectedValue.value = "";
      dataFormDelete.value.value = selectedValue.value;
    }else{
      if(selectedValue.value){
        dataFormDelete.value.value = selectedValue.value;
      }
    }
  }
  showDeleteDialog.value = true;
}

const handleDeleteNodeConfirm =async () => {
  if(randomConfirmString.value != randomConfirmInputString.value){
    showDeleteDialog.value = true;
    return;
  }
  await etc_delete_key(selectedKeyPath.value);
  showDeleteDialog.value = false;
  await load_data();
}

const handleNodeDeleteCancel = async () => {
  showDeleteDialog.value = false;
}

const handleNodeCreateSubmit = async (formEl: FormInstance | undefined) => {
  if (!formEl) return
  await etc_put_key(dataForm.value);
  showCreateDialog.value = false;
  await load_data();
}

const handleNodeCreateCancel = async (formEl: FormInstance | undefined) => {
  if (!formEl) return
  formEl.resetFields()
  showCreateDialog.value = false;
}


const mitems = ref([
  { label: 'Create', icon: 'pi pi-fw pi-send', command: handleCreateNode },
  { label: 'Delete', icon: 'pi pi-fw pi-trash', command: handleDeleteNode }
]);

const nodes = ref<NodeValue[] | null>(
  []
);

const expandAll = () => {
  if(nodes.value){
    for (let node of nodes.value) {
      expandNode(node);
    }
  }
  expandedKeys.value = { ...expandedKeys.value };
};

const collapseAll = () => {
  expandedKeys.value = {};
};

const expandNode = (node: NodeValue) => {
  if (node.children && node.children.length) {
    expandedKeys.value[node.key] = true;

    for (let child of node.children) {
      expandNode(child);
    }
  }
};

const languageList = [
  {
    name: 'JSON',
    code: 'json',
  },
  {
    name: 'YAML',
    code: 'yaml',
  },
  {
    name: 'TYPESCRIPT',
    code: 'typescript',
  },
  {
    name: 'TEXT',
    code: 'txt',
  },
  {
    name: 'INI',
    code: 'ini',
  },
  {
    name: 'JAVASCRIPT',
    code: 'javascript',
  },
]

const selectedLang = ref({
  name: 'YAML',
  code: 'yaml',
});

let editor: monaco.editor.IStandaloneCodeEditor | null = null
onMounted(async () => {
  if (main && main.value) {
    editor = monaco.editor.create(main.value, {
      value: "",
      language: 'yaml',
      fontSize: 16,
      suggestFontSize: 18,
      tabCompletion: 'on',
      automaticLayout: true,
      autoDetectHighContrast: true,
    });
  }

  await load_data();
})

const load_data = async () => {
  root = [];
  let results: JSonKeyValue[] = await get_all_keys();

  results.forEach((element) => {
    let paths = element.key.split("/")
    let parent = null;
    let parent_keypath = "";
    for (let idx = 0; idx < paths.length; idx++) {
      if (idx == paths.length - 1) {
        if (paths[idx] == "") {
          continue;
        }
        let key = parent_keypath + paths[idx];
        let child: NodeValue = {
          key: key,
          keypath: key,
          label: paths[idx],
          data: element.value,
          icon: "pi pi-fw pi-cog",
          lease: element.lease,
          mod_revision: element.mod_revision,
          create_revision: element.create_revision,
          version: element.version,
          children: null,
          leaf: true,
        }
        parent_keypath = key,
          parent = addNode(parent, child);
      } else {
        if (paths[idx] == "") {
          parent_keypath = "/";
          continue;
        }
        let key = parent_keypath + paths[idx] + "/";
        let child: NodeValue = {
          key: key,
          keypath: key,
          label: paths[idx],
          icon: "pi pi-fw pi-folder",
          children: [],
          leaf: false,
        }
        parent_keypath = key,
          parent = addNode(parent, child);
      }
    }
  });
  let parent: NodeValue = {
          key: "/",
          keypath: "/",
          label: "ROOT",
          icon: "pi pi-fw pi-list",
          children: root,
          leaf: false,
        }
  nodes.value = [parent];
}

self.MonacoEnvironment = {
  getWorker(_: any, label: string) {
    if (label === 'json') {
      return new jsonWorker();
    }
    if (label === 'css' || label === 'scss' || label === 'less') {
      return new cssWorker();
    }
    if (label === 'html' || label === 'handlebars' || label === 'razor') {
      return new htmlWorker();
    }
    if (label === 'typescript' || label === 'javascript') {
      return new tsWorker();
    }

    if (label === 'yaml' || label === 'yml') {
      return new YamlWorker();
    }

    return new editorWorker();
  }
};

const handleSwitchLanguage = async () => {
  if (editor) {
    let model = editor.getModel();
    if (model) {
      monaco.editor.setModelLanguage(model, selectedLang.value.code);
    }
  }
}


interface JSonKeyValue {
  key: string,
  create_revision: number,
  mod_revision: number,
  version: number,
  value: string,
  lease: number,
}

interface NodeValue {
  key: string,
  keypath?: string,
  label: string,
  create_revision?: number,
  mod_revision?: number,
  version?: number,
  data?: string,
  icon: string,
  lease?: number,
  leaf?: boolean,
  children?: NodeValue[] | null,
}

let root: NodeValue[] = [];
function addNode(parent: NodeValue | null, child: NodeValue): NodeValue {
  if (parent == null || parent == undefined) {
    for (let i = 0; i < root.length; i++) {
      if (root[i].key == child.key) {
        return root[i];
      }
    }

    root.push(child);

  } else {
    if (parent.children) {
      for (let i = 0; i < parent.children.length; i++) {
        if (parent.children[i].key == child.key) {
          return parent.children[i];
        }
      }
      parent.children?.push(child);
    }
  }
  return child;
}

const onNodeSelect = (node: NodeValue) => {
  if (editor && node) {
    if (node.data) {
      editor.setValue(node.data);
      if (node.keypath) {
        selectedKeyPath.value = node.keypath;
        selectedValue.value = node.data;
        versionRef.value = node.version;
        modversionRef.value = node.mod_revision;
        createversionRef.value = node.create_revision;
        leaseRef.value = node.lease;
      }
    } else {
      if (node.keypath) {
        selectedKeyPath.value = node.keypath;
        selectedKeyPath.value = node.keypath;
        versionRef.value = node.version;
        if(node.data){
          selectedValue.value = node.data;
        }
        modversionRef.value = node.mod_revision;
        createversionRef.value = node.create_revision;
        leaseRef.value = node.lease;
        if (!node.data) {
          editor.setValue('undefined');
        }
      }
      editor.setValue("");
    }
  }
};

const handleContentSave = async () => {
  if (editor) {
    let content = editor.getValue();

    let kv = {
      key: selectedKeyPath.value,
      value: content,
    };

    await etcd_simpleput_key(kv);
    await load_data();
  }
};

const handleContentForrmat = async () => {
  if (editor) {
    editor.trigger("editor", "editor.action.formatDocument", "");
  }
};

const dataFormDelete = ref<ValueModel>({
  root: "",
  name: "",
  ttl: 0,
  isdir: false,
  value: "",
});

const inputSelectOptions = ref([
  {
    value: true,
    label: '是',
  },
  {
    value: false,
    label: '否',
  }
]);

</script>
<style scoped  lang="scss">
:deep(input) {
  border-color: blue;
  border-width: 1px;
}

.monacocontainer {
  height: 100%;
}

.dockcontainer {
  height: calc(100% - 63px);
}

.toolBar {
  justify-content: space-between;
  align-items: center;
}
</style>