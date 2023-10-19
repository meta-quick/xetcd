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
