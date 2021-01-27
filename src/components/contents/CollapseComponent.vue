<template>
  <vs-row class="collapse-content" type="flex" justify="center">
    <vs-col type="flex" justify="center" lg="6" sm="12" xs="12">
      <span class="collapsible-item">
        <vs-row v-for="(value, key) in content.items" v-bind:key="key">
          <vs-col class="column-margin" w="12">
            <details>
              <summary
                class="header-slot"
                :style="{
                  'background-color': content.background,
                  color: content.text_color
                }"
              >
                <span class="expanded-content header-collapse">
                  {{ key }}
                </span>
              </summary>
              <div class="content-collapse" v-html="value"></div>
            </details>
          </vs-col>
        </vs-row>
      </span>
    </vs-col>
  </vs-row>
</template>

<script>
export default {
  name: "CollapseComponent",
  props: ["content"],
  components: {}
};
</script>

<style scoped>
.collapse-content {
  padding: 16px;
}

.header-slot {
  padding: 16px;
  margin: -8px -16px -8px -8px;
}

.header-collapse {
  font-weight: bolder;
  font-size: x-large;
}

.content-collapse {
  font-weight: bold;
  font-size: larger;
  margin: 16px 0;
  padding-top: 16px;
}

.collapsible-item {
  text-align: left;
  width: 100%;
  margin-top: 16px;
  margin-bottom: 16px;
}
.column-margin {
  margin-bottom: 16px;
  margin-top: 16px;
}
</style>
