<template>
  <vs-row
    class="contact-content"
    type="flex"
    justify="center"
    :style="{
      'background-color': content.background,
      color: content.text_color
    }"
  >
    <vs-col type="flex" justify="left" lg="4" sm="12" xs="12">
      <vs-row>
        <vs-col justify="center" w="12">
          <h2>{{ content.title }}</h2>
        </vs-col>
        <vs-col justify="center" w="12" type="flex">
          <vs-row class="box-shadow" type="flex">
            <vs-col
              justify="center"
              w="12"
              v-for="(value, key) in content.fields"
              v-bind:key="key"
              class="field"
            >
              <vs-row
                justify="center"
                type="flex"
                class="field-internal"
                v-if="value.input_type === 'switch'"
              >
                <vs-col w="6" class="label" type="flex" justify="left">
                  {{ value.label }}
                </vs-col>
                <vs-col w="6" type="flex" justify="flex-end">
                  <vs-switch
                    class="switch"
                    :value="entries[key] || false"
                    @input="entry => updateEntry(key, entry)"
                  />
                </vs-col>
              </vs-row>
              <vs-row
                justify="left"
                type="flex"
                class="field-internal"
                v-else-if="value.input_type === 'radio'"
              >
                <vs-col w="4" justify="left" type="flex">
                  {{ value.label }}
                </vs-col>
                <vs-col w="8" justify="flex-end" type="flex">
                  <vs-row justify="flex-end" type="flex">
                    <vs-col
                      justify="center"
                      w="3"
                      v-for="option in value.options"
                      :key="option"
                      class="radio-box"
                    >
                      <vs-radio
                        @input="updateEntry(key, option)"
                        :val="option"
                        :value="entries[key]"
                      >
                        {{ option }}
                      </vs-radio>
                    </vs-col>
                  </vs-row>
                </vs-col>
              </vs-row>
              <vs-row
                justify="left"
                type="flex"
                class="field-internal"
                v-else-if="value.input_type === 'textarea'"
              >
                <vs-col w="4" justify="left" type="flex">
                  {{ value.label }}
                </vs-col>
                <vs-col w="8" justify="flex-end" type="flex">
                  <textarea
                    :placeholder="value.label"
                    class="textarea"
                    :value="entries[key] || ''"
                    @input="event => updateEntry(key, event.target.value)"
                  >
                  </textarea>
                </vs-col>
              </vs-row>
              <vs-row
                justify="center"
                type="flex"
                class="field-internal text-input"
                v-else
              >
                <vs-col w="4" class="label" type="flex" justify="left">
                  {{ value.label }}
                </vs-col>
                <vs-col w="8" type="flex" justify="flex-end">
                  <vs-input
                    :placeholder="value.label"
                    :type="value.input_type"
                    :value="entries[key] || ''"
                    @input="entry => updateEntry(key, entry)"
                  />
                </vs-col>
              </vs-row>
            </vs-col>
            <vs-col justify="flex-end" type="flex" w="12" class="submit-box">
              <vs-button active @click="submitEntry">
                Submit
              </vs-button>
            </vs-col>
          </vs-row>
        </vs-col>
      </vs-row>
    </vs-col>
  </vs-row>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import { sendContact } from "@/services/contact";

const ContactProps = Vue.extend({
  props: {
    content: {
      type: Object,
      default: () => ({})
    }
  }
});

@Component
export default class ContactContent extends ContactProps {
  entries: object = {};

  updateEntry(key: string, value: string) {
    Vue.set(this.entries, key, value);
  }

  async submitEntry() {
    const sent = await sendContact(this.content, this.entries);
    if (sent) {
      this.$vs.notification({
        color: "primary",
        position: null,
        title: "Form Submitted.",
        text:
          "Your form is successfully submitted. We will get back to you soon."
      });
      this.entries = {};
    } else {
      this.$vs.notification({
        color: "warning",
        position: null,
        title: "Form Cannot be submitted.",
        text: "Your form cannot be submitted. Please check the values."
      });
    }
  }
}
</script>

<style scoped>
.field-internal {
  margin-bottom: 16px;
  margin-top: 16px;
}

.box-shadow {
  box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2);
  padding: 16px;
  margin: 16px;
}

.radio-box {
  margin-right: 16px;
}

.textarea {
  width: 80%;
  min-height: 128px;
  border: 2px solid transparent;
  background: rgba(var(--vs-gray-2), 1);
  color: rgba(var(--vs-text), 1);
  padding: 7px 13px;
  border-radius: inherit;
  -webkit-transition: all 0.25s ease;
  transition: all 0.25s ease;
}

.submit-box {
  padding-top: 16px;
  margin-top: 32px;
}
</style>
