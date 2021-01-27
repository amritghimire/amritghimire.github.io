<template>
  <div
    class="footer"
    :style="{
      'background-color': footerBackground,
      color: footerColor
    }"
  >
    <span v-html="footer"></span>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";


@Component
export default class Footer extends Vue{
  get footer() {
    return this.$store.getters["meta/getState"]("footer", "");
  }
  get footerBackground() {
    return this.$store.getters["meta/getState"]("footerBackground", "blue");
  }
  get footerColor() {
    return this.$store.getters["meta/getState"]("footerColor", "white");
  }
}
</script>

<style scoped>
.footer {
  padding: 32px 0;
  justify-content: center;
}
</style>
