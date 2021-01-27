<template>
  <div>
    <vs-row class="blog-content" type="flex" justify="center">
      <vs-col
        type="flex"
        justify="center"
        lg="3"
        sm="6"
        xs="12"
        class="blog-col"
      >
        <vs-card
          type="1"
          :style="{
            'background-color': content.background,
            color: content.text_color
          }"
        >
          <template #title v-if="content.header">
            <span v-html="content.header"></span>
          </template>
          <template #img v-if="content.image">
            <a :href="content.link" class="text-link">
              <img :src="baseUrl" class="image-img" />
            </a>
          </template>
          <span slot="text">
            <a :href="content.link" class="text-link">
              <span v-html="content.extra_content"></span>
            </a>
          </span>
          <template #interactions>
            <span v-html="content.footer"></span>
          </template>
        </vs-card>
      </vs-col>
    </vs-row>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import applyBase from "@/utils/applyBase";

const CardContentProps = Vue.extend({
  props: {
    content: {
      type: Object,
      default: () => ({})
    }
  }
});

@Component
export default class CardContent extends CardContentProps {
  get baseUrl() {
    return applyBase(this.content.image);
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
.text-link {
  text-decoration: none;
  color: inherit;
}
</style>
