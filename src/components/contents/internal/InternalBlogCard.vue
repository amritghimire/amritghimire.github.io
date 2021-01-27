<template>
  <vs-card type="1">
    <template #title v-if="enabledList.includes('title')">
      <router-link :to="{ name: 'post', params: { slug: blog.slug } }" class="text">
        <h3 class="text">{{ blog.title }}</h3>
      </router-link>
    </template>
    <template #img v-if="enabledList.includes('image')">
      <router-link :to="{ name: 'post', params: { slug: blog.slug } }">
        <img :src="blog.media" class="image-img" :alt="blog.title" />
      </router-link>
    </template>
    <span slot="text">
      <router-link :to="{ name: 'post', params: { slug: blog.slug } }" class="excerpt">
        <span
          class="content truncate-overflow text"
          v-if="enabledList.includes('excerpt')"
        >
          {{ blog.excerpt }}
        </span>
      </router-link>
    </span>
    <template #interactions> </template>
  </vs-card>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import { Blog } from "@/models/blog";

const InternalBlogCardProps = Vue.extend({
  props: {
    type: String,
    blog: Blog
  }
});

@Component
export default class InternalBlogCard extends InternalBlogCardProps {
  get enabledList(): Array<string> {
    return this.type.split(",");
  }
}
</script>

<style scoped>
.image-img {
  max-width: 100%;
  max-height: 100%;
  background-position: center center;
  background-repeat: no-repeat;
}

.excerpt {
  font-size: large;
  font-weight: bold;
  --lh: 1.4rem;
  line-height: var(--lh);
}

.content.truncate-overflow {
  --max-lines: 7;
  position: relative;
  max-height: calc(var(--lh) * var(--max-lines));
  overflow: hidden;
  padding-right: 1rem;
  display: inline-block;
}

.content.truncate-overflow::before {
  position: absolute;
  content: "...";
  bottom: 0;
  right: 0;
  z-index: 1;
}

.content.truncate-overflow::after {
  content: "";
  position: absolute;
  bottom: 0;
  right: 0;
  width: 1rem;
  height: 1rem;
  background: #fff;
  z-index: 0;
}

.text {
  color: black;
  text-decoration: none;
}
</style>
