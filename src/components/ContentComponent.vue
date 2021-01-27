<template>
  <div class="content-container">
    <ImageTextContent
      v-bind:content="content"
      v-if="content.type === 'ImageTextContent'"
    ></ImageTextContent>
    <TextContent
      :content="content"
      v-else-if="content.type === 'TextContent'"
    ></TextContent>
    <CollapseComponent
      :content="content"
      v-else-if="content.type === 'CollapseBoxContent'"
    ></CollapseComponent>
    <BlogPostContent
      :content="content"
      v-else-if="content.type === 'BlogPostContent'"
    ></BlogPostContent>
    <ImageContent
      :content="content"
      v-else-if="content.type === 'ImageContent'"
    ></ImageContent>
    <ContactContent
      :content="content"
      v-else-if="content.type === 'ContactContent'"
    ></ContactContent>
    <CardContent
      :content="content"
      v-else-if="content.type === 'CardContent'"
    ></CardContent>
    <ImageBoxContent
      :content="content"
      v-else-if="content.type === 'ImageBoxContent'"
    ></ImageBoxContent>
    <CardCollectionContent
      :content="content"
      v-else-if="content.type === 'CardCollectionContent'"
    ></CardCollectionContent>
    <DualColumnTextContent
      :content="content"
      v-else-if="content.type === 'DualColumnTextContent'"
    ></DualColumnTextContent>

    <div v-else>
      {{ content.type }}
    </div>
  </div>
</template>

<script>
import Vue from "vue";
import ImageTextContent from "@/components/contents/ImageTextContent";
import TextContent from "@/components/contents/TextContent";
import CollapseComponent from "@/components/contents/CollapseComponent";
import BlogPostContent from "@/components/contents/BlogPostContent";
import ImageContent from "@/components/contents/ImageContent";
import ContactContent from "@/components/contents/ContactContent";
import CardContent from "@/components/contents/CardContent";
import ImageBoxContent from "@/components/contents/ImageBoxContent";
import CardCollectionContent from "@/components/contents/CardCollectionContent";
import DualColumnTextContent from "@/components/contents/DualColumnTextContent";

import Component from "vue-class-component";

@Component({
  components: {
    ImageTextContent,
    TextContent,
    CollapseComponent,
    BlogPostContent,
    ImageContent,
    ContactContent,
    CardContent,
    ImageBoxContent,
    CardCollectionContent,
    DualColumnTextContent
  },
  props: ["content"]
})
export default class ContentComponent extends Vue {}
</script>

<style scoped>
.content-container {
  max-width: 100%;
}
</style>
