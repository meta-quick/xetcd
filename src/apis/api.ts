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