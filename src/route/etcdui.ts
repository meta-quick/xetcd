export default [
    {
      name: 'etcdui',
      path: '/dbman',
      meta: { title: '应用模型管理', isNav: true },
      redirect: '/applicationModel/functionModule',
      component: () => import('@/views/model/appModel/index.vue'),
      children: [
        {
          name: 'functionModule',
          path: '/applicationModel/functionModule',
          meta: { title: '功能模块列表', isNav: false },
          component: () =>
            import('@/views/model/appModel/views/functionModule/index.vue')
        }
      ]
    },
  ];
  