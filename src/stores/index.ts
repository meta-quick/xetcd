import { defineStore, createPinia } from 'pinia';

const store = createPinia();

export default store;

const cacheState: any = {
  data: {},
  options: {}
};
// Create shared store instance.
export const useCachedStore = defineStore('cached', {
  state: () => cacheState,
  actions: {
    addObject(key: string, value: any) {
      this.data[key] = value;
    },
    get(key: string) {
      return this.data[key];
    },
    removeOject(key: string) {
      delete this.data[key];
    },
    setOption(key: string, value: any) {
      this.options[key] = value;
    },
    getOption(key: string) {
      return this.options[key];
    }
  }
});


const defaultPageState: any = {
    keepAlive: [], // 缓存页面
    refresh: {
    }
  };
  
  // Create a new store instance.
  export const usePageState = defineStore('page', {
    state: () => defaultPageState,
    actions: {
        setKeepAlive(value: string[]) {
            this.keepAlive = value;
          },      
    }
  });  
