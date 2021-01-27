<template>
  <div id="app">
    <NavBar />
    <router-view class="router-slot" />
    <Footer />
  </div>
</template>

<style lang="scss">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin: 0 -8px;
}

.router-slot {
  margin-top: 5%;
}

#nav {
  padding: 30px;

  a {
    font-weight: bold;
    color: #2c3e50;

    &.router-link-exact-active {
      color: #42b983;
    }
  }
}
</style>
<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import NavBar from "@/components/NavBar.vue"; // @ is an alias to /src
import Footer from "@/components/Footer.vue";
import { config } from "@/config";

@Component({
  components: {
    NavBar,
    Footer
  },
  mounted() {
    this.$store.dispatch("meta/fetchMetas");
  },
  watch: {
    $route(to, from) {
      document.title = to.meta.title || config.websiteTitle;
    }
  }
})
export default class App extends Vue {}
</script>
