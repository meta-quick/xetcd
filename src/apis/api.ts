import { da } from 'element-plus/es/locale/index.mjs'
import {rcall} from './tauri.ts'

export const save_cluster = async (data: any): Promise<string> => {
  return await rcall("savecluster",{cluster: JSON.stringify(data)})
}

export const query_cluster = async (): Promise<string> =>{
  return await rcall("querycluster",{req: ""})
}


export const echo = async (data: any): Promise<string>=> {
  return await rcall("echo",{req: JSON.stringify(data)})
}

export const queryinstance = async (data: any): Promise<string>=> {
  return await rcall("queryselectedinstance",{req: JSON.stringify(data)})
}

export const saveinstance = async (data: any): Promise<string>=> {
  return await rcall("saveselectedinstance",{req: JSON.stringify(data)})
}

export const get_all_keys = async (): Promise<any>=> {
  return await rcall("etcd_all_key",{})
}

export const etc_put_key = async (data: any): Promise<any>=> {
  return await rcall("etcd_put_key",{req: data})
}

export const etc_delete_key = async (data: any): Promise<any>=> {
  return await rcall("etcd_delete_key",{req: data})
}

export const etcd_simpleput_key = async (data: any): Promise<any>=> {
  return await rcall("etcd_simpleput_key",{req: data})
}

export const etcd_get_all_roles = async (): Promise<any>=> {
  return await rcall("list_roles",{})
}

export const etcd_get_role_permissions = async (data: any): Promise<any>=> {
  return await rcall("list_roles_permission",{req: data})
}

export const etcd_add_role = async (data: any): Promise<any>=> {
  return await rcall("etcd_add_role",{req: data})
}

export const etcd_del_role = async (data: any): Promise<any>=> {
  return await rcall("etcd_del_role",{req: data})
}

export const etcd_role_grant_perimssions = async (data: any): Promise<any>=> {
  console.log(data)
  return await rcall("etcd_role_grant_perimssions",data)
}

export const etcd_role_revoke_perimssions = async (data: any): Promise<any>=> {
  return await rcall("etcd_role_revoke_perimssions",data)
}

export const etcd_user_list = async (): Promise<any>=> {
  return await rcall("etcd_user_list",{})
}

export const etcd_user_role_list = async (data: any): Promise<String[]>=> {
  return await rcall("etcd_user_role_list",{name: data})
}

export const etcd_user_add = async (data: any): Promise<String[]>=> {
  return await rcall("etcd_user_add",data)
}

export const etcd_user_delete = async (data: any): Promise<String[]>=> {
  return await rcall("etcd_user_delete",{name: data})
}

export const user_grant_role = async (data: any): Promise<String[]>=> {
  return await rcall("user_grant_role",data)
}

export const user_revoke_role = async (data: any): Promise<String[]>=> {
  return await rcall("user_revoke_role",data)
}

export const pki_make_ca = async (data: any): Promise<String[]>=> {
  return await rcall("pki_make_ca",data)
}

export const pki_query_ca = async (data: any): Promise<String[]>=> {
  return await rcall("pki_query_ca",data)
}

export const mk_signed_cert = async (data: any): Promise<String[]>=> {
  return await rcall("mk_signed_cert",data)
}

export const etcd_put_mapkeys = async (data: any): Promise<String[]>=> {
  return await rcall("etcd_put_mapkey",data)
}

