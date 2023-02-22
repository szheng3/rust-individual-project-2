<!--<template>-->
<!--  <div class="hello">-->
<!--    <h1>{{ msg }}</h1>-->
<!--    <h1>Made By Getters</h1>-->
<!--    <div v-for="post in store.posts" :key="post.id">-->
<!--      {{ post.id }} {{ post.title }} {{ post.body }}-->
<!--    </div>-->
<!--  </div>-->
<!--</template>-->

<!--&lt;!&ndash;<script setup>&ndash;&gt;-->
<!--&lt;!&ndash;import { ref, onMounted, computed } from "vue";&ndash;&gt;-->
<!--&lt;!&ndash;import {useUserStore} from "@/stores/users";&ndash;&gt;-->
<!--&lt;!&ndash;const store = useUserStore();&ndash;&gt;-->
<!--&lt;!&ndash;const msg = ref("Welcome to Page");&ndash;&gt;-->
<!--&lt;!&ndash;const getUsers = computed(() => {&ndash;&gt;-->
<!--&lt;!&ndash;  return store.getUsers;&ndash;&gt;-->
<!--&lt;!&ndash;});&ndash;&gt;-->
<!--&lt;!&ndash;const users = computed(() => {&ndash;&gt;-->
<!--&lt;!&ndash;  return store.users;&ndash;&gt;-->
<!--&lt;!&ndash;});&ndash;&gt;-->
<!--&lt;!&ndash;onMounted(() => {&ndash;&gt;-->
<!--&lt;!&ndash;  store.fetchUsers();&ndash;&gt;-->
<!--&lt;!&ndash;});&ndash;&gt;-->
<!--&lt;!&ndash;</script>&ndash;&gt;-->
<!--<script setup>-->
<!--import {usePostsV3} from "@/stores/usePostsV3";-->
<!--import {useQuery} from "@tanstack/vue-query";-->

<!--const store = usePostsV3();-->

<!--const {isLoading} = useQuery(-->
<!--    ["posts3"],-->
<!--    store.fetchPosts);-->
<!--</script>-->