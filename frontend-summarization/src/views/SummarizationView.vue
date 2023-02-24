<template>
  <v-container>
    <v-row>
      <v-col  >
        <v-form @submit.prevent="submitForm">
          <v-textarea
              variant="outlined"
              v-model="textInput"
              label="Enter text to summarize"
              rows="50"
              class="rounded"
              :loading="loading"></v-textarea>
          <v-btn :loading="loading" type="submit" color="primary">Submit</v-btn>
        </v-form>
      </v-col>
      <v-col  >
        <v-card v-if="showResult" class="mt-4">
          <v-card-title class="headline">{{ result.status }}</v-card-title>
          <v-card-text>{{ result.message }}</v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
import {ref} from 'vue';
import axios from "axios";

export default {
  setup() {
    const textInput = ref('');
    const loading = ref(false);
    const showResult = ref(false);
    const result = ref({status: '', message: ''});

    const submitForm = async () => {
      loading.value = true;
      console.log(textInput.value);
      try {

        const response = await axios.post('/api/summary', {context: textInput.value});
        result.value = await response.data;
        showResult.value = true;




      } catch (error) {
        console.error(error);
      } finally {
        loading.value = false;
      }

    };

    return {textInput, loading, showResult, result, submitForm};
  },
};
</script>