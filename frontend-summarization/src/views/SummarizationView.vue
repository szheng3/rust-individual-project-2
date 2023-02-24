<template>
  <v-app>
    <v-app-bar flat>
      <v-container class="fill-height d-flex align-center">
<!--        <v-avatar-->
<!--            class="me-10 ms-4"-->
<!--            color="grey-darken-1"-->
<!--            size="32"-->
<!--        ></v-avatar>-->

        <v-btn
            v-for="link in links"
            :key="link"
            variant="text"
        >
          {{ link }}
        </v-btn>

        <v-spacer></v-spacer>

      </v-container>
    </v-app-bar>
    <v-main class="bg-grey-lighten-4">

      <v-container>
        <v-row>
          <v-col cols="12"
                 sm="6">
            <v-sheet rounded class="mt-4">

              <v-form @submit.prevent="submitForm" class="pa-3">


                <v-textarea
                    rounded
                    v-model="textInput"
                    label="Enter text to summarize"
                    rows="23"
                    :loading="loading"></v-textarea>

                <div class="text-caption">
                  Longer text will take longer to process. Output words about:
                  <strong>{{ Math.round(sliderValue / 100 * textInput.split(' ').length) }}</strong>
                </div>
                <v-slider v-model="sliderValue" min="0" max="100"
                          :thumb-size="15"

                          thumb-label=true></v-slider>


                <v-btn :loading="loading" type="submit" color="primary">Submit</v-btn>
              </v-form>
            </v-sheet>

          </v-col>

          <v-col cols="12"
                 sm="6">

            <v-card v-if="showResult" variant="flat" class="mt-4">
              <v-card-title class="headline">{{ result.status }}</v-card-title>
              <v-card-text>{{ result.message }}</v-card-text>
            </v-card>
          </v-col>
        </v-row>


      </v-container>
    </v-main>
    <v-footer
        class=" text-center d-flex flex-column"
        flat
    >

      <div class="text-caption">
        This project aims to build a Rust CLI tool that summarizes text, based on the common task of reading and
        summarizing books among students.
      </div>


      <div class="text-caption">
        {{ new Date().getFullYear() }} —Shuai Zheng
      </div>
    </v-footer>
  </v-app>
</template>


<script setup>
import {ref} from 'vue';
import axios from "axios";


const textInput = ref('');
const loading = ref(false);
const showResult = ref(false);
const sliderValue = ref(50);
const result = ref({status: 'success', message: 'Sample'});
const links = ref([
  'Summarization',
]);

const submitForm = async () => {
  loading.value = true;
  try {

    const response = await axios.post('/api/summary', {
      context: textInput.value,
      minlength: Math.round(sliderValue.value / 100 * textInput.value.split(' ').length)
    });
    result.value = await response.data;
    showResult.value = true;


  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }


};
</script>