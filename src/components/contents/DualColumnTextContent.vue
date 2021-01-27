<template>
  <vs-row
    class="image-text-content"
    type="flex"
    :style="{
      'background-color': content.background,
      color: content.text_color
    }"
  >
    <vs-col
      order="1"
      lg="6"
      sm="12"
      xs="12"
      type="flex"
      justify="left"
      class="image-text--html"
    >
      <InternalHTMLTextContent :html="content.left"></InternalHTMLTextContent>
    </vs-col>
    <vs-col
      order="2"
      lg="6"
      sm="12"
      xs="12"
      type="flex"
      justify="left"
      v-if="!content.text_before_image"
      class="image-text--html"
    >
      <InternalHTMLTextContent :html="content.right"></InternalHTMLTextContent>
    </vs-col>
  </vs-row>
</template>

<script>
import InternalHTMLTextContent from "@/components/contents/internal/InternalHTMLTextContent";

export default {
  name: "DualColumnTextContent",
  props: ["content"],
  components: {
    InternalHTMLTextContent
  }
};
</script>

<style scoped>
.image-text--html {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}
</style>
