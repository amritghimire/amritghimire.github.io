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
    <vs-col
      lg="3"
      sm="4"
      xs="12"
      justify="left"
      class="text--html image-box"
      v-for="sub_content in content.imagecontent_set || []"
      :key="sub_content.id"
    >
      <span v-on:click="openImage(sub_content)">
        <ImageContent :content="sub_content" class="image"></ImageContent>
      </span>
    </vs-col>
    <vs-dialog not-close auto-width not-padding v-model="modelActive">
      <div class="con-image">
        <img :src="activeSrc" alt="" class="image" />
      </div>
    </vs-dialog>
  </vs-row>
</template>

<script>
import ImageContent from "@/components/contents/ImageContent";
import applyBase from "@/utils/applyBase";

export default {
  name: "ImageBoxContent",
  props: ["content"],
  components: {
    ImageContent
  },
  data: function() {
    return {
      modelActive: false,
      activeSrc: ""
    };
  },
  methods: {
    openImage: function(content) {
      this.activeSrc = applyBase(content.media);
      this.modelActive = !this.modelActive;
    }
  }
};
</script>

<style scoped>
.image {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  max-width: 100%;
}

.image-box {
  cursor: pointer;
}
</style>
