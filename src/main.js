import Vue from 'vue'
import App from './App.vue'
import ViewUI from 'view-design'

import { initContract } from './utils'

Vue.use(ViewUI)
Vue.config.productionTip = false

window.nearInitPromise = initContract().then(() => {
  new Vue({
    render: (h) => h(App),
  }).$mount('#app')
})
