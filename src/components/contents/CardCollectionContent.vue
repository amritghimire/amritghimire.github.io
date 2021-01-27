<template>
  <vs-row
    class="image-text-content"
    :style="{
      'background-color': content.background,
      color: content.text_color
    }"
    justify="center"
    align="center"
  >
    <vs-col
      lg="6"
      sm="6"
      xs="12"
      justify="left"
      class="text--html image-box"
      v-for="sub_content in content.cardcontent_set || []"
      :key="sub_content.id"
    >
      <CardContent :content="sub_content" class="image"></CardContent>
    </vs-col>
  </vs-row>
</template>

<script>
import CardContent from "@/components/contents/CardContent";

export default {
  name: "CardCollectionContent",
  props: ["content"],
  components: {
    CardContent
  }
};
</script>

<style scoped>

</style>
