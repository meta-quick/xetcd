export default [
    {
      name: '/role/index',
      path: '/role/index',
      meta: { title: '权限模型管理', isNav: true, isMenu: true},
      component: () => import('@/views/role/home.vue'),
    },
  ];