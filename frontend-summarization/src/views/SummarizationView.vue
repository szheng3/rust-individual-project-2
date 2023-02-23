<script setup lang="ts">
import {useQuery} from "@tanstack/vue-query";
import {useSummarization} from "../stores/useSummatization";

const store = useSummarization();

const {isLoading} = useQuery(
    ["summaries"],
    store.fetchHealth);
</script>

<template>
  <div class="hello">
    <h1>{{store.health}}</h1>

  </div>
</template>


