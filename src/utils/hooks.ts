import Component from "vue-class-component";

interface VueSax {
  loading: Function;
  notification: Function;
}

// 2. Specify a file with the types you want to augment
//    Vue has the constructor type in types/vue.d.ts
declare module "vue/types/vue" {
  // 3. Declare augmentation for Vue
  interface Vue {
    $vs: VueSax;
  }
}
// Register the router hooks with their names
Component.registerHooks([
  "beforeRouteEnter",
  "beforeRouteLeave",
  "beforeRouteUpdate"
]);
