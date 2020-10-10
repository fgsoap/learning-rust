import Vue from 'vue'
import App from './app'
import TextHighlight from 'vue-text-highlight';

Vue.component('text-highlight', TextHighlight);

new Vue({
  render: h => h(App)
}).$mount('#app')
