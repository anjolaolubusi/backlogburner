import axios from 'axios';

export const DatabaseAPI = axios.create({
    baseURL: 'http://localhost:3000/api/'
})