<template>
  <div class="post-box">
    <div v-if="blog">
      <vs-row>
        <vs-col
          v-if="blog.slug !== 'home'"
          type="flex"
          w="12"
          justify="center"
          class="page-title"
        >
          <h1>{{ blog.title }}</h1>
        </vs-col>
      </vs-row>
      <div v-if="blog.contents">
        <vs-row
          v-for="content in blog.contents"
          v-bind:key="content.id"
          justify="center"
          class="each-contents"
        >
          <vs-col vs-w="12">
            <ContentComponent v-bind:content="content"></ContentComponent>
          </vs-col>
        </vs-row>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import { getBlog } from "@/services/blog";
import ContentComponent from "@/components/ContentComponent.vue";
import { Blog } from "@/models/blog";
import Component from "vue-class-component";
import { Route } from "vue-router/types/router";
import { Watch } from "vue-property-decorator";
import { config } from "@/config";
import { getMeta } from "@/utils/api";

@Component({
  name: "PostPage",
  components: {
    ContentComponent
  },
  metaInfo: {
    title: getMeta()["title"] || ""
  }
})
export default class PostPage extends Vue {
  slug = "home";
  blog: Blog | null = null;

  async fetchPage(newSlug: string) {
    const loading = this.$vs.loading({
      type: "circles",
      color: "primary",
      text: "Loading...",
      opacity: 1,
      scale: 2
    });
    const blog = await getBlog(newSlug);
    if (blog === null) {
      await this.$router.push({ name: "404" });
    } else {
      this.blog = blog;
      document.title = `${blog.title || ""} - ${getMeta().title}`;
    }
    loading.close();
  }

  mounted() {
    this.slug = this.$route.params.slug;
    if (this.slug === "home") {
      this.fetchPage("home");
    }
  }

  beforeRouteUpdate(to: Route, from: Route, next: Function) {
    this.slug = to.params.slug;
    next();
  }

  @Watch("slug")
  onSlugChanged(value: string, oldValue: string) {
    this.fetchPage(value);
  }
}
</script>

<style scoped>
.post-box {
  padding: 64px 0;
}

.each-contents {
  margin-bottom: -8px;
}

.page-title {
  margin-bottom: 32px;
}
</style>
