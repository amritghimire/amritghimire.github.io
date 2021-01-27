<template>
  <div class="center examplex">
    <vs-navbar fixed shadow>
      <div slot="left">
        <router-link :to="{ name: 'Home' }" class="title">
          <h2>{{ title }}</h2>
        </router-link>
      </div>
      <template #right>
        <span class="right-template center-menu hidden-on-mobile">
          <span v-for="single_menu in menus" v-bind:key="single_menu.id">
            <vs-navbar-group
              v-if="single_menu.children && single_menu.children.length"
            >
              <router-link
                :to="{ name: 'post', params: { slug: single_menu.blog.slug } }"
                class="nav-link"
                >{{ single_menu.name }}
              </router-link>
              <template #items>
                <vs-navbar-item
                  v-for="child in single_menu.children || []"
                  :key="child.id"
                  class="sub-menu"
                >
                  <router-link
                    :to="{ name: 'post', params: { slug: child.blog.slug } }"
                    class="nav-link"
                    >{{ child.name }}
                  </router-link>
                </vs-navbar-item>
              </template>
            </vs-navbar-group>
            <vs-navbar-item v-else>
              <router-link
                :to="{ name: 'post', params: { slug: single_menu.blog.slug } }"
                class="nav-link"
                >{{ single_menu.name }}
              </router-link>
            </vs-navbar-item>
          </span>
        </span>
        <vs-button
          @click="activeSidebar = !activeSidebar"
          flat
          icon
          class="visible-on-mobile"
        >
          Menu
        </vs-button>
      </template>
    </vs-navbar>
    <div class="hidden">
      <vs-sidebar :open.sync="activeSidebar" right>
        <template #logo>
          <router-link :to="{ name: 'Home' }" class="title">
            <h2>{{ title }}</h2>
          </router-link>
        </template>

        <vs-sidebar-group
          v-for="single_menu in menus"
          v-bind:key="single_menu.id"
        >
          <template #header>
            <vs-sidebar-item
              v-bind:arrow="
                single_menu.children && single_menu.children.length > 0
              "
            >
              <template #icon>
                >
              </template>
              <router-link
                :to="{ name: 'post', params: { slug: single_menu.blog.slug } }"
                class="nav-link"
              >
                {{ single_menu.name }}
              </router-link>
            </vs-sidebar-item>
          </template>

          <vs-sidebar-item
            v-for="child in single_menu.children || []"
            :key="child.id"
          >
            <router-link
              :to="{ name: 'post', params: { slug: child.blog.slug } }"
              class="nav-link"
              >{{ child.name }}
            </router-link>
          </vs-sidebar-item>
        </vs-sidebar-group>
        <template #footer>
          <vs-row justify="space-between"></vs-row>
        </template>
      </vs-sidebar>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import { listMenu } from "@/services/menu";
import { getWebHost } from "@/utils/api";
import Component from "vue-class-component";
import { Menu } from "@/models/menu";
import { Watch } from "vue-property-decorator";

const fetchMenus = async () => {
  return await listMenu();
};

@Component
export default class NavBar extends Vue {
  menus: Menu[] = [];
  activeSidebar = false;
  activeSlug = "";

  get title() {
    return this.$store.getters["meta/getState"]("title", getWebHost());
  }

  async mounted() {
    this.menus = await fetchMenus();
  }

  @Watch("$route")
  onSlugChanged(value: string, oldValue: string) {
    this.activeSidebar = false;
  }
}
</script>

<style scoped>
.title {
  color: black;
  text-decoration: none;
  text-align: left;
}

.nav-link {
  text-decoration: none;
  color: inherit;
}

.visible-on-mobile {
  display: none;
}

@media (min-width: 900px) {
  .center-menu {
    margin-right: 64px;
  }
}

.hidden-on-mobile {
  display: inline-flex;
}

@media (max-width: 900px) {
  .visible-on-mobile {
    display: inline-flex;
  }

  .hidden-on-mobile {
    display: none;
  }
}

.sub-menu {
  margin: 8px;
  min-width: 10rem;
}
.right-template {
  flex-wrap: wrap;
}
</style>
