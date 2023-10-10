import { createApp } from "vue";
import {
    createRouter,
    createWebHistory,
  } from 'vue-router';
  
import ElementPlus from 'element-plus'

import "virtual:windi.css";
import 'element-plus/dist/index.css'
import routes from './route';
import "./styles.css";
import App from "./App.vue";


//Create VUE route definition
const router = createRouter({
    history: createWebHistory(
    ),
    routes
  });
  


const app = createApp(App);
app.use(ElementPlus);
app.use(router);

app.mount("#app");
