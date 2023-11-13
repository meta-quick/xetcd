export default [
    {
      name: '/serviceman/index',
      path: '/serviceman/index',
      meta: { title: '业务管理', isNav: true, isMenu: true},
      component: () => import('@/views/serviceman/home.vue'),
    },
  ];
