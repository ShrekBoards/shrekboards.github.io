import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router'
import About from '../views/About.vue'
import Download from '../views/Download.vue'
import Fields from '../views/HelpFields.vue'
import Gamecube from '../views/HelpGamecube.vue'
import Help from '../views/Help.vue'
import Options from '../views/HelpOptions.vue'
import PC from '../views/HelpPC.vue'
import Upload from '../views/Upload.vue'
import UI from '../views/Ui.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/characters/:selected',
    name: 'CharacterUi',
    component: UI,
    props: route => ({ gametype: 'characters', selected: route.params.selected }),
  },
  {
    path: '/stages/:selected',
    name: 'StageUi',
    component: UI,
    props: route => ({ gametype: 'stages', selected: route.params.selected }),
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
  },
  {
    path: '/help/fields',
    name: 'Fields',
    component: Fields
  },
  {
    path: '/help/gamecube',
    name: 'Gamecube',
    component: Gamecube
  },
  {
    path: '/help/pc',
    name: 'PC',
    component: PC
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
