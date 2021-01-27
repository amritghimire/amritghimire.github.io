import Vue from "vue";
import Vuex from "vuex";
import VuexPersistence from "vuex-persist";

import meta from "@/store/modules/meta";

Vue.use(Vuex);

const vuexLocal = new VuexPersistence({
  key: "metaValues",
  modules: ["meta"]
});

export default new Vuex.Store({
  state: {},
  mutations: {},
  actions: {},
  modules: {
    meta
  },
  plugins: [vuexLocal.plugin]
});
