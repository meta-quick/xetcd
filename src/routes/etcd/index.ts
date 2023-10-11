export default [
    {
      name: 'etcdui',
      path: '/etcdman/index',
      meta: { title: '应用模型管理', isNav: true, isMenu: true},
      // redirect: '/dbman/welcome',
      component: () => import('@/views/misc/404.vue'),
    },
  ];
  