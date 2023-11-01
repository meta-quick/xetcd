export default [
    {
      name: '/user/index',
      path: '/user/index',
      meta: { title: '用户管理', isNav: true, isMenu: true},
      component: () => import('@/views/user/home.vue'),
    },
  ];