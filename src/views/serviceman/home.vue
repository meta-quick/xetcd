<template>
  <div class="dockcontainer">
    <el-container>
      <el-aside width="280px" style="height: calc(100vh - 63px);">
        <div class="card flex justify-content-center">
          <Tree :value="sidemenus" class="w-full md:w-30rem" @nodeSelect="onNodeSelect" selectionMode="single"
            filterMode="lenient" v-model:selectionKeys="selectedKey" v-model:expandedKeys="expandedKeys">
          </Tree>
        </div>
      </el-aside>
      <el-main>
        <div class="flex ml-20 toolBar">
            <Button class="ml-1 mr-2" v-if="isCertificateRequest" @click="handleCertReset" >重置CA</Button>
            <Button class="ml-1 mr-2" v-if="isCertificate && isCertificateRequest" @click="handleCertRequst">提交申请</Button>
            <Button class="ml-1 mr-2" v-if="isCertificateRequest" @click="handleCertDownload">查询证书</Button>
            <Button class="ml-1 mr-2" v-if="isSignedRequest" @click="handleSignedCertRequst">提交证书申请</Button>
            <Button class="ml-1 mr-2" v-if="isIamConfigRequest" @click="handleIamConfigRequst">提交IAM配置</Button>
            <Button class="ml-1 mr-2" v-if="isAppConfigRequest" @click="handleAppConfigRequst">提交应用配置</Button>
        </div>
        <div id="monacoeditor" class="monacocontainer mt-2 pt-2"
          style="height: calc(100% - 63px);width: calc(100vw - 480px);" ref="main">
        </div>
      </el-main>
    </el-container>
  </div>
</template>

<script setup  lang="ts">
import { ref,onMounted } from 'vue';
import * as monaco from 'monaco-editor';
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker';
import cssWorker from 'monaco-editor/esm/vs/language/css/css.worker?worker';
import htmlWorker from 'monaco-editor/esm/vs/language/html/html.worker?worker';
import tsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker';
import YamlWorker from 'monaco-yaml/yaml.worker.js?worker';
import yaml from 'js-yaml';
import {pki_make_ca,pki_query_ca,mk_signed_cert,etcd_put_mapkeys} from '@/apis/api';


const main = ref<HTMLElement>();
const sidemenus = ref([
  {
    key: "CA证书",
    keypath:"PKI证书",
    label: "PKI证书",
    data: "PKI证书",
    icon: "pi pi-fw pi-cog",
  },
  {
    key: "密钥申请",
    keypath:"密钥申请",
    label: "密钥申请",
    data: "密钥申请",
    icon: "pi pi-fw pi-cog",
  },
  {
    key: "IAM配置",
    keypath:"IAM配置",
    label: "IAM配置",
    data: "IAM配置",
    icon: "pi pi-fw pi-cog",
  },
  {
    key: "网关证书",
    keypath:"网关证书",
    label: "网关证书",
    data: "网关证书",
    icon: "pi pi-fw pi-cog",
  },
  {
    key: "网关业务服务",
    keypath:"/configuration/policyman-devel-master.yaml",
    label: "网关业务服务",
    data: "网关业务服务",
    icon: "pi pi-fw pi-cog",
  },
  {
    key: "权限服务配置",
    keypath:"/configuration/permission-devel-master.yaml",
    label: "权限服务配置",
    data: "权限服务配置",
    icon: "pi pi-fw pi-cog",
  }
])
const expandedKeys = ref<any>({})
const selectedKey = ref<any>(null)
const selectNode = ref<any>()
const isCertificate = ref(false)
const isCertificateRequest = ref(false)
const isSignedRequest = ref(false)
const isIamConfigRequest = ref(false)
const isAppConfigRequest = ref(false)

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
})

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

const selectNodePathKey = ref<string>("");

const onNodeSelect = async (node: any) => {
  selectNode.value = node;
  if(editor){
    let template :string = createTemplate(node.data);
    selectNodePathKey.value = node.keypath;
    editor.setValue(template);
  }
}

const pkiYaml = `
CA:
  algorithm: rsa
  #stored: true
  size: 2048
  entries:
    CN: CN
    ST: SC
    O:  META
    OU: META QUICK CA
  validity:
    notBefore: 2024-01-01T00:00:00Z
    notAfter: 2025-01-01T00:00:00Z
`

const certYaml = `
CERT:
  algorithm: rsa
  size: 2048
  dns:
  - localhost
  - 127.0.0.1
  - datasafe.com
  entries:
    CN: CN
    ST: SC
    O:  META
    OU: META QUICK CA
  validity:
    notBefore: 2024-01-01T00:00:00Z
    notAfter: 2025-01-01T00:00:00Z
`

///configuration/cluster/publicKey
const iamAdminConfigYaml = `
configuration:
  cluster:
    iamadmin:
      cert: mess
      client-id: c8eee4ccbab0e67ea404
      client-secret: a663244a60606b6b02becd5d12e7122d9b1c8d1b
      enabled: true  
    publicKey: |
      -----BEGIN CERTIFICATE-----
      MIIE2TCCAsGgAwIBAgIDAeJAMA0GCSqGSIb3DQEBCwUAMCYxDjAMBgNVBAoTBWFk
      bWluMRQwEgYDVQQDDAtjZXJ0X2hhYjE4bjAeFw0yMzAxMDQwNTA5MzlaFw00MzAx
      MDQwNTA5MzlaMCYxDjAMBgNVBAoTBWFkbWluMRQwEgYDVQQDDAtjZXJ0X2hhYjE4
      bjCCAiIwDQYJKoZIhvcNAQEBBQADggIPADCCAgoCggIBAKWEYPFCe/tOnsU6prlY
      H525AXcTjlFjaWcb6gjuypaMUu/mXCXYbnMU/ZAi8YV/VFdNq4xrBDsNklaNejhx
      y2XYovpw/86UtExY5qjpFB9LfD/bnA5+D537Z68PKytFjYvdZCcttIWrP0SghGmD
      x0s6To+pFsi2YgkmOzIVis6M5ozVRZ9csLTHVI7AuQFPeYnzNiY7V8BV9LlG6MB5
      BtGZLlkX8nfjEYy1vCReHr47yWldxjS5CgNK2EE+bqRnr3aFduuxMg8gb7EghZwB
      4wHuzXXUTRbei2JOny0qFo7L5ZDYk4cZFVuloqh17rTTBLANqECmzBJu2laNZ2vQ
      lzTKPmEdwS8upTpsTupdec0fblJDgepf4oomebyUlCeHEj6ieujt7YEZhoWT0LF/
      RSqqNIPw59dmU360h8qsDv2yEfa0Iprg2ZfPtTe5yUjuNzFMU42lMFYtVErTaqTJ
      LVDGzdEWq7NjYsvSTrcu5ywPhyZFJv4lckC4MIZVbEqOJiP6u3+URQempN0JHSsM
      lbNlx6jioPGj2PU/4tfT+AxV3esynGzdC8VgJTVAk822Ok0qc83Alb362GF9q/nq
      bkuAqHMMTIq4TA35KFUuxquj80fzl7gZ7Xw0HmXsMAzwdBDYpXOUmVGdKpcm9/+7
      K0aFkgPWoJcedp1actPV6GMtAgMBAAGjEDAOMAwGA1UdEwEB/wQCMAAwDQYJKoZI
      hvcNAQELBQADggIBABoQLwCKtL2ZhxCEcV81Pt1Ntr0SV8Jf6ha7Dck9Ofkf9Y/o
      1Nq6XUHUVQw9UsZj6HIKIv5PXV3b/m2rECTN/dmYug3/LJOiiHgma/tO1OvVTIMK
      8rb3XMHM5AtekeuegsN9n4SVrOUrHZKsn3fMwnowxPT0iq4TrrTdUkVDwykf2MBm
      um6yYV3a/XZGmlyWk2xb6yT6M6U7OzSrcup/IVIFn1DIw6rkKltMYr59FvWKCoIK
      kMVm2ZngFcSCRux0x6pWw+zWZz48yev/ch3PoVw9xBAtnxI9ay54uebHvqAP5Jr2
      llUzq68aMVCJPc+QpU5SNy6OTVfd8tHmluvQQ5+F0hBOLteIQAvBu6FnYzYFIZaT
      7BqIF1t74mqKCy/x7JwdbbagwGu52IFe57sJW82nRK8CliQm4yZ3XPXe4gsULq6G
      ftLn5nh0KpRRMFiyL09CAseR5RMGSwvh4Nbr+OhDE8zLqPFJUux8yz9G1WrLyJ0a
      ZJzhtVJv7kzOLSeg+rv4xDADpU7OwzZmNMvEZb+FmPHeUefMWONV6O6hxYg8xq0Q
      btO2lfKdUqXVG/WUzwSguxnV47aGRWWj/ppmqgnmSW5OuJqgIcPuAmVve9ko3kRE
      6KFLrkH9z0jfoynmQHczTU322Br/dOodHfe0fGPGiNuPsPtOoq1rypNh1xrD
      -----END CERTIFICATE-----
`

///udscctlplane/kms/publicKey
const iamGatewayKey = `
udscctlplane:
  kms:
    publicKey: |
      -----BEGIN CERTIFICATE-----
      MIIE3TCCAsWgAwIBAgIDAeJAMA0GCSqGSIb3DQEBCwUAMCgxDjAMBgNVBAoTBWFk
      bWluMRYwFAYDVQQDEw1jZXJ0LWJ1aWx0LWluMB4XDTIzMDEwNDA0NDEwOFoXDTQz
      MDEwNDA0NDEwOFowKDEOMAwGA1UEChMFYWRtaW4xFjAUBgNVBAMTDWNlcnQtYnVp
      bHQtaW4wggIiMA0GCSqGSIb3DQEBAQUAA4ICDwAwggIKAoICAQCzRQtpQMcnUj15
      v43c4/MPqrQQIV0qHvLSgb1gyfoXaYheMEdh6SOCFvZtTiMjCG2T0hBmgM5u7GOD
      H8bheg9W1Q7sLsy2N5MdQMxvxR83c0Jq41bVzbG2Y1O5jnCJd1D54xC6es2zjZXw
      0hvYrLu/QVxWRWVWFpJDcy+si+AVOlpeIS5sTHcnqXCUdF69+v6Q6T7rcMHmvb50
      ym6ekNGtQ6Z7xlJzXWRBAD78fs1kp7mgmR+OydawiBV08BSvBaDaeb5NtMCUn/OF
      r3iTxkUJ9OVIDBLXEfjgSRAzh15pTNGSOkhxDfmUi8NrMvINYCGdiukaT3M6nLej
      1DtTS/N962tmcnjClxpsQ0hjhF2dTRvKSZ1WzKKj31ewYu0LX1fiKKQ/oCZgfdaE
      fcub72IZCQC/ChuEwxzRNXEJkB+L6SrGKrC6Rln0BfjO4hcGca7Ltjgws90LKOHn
      Id1tF6yk0SdtA6knMpEIALIyeSv8KZVVVCfl3gMC+kwgTKn3f5yfq+Hbh5gMxlVw
      aGbqoMIowZqdSdP9jZvAvQHPH6xeV67QG8iagnWtET1WxMvDtM+6WiinlWS9wBBH
      DKncm3lnXMXZ0Ks7ubkU6nun8fNirz1161p6WVKkPz4U44Gapf3gYnLxy40OEI6v
      /SqNREm86SoUwVYClsjA5zTC+WEf4wIDAQABoxAwDjAMBgNVHRMBAf8EAjAAMA0G
      CSqGSIb3DQEBCwUAA4ICAQAuo74CL/UUfPA2HHQwvXjaVPuCxfNyTwltDaXSpZk6
      yj5KrpATzGGBNnpjTF0Pz4wsaT66Bk0kl/T418KdHTyyKNMGM8hIWaP+Xsoq6PGJ
      nUXgs3ZZ6TbbDWNnIVAHX9Miz0pJM73ESLweivEQJ46zx3uJp+WzGu/owaD5Vvs5
      GR66pWe5Mmvle9/j3+ij1h3w5ciq3ehHk7KpIJEROGclM/0knmnpKAePalb4/Kat
      ynsL/gogtsLARJYmM+25S2GxyAyOKWGK2rKFUl1J92AaXPzZhnQRy5UWqxeiDzZJ
      8uBq/wJzVHJLotlPxpdQKD6ltZEvUTmoh0kimo59eMXyYwY8ez/edwauzGFULbs/
      Br7POOmhDFKeRaUk4EXq8joMxkb7dL3bhBwREuFxIUqZmSbz+7cfOfZbojOgK1z1
      4oLrV1XEmCAqVHqb76KGUUbea0WSXJob2jNEvbON5vOCurZb5yhQTG7NrM1pa8V9
      WR/0hjAQYXuGwHfxx7Y6rBbxMrjRv6c6rzNZLAvkvnRR4Rb/jiAho+927b789uDW
      HP+It0qOLfFbJqutCYDCtNtnfEx6aRNL1o+pM7EScOVQzv4ZSlQYGWPpdLCiW6Z9
      QB/TgT5qaD0+9sw1vXoHeONx9h/HYfu2iHXBxK0LcFHT88qJzUNr9+kAvNBVZnHz
      0A==
      -----END CERTIFICATE-----
`

///configuration/policyman-devel-master.yaml
const policyYaml = `
config:
  namespace: testing
  datasource:
    username: root
    password: ENC(D3AUzrax9H8uyCp69YBtig==) #root
iam:
  appid: 3eb93908691a3cc85ed5
  appsecret: 2c4cd3fc7f4fea75ea6f172d6a25d9e736177196
etcd:
  endpoints: http://etcd.testing:2379
spring:
  datasource:
    url: jdbc:mysql://mariadb.testing:3306/policy_manage?useUnicode=true&characterEncoding=utf8&zeroDateTimeBehavior=convertToNull&useSSL=true&serverTimezone=GMT%2B8
pulsar:
  service-url: pulsar://192.168.11.234:6650
casdoor:
  endpoint: http://gwm-iam.testing:8000
cas-user:
  org-name: built-in
  cert: cert-built-in
  client-id: c8eee4ccbab0e67ea404
  client-secret: a663244a60606b6b02becd5d12e7122d9b1c8d1b
`

///configuration/policyman-devel-master.yaml
const permissionYaml = `
ping: pong
`


function createTemplate(node: string): string{
  isCertificateRequest.value = false;
  isSignedRequest.value = false;
  isIamConfigRequest.value = false;
  isCertificate.value = false;
  isAppConfigRequest.value = false;

  if(node == "PKI证书"){
    isCertificateRequest.value = true;
    return ""
  }

  if(node == "密钥申请"){
    isSignedRequest.value = true;
    return certYaml
  }

  if(node == "IAM配置"){
    isIamConfigRequest.value = true;
    return iamAdminConfigYaml
  }

  if(node == "网关证书"){
    isIamConfigRequest.value = true;
    return iamGatewayKey
  }

  if(node == "网关业务服务"){
    isAppConfigRequest.value = true;
    return policyYaml
  }

  if(node == "权限服务配置"){
    isAppConfigRequest.value = true;
    return permissionYaml
  }

  return "";
}

const handleCertRequst = async () => {
  isCertificate.value = false;
  if(editor){
    let template = editor.getValue();
    let json: any = yaml.load(template);

    let notBefore = new Date(json.CA.validity.notBefore);
    let notAfter = new Date(json.CA.validity.notAfter);
    let now = new Date();
    

    if(notBefore < now){
      notBefore = now;
    }

    if(notAfter < now){
      notAfter.setDate(365);
    }

    if(notBefore > notAfter){
      alert("notBefore 不能大于 notAfter");
      return;
    }

    let notBeforeDays = Math.floor((notBefore - now)/(1000 * 60 * 60 * 24));
    let notAfterDays = Math.floor((notAfter - now)/(1000 * 60 * 60 * 24));

    let res =  await pki_make_ca({
      algorithm: json.CA.algorithm,
      stored: json.CA.stored,
      bitLen: json.CA.size,
      entries: json.CA.entries,
      notBefore: notBeforeDays,
      notAfter: notAfterDays
    });
    
    let results  = "";
    for(let i = 0; i < res.length; i++){
      results += res[i] + "\n";
    }

    editor.setValue(results);
  }
}

const handleCertDownload = async () => {
  if(editor){
     let keys = await pki_query_ca({});
     let results  = "";
     for(let i = 0; i < keys.length; i++){
       results += keys[i] + "\n";
     }

     editor.setValue(results);
  }
}

const handleCertReset = async () => {
  if(editor){
    isCertificate.value = true;
    editor.setValue(pkiYaml);
  }
}



const handleSignedCertRequst = async () => {
  isCertificate.value = false;
  if(editor){
    let template = editor.getValue();
    let json: any = yaml.load(template);

    let notBefore = new Date(json.CERT.validity.notBefore);
    let notAfter = new Date(json.CERT.validity.notAfter);
    let now = new Date();
    

    if(notBefore < now){
      notBefore = now;
    }

    if(notAfter < now){
      notAfter.setDate(365);
    }

    if(notBefore > notAfter){
      alert("notBefore 不能大于 notAfter");
      return;
    }

    let notBeforeDays = Math.floor((notBefore - now)/(1000 * 60 * 60 * 24));
    let notAfterDays = Math.floor((notAfter - now)/(1000 * 60 * 60 * 24));

    let res =  await mk_signed_cert({
      algorithm: json.CERT.algorithm,
      bitLen: json.CERT.size,
      dns: json.CERT.dns,
      entries: json.CERT.entries,
      notBefore: notBeforeDays,
      notAfter: notAfterDays
    });
    
    let results  = "";
    for(let i = 0; i < res.length; i++){
      results += res[i] + "\n";
    }

    editor.setValue(results);
  }
}

function flattenObject(obj: any,parentKey = '') {
  return Object.keys(obj).reduce((accumulator: any, key) => {
    const fullKey = parentKey ? `${parentKey}/${key}` : key;

    if (typeof obj[key] === 'object' && obj[key] !== null) {
      // Recursively flatten nested objects
      Object.assign(accumulator, flattenObject(obj[key], fullKey));
    } else {
      // Concatenate the flattened key-value pair
      accumulator["/"+fullKey] = obj[key];
    }

    return accumulator;
  }, {});
}

const handleIamConfigRequst = async () => {
  isCertificate.value = false;
  if(editor){
    let template = editor.getValue();
    let json: any = yaml.load(template);

    let data = flattenObject(json);
    await etcd_put_mapkeys({entries: data});
  }
}


const handleAppConfigRequst = async () => {
  isCertificate.value = false;
  if(editor){
    let template = editor.getValue();
    let key = selectNodePathKey.value;
    let data: any = {};
    data[key] = template;

    await etcd_put_mapkeys({entries: data});
  }
}


</script>
<style scoped  lang="scss">
.toolBar {
  align-content: end;
  align-items: center;
}

</style>