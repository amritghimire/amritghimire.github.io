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
      v-if="content.text_before_image"
      class="image-text--html"
    >
      <InternalHTMLTextContent :html="content.text"></InternalHTMLTextContent>
    </vs-col>
    <vs-col
      order="2"
      lg="6"
      sm="12"
      xs="12"
      type="flex"
      justify="space-evenly"
      v-if="content.text_before_image"
      class="image-text--image"
    >
      <InternalImageContent :src="content.image"></InternalImageContent>
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
      <InternalHTMLTextContent :html="content.text"></InternalHTMLTextContent>
    </vs-col>
    <vs-col
      order="1"
      lg="6"
      sm="12"
      xs="12"
      type="flex"
      justify="space-evenly"
      v-if="!content.text_before_image"
      class="image-text--image"
    >
      <InternalImageContent :src="content.image"></InternalImageContent>
    </vs-col>
  </vs-row>
</template>

<script>
import InternalHTMLTextContent from "@/components/contents/internal/InternalHTMLTextContent";
import InternalImageContent from "@/components/contents/internal/InternalImageContent";

export default {
  name: "ImageTextContent",
  props: ["content"],
  components: {
    InternalImageContent,
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
