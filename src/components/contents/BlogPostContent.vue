<template>
  <div>
    <vs-row>
      <vs-col w="12" class="title-home">
        <h1>{{ content.title || "" }}</h1>
      </vs-col>
    </vs-row>
    <vs-row class="blog-content" type="flex">
      <vs-col
        type="flex"
        justify="center"
        lg="3"
        sm="6"
        xs="12"
        v-for="blog in blogs"
        :key="blog.id"
        class="blog-col"
      >
        <InternalBlogCard
          v-bind:blog="blog"
          v-bind:type="content.style"
          class="blog-card"
        ></InternalBlogCard>
      </vs-col>
    </vs-row>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import { listBlog } from "@/services/blog";
import InternalBlogCard from "@/components/contents/internal/InternalBlogCard.vue";
import { Blog } from "@/models/blog";
import Component from "vue-class-component";

const BlogPostProps = Vue.extend({
  props: {
    content: {
      type: Object,
      default: () => ({})
    }
  },
  components: {
    InternalBlogCard
  }
});

@Component
export default class BlogPostContent extends BlogPostProps {
  blogs: Array<Blog> = [];

  async mounted() {
    const params = this.content.params || {};
    this.blogs = await listBlog({
      ...params,
      limit: this.content.count || 4
    });
  }
}
</script>

<style scoped>
.blog-content {
  padding: 16px;
}

.blog-card {
  margin: 16px;
  padding-top: 16px;
}

.blog-col {
  margin-top: 16px;
}

.title-home {
  padding-top: 64px;
}
</style>
