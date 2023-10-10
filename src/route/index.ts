import { RouteRecordRaw } from 'vue-router';
import PageLayout from '@/layout/index.vue';

// import BusinessModel from './etcdui';

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'layout',
    component: PageLayout,
    redirect: '/applicationModel/functionModule',
    // meta: { title: '业务模型管理', isNav: true },
    children: [
    //   ...BusinessModel,
    ]
  }
];

export default routes;
