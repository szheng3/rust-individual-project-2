import {defineStore} from 'pinia'
// Import axios to make HTTP requests
import axios from "axios"

export const useUserStore = defineStore("user", {
    state: () => ({
        users: [],
    }),
    getters: {
        getUsers(state){
            return state.users
        }
    },
    actions: {
        async fetchUsers() {
            try {
                const data = await axios.get('https://jsonplaceholder.typicode.com/users')
                this.users = data.data
            }
            catch (error) {
                alert(error)
                console.log(error)
            }
        }
    },
})