<template>
  <v-app>
    <v-container>
      <v-row>
        <v-col>
          <v-form @submit.prevent="submitForm">


            <v-textarea
                rounded
                v-model="textInput"
                label="Enter text to summarize"
                rows="50"
                class="rounded"
                :loading="loading"></v-textarea>
            <div class="text-caption">
              Longer text will take longer to process. Output words Around: {{ Math.round(sliderValue / 100 * textInput.split(' ').length) }}
            </div>
            <v-slider v-model="sliderValue" min="0" max="100"
                      :thumb-size="15"

                      thumb-label=true></v-slider>


            <v-btn :loading="loading" type="submit" color="primary">Submit</v-btn>
          </v-form>
        </v-col>
        <v-col>
          <v-card v-if="showResult" class="mt-4" variant="tonal">
            <v-card-title class="headline">{{ result.status }}</v-card-title>
            <v-card-text>{{ result.message }}</v-card-text>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
  </v-app>
</template>

<script>
import {ref} from 'vue';
import axios from "axios";

export default {
  setup() {
    const textInput = ref('');
    const loading = ref(false);
    const showResult = ref(false);
    const sliderValue = ref(50);
    const result = ref({status: '', message: ''});

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

    return {textInput, loading, showResult, result, submitForm, sliderValue};
  },
};
</script>