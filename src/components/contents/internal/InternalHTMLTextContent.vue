<template>
  <span v-html="html" class="internal-html-component"></span>
</template>

<script>
export default {
  name: "InternalHTMLTextContent",
  props: ["html"]
};
</script>

<style scoped>
.internal-html-component {
  width: 100%;
  padding: 16px;
}
h1 {
  font-size: 36px;
}
</style>
