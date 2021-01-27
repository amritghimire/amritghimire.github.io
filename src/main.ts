import Vue from "vue";
// eslint-disable-next-line @typescript-eslint/ban-ts-ignore
// @ts-ignore
import Vuesax from "vuesax";
import VueMeta from "vue-meta";

import App from "./App.vue";
import "./registerServiceWorker";
import router from "./router";
import store from "./store";
import "@/utils/hooks";
import "vuesax/dist/vuesax.css";
Vue.config.productionTip = false;

Vue.use(VueMeta);
Vue.use(Vuesax, {
  theme: {
    colors: {
      primary: "#5b3cc4",
      success: "rgb(23, 201, 100)",
      danger: "rgb(242, 19, 93)",
      warning: "rgb(255, 130, 0)",
      dark: "rgb(36, 33, 69)"
    }
  }
});

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount("#app");
