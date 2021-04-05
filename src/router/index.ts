import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router'
import About from '../views/About.vue'
import Download from '../views/Download.vue'
import Help from '../views/Help.vue'
import Upload from '../views/Upload.vue'
import UI from '../views/Ui.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/characters/:selectedCharacter',
    name: 'UI',
    component: UI,
    props: true,
  },
  {
    path: '/about',
    name: 'About',
    component: About
  },
  {
    path: '/upload',
    name: 'Upload',
    component: Upload
  },
  {
    path: '/',
    redirect: '/upload'
  },
  {
    path: '/download',
    name: 'Download',
    component: Download
  },
  {
    path: '/help',
    name: 'Help',
    component: Help
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
