import { createApp } from "vue";
import PrimeVue from 'primevue/config';
import Ripple from 'primevue/ripple';
import StyleClass from 'primevue/styleclass';
import Tooltip from 'primevue/tooltip';
import 'primevue/resources/themes/bootstrap4-light-blue/theme.css';
import 'primevue/resources/primevue.min.css';
import 'primeicons/primeicons.css';
import 'primeflex/primeflex.min.css'

import {
    createRouter,
    createWebHistory,
  } from 'vue-router';
  
import ElementPlus from 'element-plus'
import "./styles/normalize.css"
import "virtual:windi.css";
import 'element-plus/dist/index.css'
import routes from './routes';
import store from "./stores";
import "./styles/styles.css";
import App from "./App.vue";


//Create VUE route definition
const router = createRouter({
    history: createWebHistory(
    ),
    routes
  });
  


const app = createApp(App);
app.use(ElementPlus);
app.use(PrimeVue,{ripple: true,csp: {
  nonce: 'prime'
}, inputStyle: "outlined"
});
app.use(router);
app.use(store);


app.directive('ripple', Ripple);
app.directive('styleclass', StyleClass);
app.directive('tooltip', Tooltip);

app.mount("#app");
