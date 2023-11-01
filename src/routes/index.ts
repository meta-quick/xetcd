import { RouteRecordRaw } from 'vue-router';
import PageLayout from '@/layout/index.vue';

import DataManRoutes from './etcd/dataman.ts';
import ClusterRoutes from './etcd/cluster.ts';
import RoleRoutes from './etcd/role.ts';
import UserRoutes from './etcd/user.ts'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'home',
    component: PageLayout,
  },
  {
    path: '/dataman',
    name: 'dataman',
    component: PageLayout,
    meta: { title: '数据配置', activeIndex: true, isNav: true,isMenu: true },
    redirect: "/dataman/index",
    children: [
      ...DataManRoutes,
    ]
  },
  {
    path: '/runtime',
    name: 'runtimeman',
    component: PageLayout,
    meta: { title: '集群配置', isNav: true,isMenu: true },
    redirect: '/runtime/index',
    children: [
      ...ClusterRoutes,
    ]
  },
  {
    path: '/role',
    name: 'roleman',
    component: PageLayout,
    redirect: '/role/index',
    meta: { title: '授权管理', isNav: true,isMenu: true },
    children: [
      ...RoleRoutes,
    ]
  },
  {
    path: '/user',
    name: 'userman',
    component: PageLayout,
    redirect: '/user/index',
    meta: { title: '用户管理', isNav: true,isMenu: true },
    children: [
      ...UserRoutes,
    ]
  },
  {
    path: '/service',
    name: 'serviceman',
    component: PageLayout,
    // redirect: '/etcdman/index',
    meta: { title: '业务模型管理', isNav: true,isMenu: true },
    // children: [
    //   ...DBRoutes,
    // ]
  },
  {
    path: '/cluster',
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
