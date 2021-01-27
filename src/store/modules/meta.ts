import Vue from "vue";
import { MetaState } from "@/store/modules/types";
import { MARK_META_FETCHED, SET_META_OBJECT, SET_SINGLE_META } from "@/store/mutationTypes";
import { MetaValue } from "@/models/meta";
import { listMeta } from "@/services/meta";

const state = (): MetaState => ({
  updatedFromAPI: false,
  metaValues: {}
});

const getters = {
  getState: (state: MetaState) => (name: string, fallback: string) => {
    return state.metaValues[name] || fallback;
  }
};

const actions = {
  async fetchMetas({ commit }: { commit: Function }) {
    const metas = await listMeta();
    commit(SET_META_OBJECT, metas);
    commit(MARK_META_FETCHED);
  }
};

const mutations = {
  [SET_SINGLE_META](state: MetaState, payload: MetaValue) {
    if (payload.name) {
      const value = payload.value || payload.html_value;
      Vue.set(state.metaValues, payload.name, value);
    }
  },
  [SET_META_OBJECT](state: MetaState, payload: Array<MetaValue>) {
    const metas: { [index: string]: string } = {};
    payload.forEach(meta => {
      if (meta.name) {
        metas[meta.name] = meta.value || meta.html_value || "";
      }
    });
    state.metaValues = metas;
  },
  [MARK_META_FETCHED](state: MetaState) {
    state.updatedFromAPI = true;
  }
};

export default {
  namespaced: true,
  state,
  getters,
  actions,
  mutations
};
