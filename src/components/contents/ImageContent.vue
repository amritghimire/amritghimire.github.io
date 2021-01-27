<template>
  <vs-row
    class="image-text-content"
    type="flex"
    :style="{
      'background-color': content.background,
      color: content.text_color
    }"
    justify="center"
    align="center"
  >
    <vs-col lg="10" sm="11" xs="12" justify="left" class="text--html">
      <InternalImageContent
        :src="content.media"
        class="image"
      ></InternalImageContent>
    </vs-col>
  </vs-row>
</template>

<script>
import InternalImageContent from "@/components/contents/internal/InternalImageContent";

export default {
  name: "ImageContent",
  props: ["content"],
  components: {
    InternalImageContent
  }
};
</script>

<style scoped>
.image {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 16px;
}
</style>
