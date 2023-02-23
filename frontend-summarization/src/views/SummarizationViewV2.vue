<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
    <h1>Made By Getters</h1>
    <div v-for="post in store.posts" :key="post.id">
      {{ post.id }} {{ post.title }} {{ post.body }}
    </div>
  </div>
</template>

<!--<script setup>-->
<!--import { ref, onMounted, computed } from "vue";-->
<!--import {useUserStore} from "@/stores/users";-->
<!--const store = useUserStore();-->
<!--const msg = ref("Welcome to Page");-->
<!--const getUsers = computed(() => {-->
<!--  return store.getUsers;-->
<!--});-->
<!--const users = computed(() => {-->
<!--  return store.users;-->
<!--});-->
<!--onMounted(() => {-->
<!--  store.fetchUsers();-->
<!--});-->
<!--</script>-->
<script setup>
import {usePostsV3} from "@/stores/usePostsV3";
import {useQuery} from "@tanstack/vue-query";

const store = usePostsV3();

const {isLoading} = useQuery(
    ["posts3"],
    store.fetchPosts);
</script>