import {ref} from "vue";
import {defineStore} from "pinia";

export const useSummarization = defineStore("summarization", () => {
    const health = ref({});

    const fetchSummarization = async () => {
        const re = await fetch("/api/health").then((response) => response.json());
        health.value = re;
        return re;
    };

    const fetchHealth = async () => {
        const re = await fetch("/api/health").then((response) => response.json());
        health.value = re;
        return re;

    };

    return {
        health,
        fetchSummarization,
        fetchHealth
    };
});