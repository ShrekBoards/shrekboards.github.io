import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router'
import Download from '../views/Download.vue'
import Home from '../views/Home.vue'
import Upload from '../views/Upload.vue'
import UI from '../views/Ui.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/about',
    name: 'About',
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () => import(/* webpackChunkName: "about" */ '../views/About.vue')
  },
  {
    path: '/upload',
    name: 'Upload',
    component: Upload
  },
  {
    path: '/ui/:selectedCharacter',
    name: 'UI',
    component: UI,
    props: true
  },
  {
    path: '/download',
    name: 'Download',
    component: Download
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
