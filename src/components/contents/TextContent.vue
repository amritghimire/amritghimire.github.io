<template>
  <vs-row
    class="image-text-content"
    type="flex"
    :style="{
      'background-color': content.background,
      color: content.text_color || 'white'
    }"
    justify="center"
  >
    <vs-col
     lg="10"
     sm="11"
     xs="12"
     justify="left"
      class="text--html"
    >
      <InternalHTMLTextContent :html="content.body"></InternalHTMLTextContent>
    </vs-col>
  </vs-row>
</template>

<script>
import InternalHTMLTextContent from "@/components/contents/internal/InternalHTMLTextContent";

export default {
  name: "TextContent",
  props: ["content"],
  components: {
    InternalHTMLTextContent
  }
};
</script>

<style scoped>
.text--html {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 16px;
}
</style>
