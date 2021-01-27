<template>
  <span class="image">
    <img :src="baseUrl" class="image-img" />
  </span>
</template>

<script>
import applyBase from "@/utils/applyBase";

export default {
  name: "InternalImageContent",
  props: ["src"],
  computed: {
    baseUrl: function() {
      return applyBase(this.src);
    }
  }
};
</script>

<style scoped>
.image-img {
  max-width: 100%;
  max-height: 100%;
  background-position: center center;
  background-repeat: no-repeat;
}
</style>
