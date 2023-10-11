export default [
    {
      name: '/runtime/index',
      path: '/runtime/index',
      meta: { title: '应用模型管理', isNav: true, isMenu: true},
      component: () => import('@/views/cluster/home.vue'),
    },
  ];