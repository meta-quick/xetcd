import { RouteRecordRaw } from 'vue-router';
import PageLayout from '@/layout/index.vue';

import DBRoutes from './etcd/index.ts';
import ClusterRoutes from './etcd/cluster.ts'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'home',
    component: PageLayout,
    // redirect: "/etcdman/runtime",
    // children: [
    //   // ...DBRoutes,
    // ]
  },
  {
    path: '/etcdman/data',
    name: 'dataman',
    component: PageLayout,
    meta: { title: '数据配置', activeIndex: true, isNav: true,isMenu: true },
    children: [
      ...DBRoutes,
    ]
  },
  {
    path: '/etcdman/runtime',
    name: 'runtimeman',
    component: PageLayout,
    meta: { title: '集群配置', isNav: true,isMenu: true },
    redirect: '/etcdman/runtime/index',
    children: [
      ...DBRoutes,
    ]
  },
  {
    path: '/etcdman/role',
    name: 'roleman',
    component: PageLayout,
    // redirect: '/etcdman/index',
    meta: { title: '授权管理', isNav: true,isMenu: true },
    // children: [
    //   ...DBRoutes,
    // ]
  },
  {
    path: '/etcdman/user',
    name: 'userman',
    component: PageLayout,
    // redirect: '/etcdman/index',
    meta: { title: '用户管理', isNav: true,isMenu: true },
    // children: [
    //   ...DBRoutes,
    // ]
  },
  {
    path: '/etcdman/service',
    name: 'serviceman',
    component: PageLayout,
    // redirect: '/etcdman/index',
    meta: { title: '业务模型管理', isNav: true,isMenu: true },
    // children: [
    //   ...DBRoutes,
    // ]
  },
  {
    path: '/etcdman/cluster',
    name: 'clusterman',
    component: PageLayout,
    // redirect: '/etcdman/index',
    meta: { title: '集群维护', isNav: true,isMenu: true },
    // children: [
    //   ...DBRoutes,
    // ]
  }
];

export default routes;
