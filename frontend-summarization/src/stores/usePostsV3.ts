import {ref} from "vue";
import {defineStore} from "pinia";

export const usePostsV3 = defineStore("posts", () => {
    const posts = ref([]);
    const post = ref([]);

    const fetchPosts = async () => {
        const re = await fetch("https://jsonplaceholder.typicode.com/posts").then((response) => response.json());
        posts.value = re;
        return re;

    };
    // @ts-ignore
    const fetchPost = async ({ queryKey }) => {
        const re =await fetch(`https://jsonplaceholder.typicode.com/posts/${queryKey[1]}`).then((response) => response.json());
        post.value = re;
        return re;
    };

    return {
        posts,
        post,
        fetchPosts,
        fetchPost
    };
});